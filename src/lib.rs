#[cfg(feature = "convert")]
mod convert;
mod detect;

pub use detect::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorSupport {
    None,
    Ansi16,
    Ansi256,
    TrueColor,
}
