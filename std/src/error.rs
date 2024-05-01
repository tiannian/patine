pub trait PanicError {
    fn revert(&self);
}

pub enum Error {
    ArrayOutOfBound,
}

impl PanicError for Error {
    fn revert(&self) {}
}

pub type Result<T> = core::result::Result<T, Error>;
