use failure::Error;

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Win32 error occurred: {}", _0)]
    Os(u32),
}

pub type Result<T> = ::std::result::Result<T, Error>;
