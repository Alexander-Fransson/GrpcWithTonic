#[cfg(test)]
mod tests;

const DEFAULT_USER_CONNECTION_STRING: &str = "postgres://madeup_default_user:super_secret_password@localhost:5011";

use crate::{Error, Result};

