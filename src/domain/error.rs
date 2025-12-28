use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("unable to execute command {cmd}: {source}")]
    Spawn {
        cmd: &'static str,
        #[source]
        source: std::io::Error,
    },

    #[error("the command {cmd} ended with code {code}: {stderr}")]
    NonZeroExit {
        cmd: &'static str,
        code: i32,
        stderr: String,
    },

    #[error("unable to parse JSON from {cmd}: {source}")]
    Json {
        cmd: &'static str,
        #[source]
        source: serde_json::Error,
    },
}
