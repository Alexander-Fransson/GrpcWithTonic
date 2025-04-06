
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    
    // env
    FailedToGetEnvVar(String),

    // general
    FailedToParse(&'static str),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(fmt, "{self:?}")    
    }
}

impl std::error::Error for Error {}