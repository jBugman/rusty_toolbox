#[macro_use]
extern crate failure;

use std::fmt::Display;
use std::path::Path;
use std::result::Result as StdResult;

pub use failure::{Error, Fail, ResultExt};

pub type Result<T> = StdResult<T, Error>;

fn wrap_error<E, D>(err: E, msg: &str, data: D) -> Error
where
    E: Fail,
    D: Display,
{
    Error::from(err.context(format_err!("{} ({})", msg, data)))
}

pub trait FmtResultExt<T, D> {
    fn context_fmt(self, msg: &str, data: D) -> Result<T>;
}

impl<T, D: Display, E: Fail> FmtResultExt<T, D> for StdResult<T, E> {
    fn context_fmt(self, msg: &str, data: D) -> Result<T> {
        self.map_err(|err| wrap_error(err, msg, data))
    }
}

pub trait PathResultExt<T> {
    fn context_path(self, msg: &str, path: &Path) -> Result<T>;
}

impl<T, E: Fail> PathResultExt<T> for StdResult<T, E> {
    fn context_path(self, msg: &str, path: &Path) -> Result<T> {
        self.map_err(|err| wrap_error(err, msg, path.display()))
    }
}

pub trait OptionFailExt<T> {
    fn or_fail(self, msg: &str) -> Result<T>;
}

impl<T> OptionFailExt<T> for Option<T> {
    fn or_fail(self, msg: &str) -> Result<T> {
        self.ok_or_else(|| format_err!("{}", msg))
    }
}

pub trait UnwrapOrExit<T> {
    fn unwrap_or_exit(self) -> T;
}

impl<T> UnwrapOrExit<T> for Result<T> {
    fn unwrap_or_exit(self) -> T {
        match self {
            Err(err) => {
                let mut causes = err.causes();
                fmt_error(causes.next().unwrap());
                for c in causes {
                    eprintln!(" caused by: {}", c);
                }
                std::process::exit(1);
            }
            Ok(v) => v,
        }
    }
}

#[cfg(not(feature = "color"))]
fn fmt_error(err: &Fail) {
    eprintln!("error: {}", err);
}

#[cfg(feature = "color")]
extern crate yansi;

#[cfg(feature = "color")]
fn fmt_error(err: &Fail) {
    eprintln!("{} {}", yansi::Paint::red("error:"), err);
}
