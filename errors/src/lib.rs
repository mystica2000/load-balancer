use std::fmt;

#[derive(Debug)]
pub enum CustomError {
  Ipv4AddrParseError,
  TcpOnListenError,
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CustomError::Ipv4AddrParseError => write!(f, "IPV4 Address cannot be parsed - Invalid"),
      CustomError::TcpOnListenError => write!(f, "Tcp cannot listen "),
    }
  }
}

#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
  Custom(CustomError)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::Io(err) => write!(f, "{}",err),
      Error::Custom(err) => write!(f, "{}",err),
    }
  }
}


impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::Io(err)
  }
}

impl From<CustomError> for Error {
  fn from(err: CustomError) -> Self {
      Error::Custom(err)
  }
}