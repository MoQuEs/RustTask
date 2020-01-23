use std::{error, fmt, io, num, str};

pub struct AppError {
    kind: String,
    message: String,
}

impl AppError {
    #[allow(dead_code)]
    pub fn new(kind: &str, message: &str) -> Self {
        Self {
            kind: String::from(kind),
            message: String::from(message),
        }
    }

    pub fn new_res<T>(kind: &str, message: &str) -> Result<T, Self> {
        Err(Self {
            kind: String::from(kind),
            message: String::from(message),
        })
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", &self.message)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error type: {}, message: {} (file: {}, line: {})",
            &self.kind,
            &self.message,
            file!(),
            line!()
        )
    }
}

impl error::Error for AppError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        Self {
            kind: String::from("parse int"),
            message: error.to_string(),
        }
    }
}

impl From<num::ParseFloatError> for AppError {
    fn from(error: num::ParseFloatError) -> Self {
        Self {
            kind: String::from("parse float"),
            message: error.to_string(),
        }
    }
}

impl From<str::Utf8Error> for AppError {
    fn from(error: str::Utf8Error) -> Self {
        Self {
            kind: String::from("utf8"),
            message: error.to_string(),
        }
    }
}

impl From<json::Error> for AppError {
    fn from(error: json::Error) -> Self {
        Self {
            kind: String::from("json"),
            message: error.to_string(),
        }
    }
}

impl From<csv::Error> for AppError {
    fn from(error: csv::Error) -> Self {
        Self {
            kind: String::from("csv"),
            message: error.to_string(),
        }
    }
}
