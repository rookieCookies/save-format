use sti::reader::Reader;

const MAGIC_VALUE : &[u8] = b"FANCYSAVEFORMAT";
const VERSION : u32 = 1;

pub struct ByteWriter {
    buffer: Vec<u8>,
}


impl ByteWriter {
    pub fn new() -> Self {
        let mut buffer = Vec::with_capacity(128);
        buffer.extend(MAGIC_VALUE);
        buffer.extend(VERSION.to_le_bytes());

        Self {
            buffer,
        }
    }


    pub fn finish(self) -> Vec<u8> {
        self.buffer
    }


    pub fn write_i8(&mut self, value: i8) {
        self.buffer.push(value.to_le_bytes()[0]);
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value.to_le_bytes()[0]);
    }

    pub fn write_u32(&mut self, value: u32) {
        self.buffer.extend(value.to_le_bytes());
    }

    pub fn write_u64(&mut self, value: u64) {
        self.buffer.extend(value.to_le_bytes());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.buffer.extend(value.to_le_bytes());
    }

    pub fn write_i64(&mut self, value: i64) {
        self.buffer.extend(value.to_le_bytes());
    }

    pub fn write_f32(&mut self, value: f32) {
        self.buffer.extend(value.to_le_bytes());
    }

    pub fn write_f64(&mut self, value: f64) {
        self.buffer.extend(value.to_le_bytes());
    }

    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(value as u8);
    }

    pub fn write_str(&mut self, str: &str) {
        self.write_bytes(str.as_bytes());
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_u32(bytes.len().try_into().expect("buffer is too big"));

        self.buffer.extend(bytes);
    }
}


pub struct ByteReader<'a> {
    reader: Reader<'a, u8>,
}


impl<'me> ByteReader<'me> {
    pub fn new(bytes: &'me [u8]) -> Option<Self> {
        let mut reader = Reader::new(bytes);
        if reader.next_n(MAGIC_VALUE.len()) != Some(MAGIC_VALUE) {
            return None;
        }

        if reader.next_u32_le() != Some(VERSION) {
            return None;
        }


        Some(Self { reader })
    }


    pub fn read_i8(&mut self) -> Option<i8> {
        Some(self.read_u8()? as i8)
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        self.reader.next_u8_le()
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        Some(self.read_u32()? as i32)
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        self.reader.next_u32_le()
    }

    pub fn read_i64(&mut self) -> Option<i64> {
        Some(self.read_u64()? as i64)
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        self.reader.next_u64_le()
    }

    pub fn read_f32(&mut self) -> Option<f32> {
        Some(f32::from_le_bytes(self.reader.next_array()?))
    }

    pub fn read_f64(&mut self) -> Option<f64> {
        Some(f64::from_le_bytes(self.reader.next_array()?))
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        Some(self.read_u8()? == 1)
    }

    pub fn read_bytes(&mut self) -> Option<&'me [u8]> {
        let len = self.read_u32()?;
        self.reader.next_n(len as usize)
    }

    pub fn read_str(&mut self) -> Option<&'me str> {
        Some(str::from_utf8(self.read_bytes()?).unwrap())
    }
}
