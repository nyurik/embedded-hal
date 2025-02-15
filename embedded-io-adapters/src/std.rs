//! Adapters to/from `std::io` traits.

/// Adapter from `std::io` traits.
#[derive(Clone)]
pub struct FromStd<T: ?Sized> {
    inner: T,
}

impl<T> FromStd<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> FromStd<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ?Sized> embedded_io::ErrorType for FromStd<T> {
    type Error = std::io::Error;
}

impl<T: std::io::Read + ?Sized> embedded_io::Read for FromStd<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.inner.read(buf)
    }
}

impl<T: std::io::BufRead + ?Sized> embedded_io::BufRead for FromStd<T> {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

impl<T: std::io::Write + ?Sized> embedded_io::Write for FromStd<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.inner.flush()
    }
}

impl<T: std::io::Seek + ?Sized> embedded_io::Seek for FromStd<T> {
    fn seek(&mut self, pos: embedded_io::SeekFrom) -> Result<u64, Self::Error> {
        self.inner.seek(pos.into())
    }
}

/// Adapter to `std::io` traits.
#[derive(Clone)]
pub struct ToStd<T: ?Sized> {
    inner: T,
}

impl<T> ToStd<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> ToStd<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: embedded_io::Read + ?Sized> std::io::Read for ToStd<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.inner.read(buf).map_err(to_std_error)
    }
}

impl<T: embedded_io::Write + ?Sized> std::io::Write for ToStd<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.inner.write(buf).map_err(to_std_error)
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.inner.flush().map_err(to_std_error)
    }
}

impl<T: embedded_io::Seek + ?Sized> std::io::Seek for ToStd<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64, std::io::Error> {
        self.inner.seek(pos.into()).map_err(to_std_error)
    }
}

/// Convert a embedded-io error to a std::io::Error
pub fn to_std_error<T: embedded_io::Error>(err: T) -> std::io::Error {
    std::io::Error::new(err.kind().into(), format!("{:?}", err))
}
