use std::{io::stdout, fmt::Display};

use crossterm::{style::{StyledContent, Stylize}, tty::IsTty};
use self::RColor::*;
pub enum RColor<'a> {
    Colorless(&'a str),
    Colored(StyledContent<&'a str>)
}

impl Display for RColor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Colorless(str) => write!(f, "{str}"),
            Self::Colored(content) => write!(f, "{content}")
        }
    }
}

pub fn green(str: &str) -> RColor {
    if stdout().is_tty() {
        Colored(str.green())
    } else {
        Colorless(str)
    }
}

pub fn blue(str: &str) -> RColor {
    if stdout().is_tty() {
        Colored(str.blue())
    } else {
        Colorless(str)
    }
}

pub fn dark_green(str: &str) -> RColor {
    if stdout().is_tty() {
        Colored(str.dark_green())
    } else {
        Colorless(str)
    }
}

pub fn bold(str: &str) -> RColor {
    if stdout().is_tty() {
        Colored(str.bold())
    } else {
        Colorless(str)
    }
}