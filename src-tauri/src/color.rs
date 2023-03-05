use std::{io::stdout, fmt::Display};

use crossterm::{style::{StyledContent, Stylize}, tty::IsTty};

pub trait TtyColor: Stylize {
    fn tty_red(self) -> <Self as Stylize>::Styled {
        self.red()
    }
    fn tty_green(self) -> <Self as Stylize>::Styled {
        self.green()
    }
    fn tty_blue(self) -> <Self as Stylize>::Styled {
        self.blue()
    }
    fn tty_dark_green(self) -> <Self as Stylize>::Styled {
        self.dark_green()
    }
    fn tty_bold(self) -> <Self as Stylize>::Styled {
        self.bold()
    }
}

impl TtyColor for String {}
impl TtyColor for &str {}
impl TtyColor for char {}
impl<T: std::fmt::Display> TtyColor for StyledContent<T> {}