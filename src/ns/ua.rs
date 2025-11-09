use log::error;
use std::{env, process::exit};
use crate::ns::format::canonicalize_name;

#[derive(Clone)]
pub struct UserAgent {
    program: &'static str,
    version: &'static str,
    author: &'static str,
    user: String,
}

impl UserAgent {
    pub fn read_from_env(
        program: &'static str,
        version: &'static str, 
        author: &'static str
    ) -> Self {
        let user = match env::var("NS_USER_AGENT") {
            Ok(user) => user,
            Err(err) => match err {
                env::VarError::NotPresent => {
                    error!("No user agent provided, please set the NS_USER_AGENT environment variable to your main nation name");
                    exit(1);
                },
                env::VarError::NotUnicode(_) => {
                    error!("User agent is not valid unicode");
                    exit(1);
                }
            }
        };

        Self { program, version, author, user: canonicalize_name(&user) }
    }

    pub fn api(&self) -> String {
        format!("{}/{} by {}, in use by {}", self.program, self.version, self.author, self.user)
    }

    pub fn web(&self) -> String {
        format!("{}__{}__by_{}__usedBy_{}", self.program, self.version, self.author, self.user)
    }
}