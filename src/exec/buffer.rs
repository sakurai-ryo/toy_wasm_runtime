use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Buffer {
    cursor: u32,
    buf: Vec<u8>,
}

type ReadVecFn<T> = Box<dyn Fn(&mut Buffer) -> Result<T>>;

impl Buffer {
    pub fn new(buf: Vec<u8>) -> Buffer {
        Buffer { cursor: 0, buf }
    }

    pub fn byte_len(&self) -> u64 {
        self.buf.len() as u64
    }

    pub fn eof(&self) -> bool {
        self.cursor >= self.byte_len() as u32
    }

    pub fn read_byte(&mut self) -> Result<u8> {
        let buf_slice = self.read_bytes(1)?;
        Ok(buf_slice[0])
    }

    pub fn read_bytes(&mut self, size: u32) -> Result<Vec<u8>> {
        if (self.buf.len() as u32) < (self.cursor + size) {
            return Err(anyhow!("Buffer too small"));
        }

        let slice = self.buf[self.cursor as usize..(self.cursor + size) as usize].to_vec();
        self.cursor += size;
        Ok(slice)
    }

    pub fn read_buffer(&mut self, size: u32) -> Result<Buffer> {
        let buf_slice = self.read_bytes(size)?;
        Ok(Buffer::new(buf_slice))
    }

    /// Read a 32-bit unsigned integer from the buffer.
    /// https://en.wikipedia.org/wiki/LEB128
    pub fn read_u32(&mut self) -> Result<u32> {
        let mut result = 0_u32;
        let mut shift = 0_u32;
        loop {
            let byte = self.read_byte()?;
            result |= ((byte & 0b01111111) as u32) << shift;
            shift += 7;
            // if the top bit of the byte is 0, return result.
            if (0b10000000 & byte) == 0 {
                return Ok(result);
            }
        }
    }

    /// Read a 32-bit signed integer from the buffer.
    /// https://en.wikipedia.org/wiki/LEB128
    pub fn read_i32(&mut self) -> Result<i32> {
        let mut result = 0_i32;
        let mut shift = 0_i32;
        loop {
            let byte = self.read_byte()?;
            result |= ((byte & 0b01111111) as i32) << shift;
            shift += 7;
            // if the top bit of the byte is 0, return result.
            if (0b10000000 & byte) == 0 {
                // sign extend negative numbers
                if (shift < 32) && ((byte & 0b01000000) != 0) {
                    result |= !0 << shift;
                }
                return Ok(result);
            }
        }
    }

    pub fn read_vec<T>(&mut self, f: ReadVecFn<T>) -> Result<Vec<T>> {
        let mut vec = Vec::new();
        let size = self.read_u32()?;
        for _ in 0..size {
            vec.push(f(self)?);
        }
        Ok(vec)
    }

    pub fn read_name(&mut self) -> Result<String> {
        let size = self.read_u32()?;
        let bytes = self.read_bytes(size)?;
        let name = String::from_utf8(bytes)?;
        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// test cases from https://github.com/xtuc/webassemblyjs/blob/master/packages/leb128/test/index.js
    #[rstest(
        input,
        expected,
        case(vec![0b00000000], 0),
        case(vec![0b00001000], 8),
        case(vec![0b10000000, 0b01111111], 16256),
        case(vec![0b11100101, 0b10001110, 0b00100110], 624485),
        case(vec![0b10000000, 0b10000000, 0b10000000, 0b01001111], 165675008),
        case(vec![0b10001001, 0b10000000, 0b10000000, 0b10000000, 0b00000000], 9)
    )]
    fn test_read_u32(input: Vec<u8>, expected: u32) {
        let mut buffer = Buffer::new(input);
        assert_eq!(buffer.read_u32().unwrap(), expected);
    }

    #[rstest(
        input,
        expected,
        case(vec![0x7eu8], -2),
        case(vec![0b10000001, 0b01111111], -127)
    )]
    fn test_read_i32(input: Vec<u8>, expected: i32) {
        let mut buffer = Buffer::new(input);
        assert_eq!(buffer.read_i32().unwrap(), expected);
    }
}
