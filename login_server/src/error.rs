
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {

    // data access
    FailedToReadFiles(String),
    QueryFailed(sqlx::Error),
    MigrationFailed(sqlx::migrate::MigrateError),
    EntityNotFound,

    // crypt
    FailedToCreateSaltString(String),
    FailedToHashPassword(String),
    PasswordInvalid,
    FailedToCreateMacKey(String),
    JwtTokenWrongFormat,
    InvalidJwtTokenSignature,
    JwtTokenExpired,

    //utils
    FailedToDecodeB64(String),
    FailedToDecodeB64Bytes(String),
    FailedToFormtOffsetDateTime(String),
    FailedToParseOffsetDateTime(String),

    // env
    FailedToGetEnvVar(String),

    // general
    FailedToParse(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(fmt, "{self:?}")    
    }
}

impl std::error::Error for Error {}