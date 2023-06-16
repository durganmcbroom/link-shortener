use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum AppErr {
    AliasAlreadyUsed(String, String),
    AliasNotFound(String)
}

impl Display for AppErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppErr::AliasAlreadyUsed(path, alias) => write!(f, "Cannot shorten link: '{}', alias: '{}' already in use.", path, alias)
        }
    }
}

impl Debug for AppErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for AppErr {}