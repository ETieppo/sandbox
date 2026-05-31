use std::io::{Error, ErrorKind};

pub struct Log;

impl Log {
	pub fn err<T: Into<String>>(message: T) -> Error {
		Error::new(ErrorKind::InvalidData, message.into())
	}
}
