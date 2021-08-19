use std::io::{Read, Result, Write};

pub struct ReadStats<R>{
    wrapped: R,
    bytes: usize,
    counts: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self{
            wrapped,
            bytes: 0,
            counts: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.counts 
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.wrapped.read(buf) {
            Ok(n) => {
                self.bytes += n;
                self.counts += 1;
                return Ok(n)
            }
            e => e
        }
    }
}

pub struct WriteStats<W>{
    wrapped: W,
    bytes: usize,
    counts: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self{
            wrapped,
            bytes: 0,
            counts: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.counts
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.wrapped.write(buf) {
            Ok(n) => {
                self.bytes += n;
                self.counts += 1;
                return Ok(n)
            }
            e => e
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
