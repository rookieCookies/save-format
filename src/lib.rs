pub mod byte;

use std::{collections::HashMap, fmt::Write};

use sti::reader::Reader;

#[derive(Debug)]
pub enum Value<'a> {
    String(&'a str),

    Num(f64),
    Bool(bool),

    Vec2(f64, f64),

    Vec3(f64, f64, f64),

    Vec(Vec<f64>),
}


#[derive(Debug)]
pub enum Error {
    InvalidCharacter(usize),
    UnfinishedKey(usize),
    UnfinishedStr(usize),
}


pub fn parse_str<'me>(str: &'me str) -> Result<HashMap<&'me str, Value<'me>>, Error> {
    let mut pairs = HashMap::new();
    let mut reader = Reader::new(str.as_bytes());

    while let Some(chr) = reader.next() {
        match chr as char {
            '[' => {
                let key = reader.consume_while_slice(|&c| c != b']');
                if reader.next() != Some(b']') {
                    return Err(Error::UnfinishedKey(reader.offset()))
                }

                let key = str::from_utf8(key.0).unwrap();
                let val = value(&mut reader)?;

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


pub fn to_string(keys: HashMap<&str, Value>) -> String {
    let mut string = String::new();

    for (k, v) in keys.iter() {
        let _ = write!(string, "[{k}] ");

        let _ = match v {
            Value::String(s) => writeln!(string, "\"{s}\""),
            Value::Num(n) => writeln!(string, "{n}"),
            Value::Bool(b) => writeln!(string, "{b}"),
            Value::Vec2(n1, n2) => writeln!(string, "{n1} {n2}"),
            Value::Vec3(n1, n2, n3) => writeln!(string, "{n1} {n2} {n3}"),
            Value::Vec(items) => {
                for item in items {
                    let _ = write!(string, "{item} ");
                }

                writeln!(string)
            },
        };
    }

    string
}


fn value<'me>(reader: &mut Reader<'me, u8>) -> Result<Value<'me>, Error> {
    let Some(chr) = reader.next()
    else { return Err(Error::UnfinishedKey(reader.offset())) };


    Ok(match chr as char {
        '0'..='9' => {
            let mut nums = vec![];
            while reader.peek().is_some() && reader.peek() != Some(b'\n') {
                let num = number(reader);
                nums.push(num);
            }


            match nums.len() {
                0 => unreachable!(),
                1 => Value::Num(nums[0]),
                2 => Value::Vec2(nums[0], nums[1]),
                3 => Value::Vec3(nums[0], nums[1], nums[2]),
                _ => Value::Vec(nums),
            }
        }


        '"' => {
            let str = reader.consume_while_slice(|&c| c != b'"');
            if reader.next() != Some(b'"') {
                return Err(Error::UnfinishedKey(reader.offset()))
            }

            Value::String(str::from_utf8(str.0).unwrap())

        }


        't' => {
            assert_eq!(reader.next(), Some(b'r'));
            assert_eq!(reader.next(), Some(b'u'));
            assert_eq!(reader.next(), Some(b'e'));
            Value::Bool(true)
        }

        'f' => {
            assert_eq!(reader.next(), Some(b'a'));
            assert_eq!(reader.next(), Some(b'l'));
            assert_eq!(reader.next(), Some(b's'));
            assert_eq!(reader.next(), Some(b'e'));
            Value::Bool(false)
        }

        ' ' => value(reader)?,

        _ => return Err(Error::InvalidCharacter(reader.offset()))
    })
}


fn number<'me>(reader: &mut Reader<u8>) -> f64 {
    let (num, _) = reader.consume_while_slice(|&c| (c as char).is_numeric() || c == b'.');
    let num = str::from_utf8(num).unwrap();
    dbg!(&reader, num);
    let num : f64 = num.parse().unwrap();
    reader.next_if(|&f| f == b' ');

    num
}

