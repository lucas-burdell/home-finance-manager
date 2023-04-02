use std::fmt::Display;

#[derive(Debug, Clone, Responder)]
#[response(status = 500, content_type = "json")]
pub struct CustomError {
    message: String
}

impl CustomError {
    pub fn with_inner<T : Display>(message: String, cause: T) -> Self {
        Self { message: format!("{}. Inner error: {}", message, cause) }
    }
    pub fn msg(message: String) -> Self {
        Self { message: format!("{}", message) }
    }
    pub fn to_string(&self) -> String {
        self.message.to_string()
    }
}


#[macro_export]
macro_rules! conv_err {
    ($result:expr) => {
        match $result {
            Ok(val) => Ok(val),
            Err(e) => Err(CustomError::msg(e.to_string())),
        }
    };
}

#[macro_export]
macro_rules! expect_err {
    ($result:expr, $msg:expr) => {
        match $result {
            Ok(val) => Ok(val),
            Err(e) => Err(CustomError::with_inner($msg, e.to_string())),
        }
    };
}