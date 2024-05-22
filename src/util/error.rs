use core::fmt::{Debug, Display};

use super::errno::Errno;

pub type KResult<T> = Result<T, KError>;

#[derive(Clone)]
pub struct KError {
    pub message: Option<&'static str>,
    pub errno: Option<Errno>,
}

impl KError {
    pub fn new(message: Option<&'static str>, errno: Option<Errno>) -> Self {
        Self { message, errno }
    }

    pub fn from_errno(errno: Errno) -> Self {
        Self {
            message: None,
            errno: Some(errno),
        }
    }

    pub fn from_message(message: &'static str) -> Self {
        Self {
            message: Some(message),
            errno: None,
        }
    }

    pub fn message(&self) -> Option<&'static str> {
        self.message
    }

    pub fn errno(&self) -> Option<Errno> {
        self.errno
    }
}

impl Debug for KError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(message) = self.message {
            write!(f, "{}", message)?;
        }

        if let Some(errno) = self.errno {
            write!(f, " ({:?})", errno)?;
        }

        Ok(())
    }
}

impl Display for KError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&'static str> for KError {
    fn from(message: &'static str) -> Self {
        Self::from_message(message)
    }
}

impl From<Errno> for KError {
    fn from(errno: Errno) -> Self {
        Self::from_errno(errno)
    }
}

#[macro_export]
macro_rules! kernel_error {
    ($message:literal, $errno:expr) => {
        $crate::util::error::KernelError::new($message, $errno)
    };
    ($message:literal) => {
        $crate::util::error::KernelError::from_message($message)
    };
    ($errno:expr) => {
        $crate::util::error::KernelError::from_errno($errno)
    };
    () => {
        $crate::util::error::KernelError::new(None, None)
    };
}
