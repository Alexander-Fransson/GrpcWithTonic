use crate::{utils::base64::b64_to_u8, Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

// statically load env variables as to not have to construct them every time you need them
pub fn get_env_static() -> &'static EnvVariables {
    static INSTANCE: OnceLock<EnvVariables> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        EnvVariables::get()
        .unwrap_or_else(|ex| {
            panic!("Failed to load EnvVariableContainer: {}", ex)
        })
    })
}

#[allow(non_snake_case)]
pub struct EnvVariables {
    pub DB_CONNECTION_STRING: String,
    pub DB_DEFAULT_USER_CONNECTION_STRING: String,

    pub JWT_KEY: Vec<u8>,
    pub JWT_DURRATION_SEC: f64
}

impl EnvVariables {
    pub fn get() -> Result<Self> {
        Ok(Self {
            DB_CONNECTION_STRING: get_var_from_env_parsed("DB_CONNECTION_STRING")?,
            DB_DEFAULT_USER_CONNECTION_STRING: get_var_from_env_parsed("DB_DEFAULT_USER_CONNECTION_STRING")?,

            JWT_KEY: get_env_b64_var_as_u8("JWT_KEY")?,
            JWT_DURRATION_SEC: get_var_from_env_parsed("JWT_DURRATION_SEC")?,
        })
    }
}

fn get_var_from_env(name: &'static str) -> Result<String> {
    env::var(name)
    .map_err(|e| Error::FailedToGetEnvVar(e.to_string()))
}

fn get_var_from_env_parsed<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_var_from_env(name)?;
    val.parse::<T>().map_err(|_| Error::FailedToParse("env var".to_string()))
}

fn get_env_b64_var_as_u8(name: &'static str) -> Result<Vec<u8>> {
    let val = get_var_from_env(name)?;
    let bytes = b64_to_u8(&val)?;
    Ok(bytes)
}