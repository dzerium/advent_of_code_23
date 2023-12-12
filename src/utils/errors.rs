use std::fmt;
use std::backtrace::Backtrace;

#[derive(Debug)]
pub enum ErrorCode {
    InvalidInput,
}

#[derive(Debug)]
pub enum AreaCode {
    Utils,
}

pub struct AppError {
    pub code: ErrorCode,
    pub area: AreaCode,
    pub message: String,
    pub backtrace: Backtrace,
 }

 impl AppError {
    pub fn new(code: ErrorCode, area: AreaCode, message: String) -> AppError {
        AppError {
            code,
            area,
            message,
            backtrace: Backtrace::capture(),
        }
    }
 }

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code {:?}, area: {:?}, message: {}", self.code, self.area, self.message)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppError {{ code: {:?}, area: {:?}, message: {} }}\n {}",
            self.code, self.area, self.message, self.backtrace
        )
    }
}