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
    fn context_path(self, msg: &str, path: impl AsRef<Path>) -> Result<T>;
}

impl<T, E: Fail> PathResultExt<T> for StdResult<T, E> {
    fn context_path(self, msg: &str, path: impl AsRef<Path>) -> Result<T> {
        self.map_err(|err| wrap_error(err, msg, path.as_ref().display()))
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

#[deprecated(since = "0.5.0", note = "use log_errors function")]
pub trait UnwrapOrExit<T> {
    fn unwrap_or_exit(self) -> T;
}

#[allow(deprecated)]
#[deprecated(since = "0.5.0", note = "use log_errors function")]
impl<T> UnwrapOrExit<T> for Result<T> {
    fn unwrap_or_exit(self) -> T {
        self.unwrap_or_else(|err| log_errors(Err(err)))
    }
}

pub fn log_errors(r: Result<()>) -> ! {
    ::std::process::exit(match r {
        Err(err) => {
            print_causes(err);
            1
        }
        Ok(_) => 0,
    });
}

fn print_causes(err: Error) {
    let mut causes = err.causes();
    fmt_error(causes.next().unwrap());
    for c in causes {
        eprintln!(" caused by: {}", c);
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

pub trait ContextAsErrorExt<T> {
    fn context_err<D>(self, context: D) -> Result<T>
    where
        D: Display + Send + Sync + 'static;
}

impl<T, E: Fail> ContextAsErrorExt<T> for StdResult<T, E> {
    fn context_err<D>(self, c: D) -> Result<T>
    where
        D: Display + Send + Sync + 'static,
    {
        self.context(c).map_err(Error::from)
    }
}
