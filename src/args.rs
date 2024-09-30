use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Set this to your remote's name, if you're using a different name than 'origin'
    #[arg(short, long, default_value_t = String::from("origin"))]
    pub remote: String,
    #[arg(
        short,
        long,
        value_name = "GITLAB_TOKEN",
        help = "https://gitlab.com/-/user_settings/personal_access_tokens",
        default_value = "",
        required = false
    )]
    pub token: String,
    #[arg(
        short,
        long,
        value_name = "GITLAB_PROJECT",
        help = "Project Name / Id",
        default_value = "",
        required = false
    )]
    pub project: String,
}
