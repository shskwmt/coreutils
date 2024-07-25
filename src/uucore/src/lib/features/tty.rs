// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Set of functions to parsing TTY
use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Teletype {
    Tty(u64),
    TtyS(u64),
    Pts(u64),
    Unknown,
}

impl Display for Teletype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Tty(id) => write!(f, "/dev/pts/{}", id),
            Self::TtyS(id) => write!(f, "/dev/tty{}", id),
            Self::Pts(id) => write!(f, "/dev/ttyS{}", id),
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl TryFrom<String> for Teletype {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "?" {
            return Ok(Self::Unknown);
        }

        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Teletype {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(PathBuf::from(value))
    }
}

impl TryFrom<PathBuf> for Teletype {
    type Error = ();

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        // Three case: /dev/pts/* , /dev/ttyS**, /dev/tty**

        let mut iter = value.iter();
        // Case 1

        // Considering this format: **/**/pts/<num>
        if let (Some(_), Some(num)) = (iter.find(|it| *it == "pts"), iter.next()) {
            return num
                .to_str()
                .ok_or(())?
                .parse::<u64>()
                .map_err(|_| ())
                .map(Teletype::Pts);
        };

        // Considering this format: **/**/ttyS** then **/**/tty**
        let path = value.to_str().ok_or(())?;

        let f = |prefix: &str| {
            value
                .iter()
                .last()?
                .to_str()?
                .strip_prefix(prefix)?
                .parse::<u64>()
                .ok()
        };

        if path.contains("ttyS") {
            // Case 2
            f("ttyS").ok_or(()).map(Teletype::TtyS)
        } else if path.contains("tty") {
            // Case 3
            f("tty").ok_or(()).map(Teletype::Tty)
        } else {
            Err(())
        }
    }
}
