#[derive(Debug)]
pub enum ProgramErr {
    Io(std::io::Error),
    Clap(clap::Error),
    SerdeJson(serde_json::Error),
    ExitErr(String),
    FirstRunPromptCancelErr(String),
}

impl From<std::io::Error> for ProgramErr {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<clap::Error> for ProgramErr {
    fn from(e: clap::Error) -> Self {
        Self::Clap(e)
    }
}

impl From<serde_json::Error> for ProgramErr {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}
