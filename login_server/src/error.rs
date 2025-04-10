use tonic::Status;

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

    // main
    FailedToStartGrpcServer(String),
}

// To turn the error into a tonic status
impl From<Error> for tonic::Status {
    fn from(error: Error) -> Self {
        match error {
          Error::PasswordInvalid => Status::permission_denied("password invalid"),
          Error::JwtTokenWrongFormat |
          Error::InvalidJwtTokenSignature |
          Error::JwtTokenExpired => Status::unauthenticated("jwt token invalid"),
          Error::EntityNotFound => Status::not_found("entity not found"),
          _ => Status ::internal("internal server error"),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(fmt, "{self:?}")    
    }
}

impl std::error::Error for Error {}