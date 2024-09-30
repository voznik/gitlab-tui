mod api;
mod app;
mod args;
mod error;
mod config;
mod handlers;
mod ui;

use clap::Parser;
use gitlab::api::{
    projects::{issues::Issues,  merge_requests::MergeRequests, Project},
    AsyncQuery,
};
use std::{io, sync::Arc};
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration};

use app::App;
use config::Config;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let args = args::Args::parse();
    let mut config = Config::default();

    if config.token().is_empty() {
        let token = api::get_token(config.base_url().to_string()).unwrap();
        config.set_token(token);
    }

    if config.project().is_empty() {
        let (domain, project) = api::get_gitlab_remote(&args.remote).ok().unwrap(); // todo change
        config.set_project(project);
    }

    let mut app = App::new(config);
    app.active_git_remote = args.remote;
    let app = Arc::new(Mutex::new(app));

    // create app and run it
    run(app)
}

#[tokio::main()]
async fn run(app: Arc<Mutex<App>>) -> Result<(), Box<dyn std::error::Error>> {
    let fetch_app = app.clone();
    task::spawn(async {
        if let Some(e) = run_fetch(fetch_app).await {
            panic!("Failed to fetch data from gitlab: {}", e)
        }
    });

    if let Err(e) = run_ui(&app.clone()).await {
        panic!("Failed to render UI: {}", e)
    }
    Ok(())
}

async fn run_ui(app: &Arc<Mutex<App>>) -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| ui::main_ui::draw_main_layout(f, &app))?;
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            match event::read()? {
                Event::Key(key) => match key {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => break,
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => break,
                    e => handlers::handle_input(e, &mut app),
                },
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_fetch(app: Arc<Mutex<App>>) -> Option<Box<dyn std::error::Error>> {
    // have queue on app
    // continously pull work items out of queue and run requests and update corresponding state
    // in separate thread, every second push update request
    let mut remote: String;
    let mut config: Config;
    loop {
        {
            // only hold the lock for getting the active remote
            // release the lock after, so that fetching won't block the render loop
            let app = app.clone();
            remote = app.lock().await.active_git_remote.clone();
            config = app.lock().await.config.clone();
        }
        let api = {
            let this = gitlab::Gitlab::builder(config.base_url(), config.token()).build_async().await;
            match this {
                Ok(t) => t,
                Err(e) => panic!("Failed to connect to gitlab: {}", e),
            }
        };
        let issues_query = Issues::builder().project(config.project()).build().ok()?;
        let project_query = Project::builder().project(config.project()).build().ok()?;
        let mr_query = MergeRequests::builder()
            .project(config.project())
            .build()
            .ok()?;

        let (issues, mrs, project) = tokio::join!(
            issues_query.query_async(&api),
            mr_query.query_async(&api),
            project_query.query_async(&api),
        );
        {
            let mut app = app.lock().await;
            // dbg!(i.clone());
            app.issues = issues.ok()?;
            app.mrs = mrs.ok()?;
            app.project = project.ok()?;
        }
        sleep(Duration::from_secs(1)).await;
    }
}
