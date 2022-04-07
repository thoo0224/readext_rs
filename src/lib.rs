use byteorder::{ReadBytesExt, BigEndian, LittleEndian};

use std::result;
use std::error;
use std::io;

pub type ReaderResult<T> = result::Result<T, Box<dyn error::Error>>;

pub trait ReadExt {

    fn read_array<T, F>(&mut self, serialize: F) -> ReaderResult<Vec<T>>
    where F: Fn(&mut Self) -> T;

    fn read_array_be<T, F>(&mut self, serialize: F) -> ReaderResult<Vec<T>>
    where F: Fn(&mut Self) -> T;

    fn read_array_with_length<T, F>(&mut self, serialize: F, length: i32) -> ReaderResult<Vec<T>>
    where F: Fn(&mut Self) -> T;

    fn read_fstring(&mut self) -> ReaderResult<String>;

    fn read_i32_le(&mut self) -> ReaderResult<i32>;
    fn read_u32_le(&mut self) -> ReaderResult<u32>;

    fn read_i64_le(&mut self) -> ReaderResult<i64>;
    fn read_u64_le(&mut self) -> ReaderResult<u64>;

    fn read_i32_be(&mut self) -> ReaderResult<i32>;
    fn read_u32_be(&mut self) -> ReaderResult<u32>;

    fn read_i64_be(&mut self) -> ReaderResult<i64>;
    fn read_u64_be(&mut self) -> ReaderResult<u64>;

}

impl<Impl> ReadExt for Impl
where
    Impl: ReadBytesExt + io::Read
{

    #[inline]
    fn read_array<T, F>(&mut self, serialize: F) -> ReaderResult<Vec<T>>
    where 
        F: Fn(&mut Self) -> T 
    {
        let length = self.read_i32_le()?;
        self.read_array_with_length(serialize, length)
    }

    #[inline(always)]
    fn read_array_be<T, F>(&mut self, serialize: F) -> ReaderResult<Vec<T>>
    where 
        F: Fn(&mut Self) -> T 
    {
        let length = self.read_i32::<BigEndian>()?;
        self.read_array_with_length(serialize, length)
    }

    fn read_array_with_length<T, F>(&mut self, serialize: F, length: i32) -> ReaderResult<Vec<T>>
    where 
        F: Fn(&mut Self) -> T 
    {
        let mut result = Vec::with_capacity(usize::try_from(length)?);
        for _ in 0..length {
            let item = serialize(self);
            result.push(item);
        }

        Ok(result)
    }

    fn read_fstring(&mut self) -> ReaderResult<String> {
        let length = self.read_i32_le()?;
        if length == 0 {
            return Ok(String::from(""));
        }

        if length < 0  {
            if length == i32::MIN {
                panic!("Invalid FString")
            }

            let len = -length * 2;
            let mut buffer: Vec<u8> = vec![0; usize::try_from(len)?]; 
            self.read_exact(buffer.as_mut_slice())?;

            // TODO
            panic!("Unicode FString's are not supported yet.");
        }

        let len = usize::try_from(length - 1)?;
        let mut buffer = vec![0u8; usize::try_from(length)?];
        self.read_exact(buffer.as_mut_slice())?;

        Ok(String::from_utf8(buffer[0..len].to_vec())?)
    }

    #[inline(always)]
    fn read_i32_le(&mut self) -> ReaderResult<i32> {
        Ok(self.read_i32::<LittleEndian>()?)
    }

    #[inline(always)]
    fn read_u32_le(&mut self) -> ReaderResult<u32> {
        Ok(self.read_u32::<LittleEndian>()?)
    }

    #[inline(always)]
    fn read_i64_le(&mut self) -> ReaderResult<i64> {
        Ok(self.read_i64::<LittleEndian>()?)
    }

    #[inline(always)]
    fn read_u64_le(&mut self) -> ReaderResult<u64> {
        Ok(self.read_u64::<LittleEndian>()?)
    }

    #[inline(always)]
    fn read_i32_be(&mut self) -> ReaderResult<i32> {
        Ok(self.read_i32::<BigEndian>()?)
    }

    #[inline(always)]
    fn read_u32_be(&mut self) -> ReaderResult<u32> {
        Ok(self.read_u32::<BigEndian>()?)
    }

    #[inline(always)]
    fn read_i64_be(&mut self) -> ReaderResult<i64> {
        Ok(self.read_i64::<BigEndian>()?)
    }

    #[inline(always)]
    fn read_u64_be(&mut self) -> ReaderResult<u64> {
        Ok(self.read_u64::<BigEndian>()?)
    }

}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use byteorder::{ReadBytesExt, LittleEndian};

    use crate::ReadExt;

    #[test]
    fn read_array() {
        let mut cursor = Cursor::new(vec![2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
        let result = cursor.read_array(|r| r.read_i32::<LittleEndian>().unwrap()).unwrap();

        assert_eq!(result.as_slice(), &[3, 4]);
    }

    #[test]
    fn read_fstring() {
        let mut cursor = Cursor::new(vec![6u8, 0, 0, 0, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x00]);
        let result = cursor.read_fstring().unwrap();

        assert_eq!(result, "Hello")
    }

}
