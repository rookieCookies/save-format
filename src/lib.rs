pub mod byte;

use std::{collections::HashMap, fmt::Write};

use glam::{Vec2, Vec3};
use sti::reader::Reader;

pub use sti::arena::Arena;

#[derive(Debug, Clone, Copy)]
pub enum Value<'a> {
    String(&'a str),

    Num(f32),
    Bool(bool),

    Vec2(Vec2),

    Vec3(Vec3),

    Vec(&'a [f32]),

    None,
}


#[derive(Debug)]
pub enum Error {
    InvalidCharacter(usize),
    UnfinishedKey(usize),
    UnfinishedStr(usize),
}


impl<'a> Value<'a> {
    pub fn as_str(self) -> &'a str {
        let Value::String(val) = self
        else { unreachable!() };
        val
    }

    pub fn as_num(self) -> f32 {
        let Value::Num(val) = self
        else { unreachable!() };
        val
    }

    pub fn as_vec2(self) -> Vec2 {
        let Value::Vec2(val) = self
        else { unreachable!() };
        val
    }

    pub fn as_vec3(self) -> Vec3 {
        let Value::Vec3(val) = self
        else { unreachable!() };
        val
    }

    pub fn as_vec(self) -> &'a [f32] {
        let Value::Vec(val) = self
        else { unreachable!() };
        val
    }
}


pub fn parse_str<'me>(arena: &'me Arena, str: &'me str) -> Result<HashMap<&'me str, Value<'me>>, Error> {
    let mut pairs = HashMap::new();
    let mut reader = Reader::new(str.as_bytes());

    while let Some(chr) = reader.next() {
        match chr as char {
            '[' => {
                let key = reader.consume_while_slice(|&c| c != b' ');
                if key.0.last() != Some(&b']') {
                    return Err(Error::UnfinishedKey(reader.offset()))
                }

                let key = str::from_utf8(&key.0[..key.0.len()-1]).unwrap();
                let val = value(arena, &mut reader)?;

                pairs.insert(key, val);
            }


            '\n' | '\r' | ' ' => continue,

            _ => {
                return Err(Error::InvalidCharacter(reader.offset()));
            }
        }
    }

    Ok(pairs)
}



pub fn slice_to_string(keys: &[(&str, Value)]) -> String {
    let mut string = String::new();

    for (k, v) in keys.iter() {
        let _ = write!(string, "[{k}] ");

        let _ = match v {
            Value::String(s) => writeln!(string, "\"{s}\""),
            Value::Num(n) => writeln!(string, "{n}"),
            Value::Bool(b) => writeln!(string, "{b}"),
            Value::Vec2(v) => writeln!(string, "{} {}", v.x, v.y),
            Value::Vec3(v) => writeln!(string, "{} {} {}", v.x, v.y, v.z),
            Value::None => writeln!(string),
            Value::Vec(items) => {
                for item in items.iter() {
                    let _ = write!(string, "{item} ");
                }

                writeln!(string)
            },
        };
    }

    string
}



pub fn hashmap_to_string(keys: HashMap<&str, Value>) -> String {
    let mut string = String::new();

    for (k, v) in keys.iter() {
        let _ = write!(string, "[{k}] ");

        let _ = match v {
            Value::String(s) => writeln!(string, "\"{s}\""),
            Value::Num(n) => writeln!(string, "{n}"),
            Value::Bool(b) => writeln!(string, "{b}"),
            Value::Vec2(v) => writeln!(string, "{} {}", v.x, v.y),
            Value::Vec3(v) => writeln!(string, "{} {} {}", v.x, v.y, v.z),
            Value::None => writeln!(string),
            Value::Vec(items) => {
                for item in items.iter() {
                    let _ = write!(string, "{item} ");
                }

                writeln!(string)
            },
        };
    }

    string
}


fn value<'me>(arena: &'me Arena, reader: &mut Reader<'me, u8>) -> Result<Value<'me>, Error> {
    let Some(chr) = reader.peek()
    else { return Err(Error::UnfinishedKey(reader.offset())) };


    Ok(match chr as char {
        '-' | '0'..='9' => {
            let mut nums = sti::vec::Vec::new_in(arena);
            while reader.peek().is_some() && reader.peek() != Some(b'\n') {
                let num = number(reader);
                if nums.is_empty() && chr == b'-' {
                    nums.push(-num);
                } else {
                    nums.push(num);
                }
            }

            match nums.len() {
                0 => unreachable!(),
                1 => Value::Num(nums[0]),
                2 => Value::Vec2(Vec2::new(nums[0], nums[1])),
                3 => Value::Vec3(Vec3::new(nums[0], nums[1], nums[2])),
                _ => Value::Vec(nums.leak()),
            }
        }


        '"' => {
            let _ = reader.next();
            let str = reader.consume_while_slice(|&c| c != b'"');
            if reader.next() != Some(b'"') {
                return Err(Error::UnfinishedKey(reader.offset()))
            }
            dbg!(str);

            Value::String(str::from_utf8(str.0).unwrap())

        }


        't' => {
            assert_eq!(reader.next(), Some(b't'));
            assert_eq!(reader.next(), Some(b'r'));
            assert_eq!(reader.next(), Some(b'u'));
            assert_eq!(reader.next(), Some(b'e'));
            Value::Bool(true)
        }

        'f' => {
            assert_eq!(reader.next(), Some(b'f'));
            assert_eq!(reader.next(), Some(b'a'));
            assert_eq!(reader.next(), Some(b'l'));
            assert_eq!(reader.next(), Some(b's'));
            assert_eq!(reader.next(), Some(b'e'));
            Value::Bool(false)
        }



        '\n' => Value::None,
        ' ' => {
            let _ = reader.next();
            value(arena, reader)?
        },

        _ => {
            println!("invalid character {}", chr as char);
            return Err(Error::InvalidCharacter(reader.offset()))
        }
    })
}


fn number<'me>(reader: &mut Reader<u8>) -> f32 {
    let is_neg = if reader.peek() == Some(b'-') {
        let _ = reader.next();
        true
    } else { false };
    let (num, _) = reader.consume_while_slice(|&c| (c as char).is_numeric() || c == b'.');
    let num = str::from_utf8(num).unwrap();
    let num : f32 = num.parse().unwrap();
    reader.next_if(|&f| f == b' ');

    if is_neg { -num }
    else { num }
}

