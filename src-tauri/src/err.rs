

#[derive(thiserror::Error, Debug)]
#[error("{message}")]
pub enum Error {
    #[error("{message}")]
    SimpleError{message: String},
}

#[macro_export]
macro_rules! err {

    ($($arg:tt)*) => {{
        let res = err::Error::SimpleError { message: format!($($arg)*) };
        res
    }};

}

#[macro_export]
macro_rules! errErr {

    ($($arg:tt)*) => {{
        let res = Err(err::Error::SimpleError { message: format!($($arg)*) });
        res
    }};

}

#[derive(serde::Serialize)]
struct ErrorWrapper {
    error: String,
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}