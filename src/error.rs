#[derive(Debug)]
pub enum ProgramErr {
    IoErr(std::io::Error),
    ClapErr(clap::Error),
    SerdeJsonErr(serde_json::Error),
    ExitErr(String),
    FirstRunPromptCancelErr(String),
}

impl From<std::io::Error> for ProgramErr {
    fn from(e: std::io::Error) -> Self {
        Self::IoErr(e)
    }
}

impl From<clap::Error> for ProgramErr {
    fn from(e: clap::Error) -> Self {
        Self::ClapErr(e)
    }
}

impl From<serde_json::Error> for ProgramErr {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonErr(e)
    }
}
