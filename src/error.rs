#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    HttpError(#[from] reqwest::Error),
}
