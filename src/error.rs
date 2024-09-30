#[allow(unused_imports)]
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;


/// Groups up the kinds of errors that may happen in this crate.
#[derive(Error, Diagnostic, Debug)]
#[error("ralertsinua error!")]
// coveralls-ignore-next-line
pub enum AppError {
    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
    // #[error("tokio send error: {0}")]
    // TokioSend(#[from] tokio::sync::mpsc::error::SendError<Action>),
    // #[error("tokio channel error: {0}")]
    // TokioChannel(#[from] tokio::sync::mpsc::error::TrySendError<Action>),
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("component error")]
    #[diagnostic(code(ralertsinua::component))]
    ComponentError,
    #[error("unknown error")]
    Unknown,
}
