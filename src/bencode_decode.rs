use serde_json::Value;
use std::str::{self, FromStr};

pub enum BenValue {
    BenString(String),
    BenInteger(i64),
}

pub fn decode_bencoded_values(encoded_values: &[u8]) -> Value {
    let mut values = Vec::new();
    let mut index = 0;

    while index < encoded_values.len() {
        if encoded_values[index].is_ascii_digit() {
            let (decoded, next_index) =
                decode_string(&encoded_values[index..]).expect("Failed to decode string");
            values.push(decoded);
            index += next_index;
        } else if encoded_values[index] == b'i' {
            let (decoded, next_index) =
                decode_integer(&encoded_values[index..]).expect("Failed to decode integer");
            values.push(decoded);
            index += next_index;
        } else {
            panic!("Invalid format: expected a digit, 'i', 'l', or 'd'");
        }
    }

    Value::Array(
        values
            .into_iter()
            .map(|v| match v {
                BenValue::BenString(s) => Value::String(s),
                BenValue::BenInteger(i) => Value::Number(i.into()),
            })
            .collect(),
    )
}

fn decode_string(encoded_values: &[u8]) -> Result<(BenValue, usize), &'static str> {
    let mut length_str = String::new();
    let mut iter = encoded_values.iter().enumerate();

    // Extract the length of the string
    while let Some((index, &b)) = iter.next() {
        if b.is_ascii_digit() {
            length_str.push(b as char);
        } else if b == b':' {
            let length = usize::from_str(&length_str).map_err(|_| "Invalid length")?;
            let string_start = index + 1;
            let string_end = string_start + length;

            if string_end > encoded_values.len() {
                return Err("Invalid length");
            }

            let string = str::from_utf8(&encoded_values[string_start..string_end])
                .map_err(|_| "Invalid UTF-8 string")?;
            return Ok((BenValue::BenString(string.to_string()), string_end));
        } else {
            // print the whole string
            println!("length_str: {}", length_str);
            println!("b: {}", b as char);
            println!("index: {}", index);
            println!("encoded_values: {:?}", String::from_utf8(encoded_values.to_vec()).unwrap());
            return Err("Invalid format");
        }
    }

    Err("Invalid format")
}

fn decode_integer(encoded_values: &[u8]) -> Result<(BenValue, usize), &'static str> {
    let mut value_str = String::new();
    let mut iter = encoded_values.iter().enumerate();
    iter.next();

    // Extract the length of the string
    while let Some((index, &b)) = iter.next() {
        if b.is_ascii_digit() || b == b'-' {
            value_str.push(b as char);
        } else if b == b'e' {
            let value = i64::from_str(&value_str).map_err(|_| "Invalid value")?;
            return Ok((BenValue::BenInteger(value), index + 1));
        } else {
            return Err("Invalid format");
        }
    }

    Err("Invalid format")
}
