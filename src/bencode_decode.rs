use serde_json::Value;
use std::result::Result::Ok;
use std::str::{self, FromStr};

pub enum BenValue {
    BenString(String),
    BenInteger(i64),
    BenList(Vec<BenValue>),
}

pub fn decode_bencoded_values(encoded_values: &[u8]) -> Value {
    let mut values = Vec::new();
    let mut index = 0;

    while index < encoded_values.len() {
        let (value, new_index) = decode_bencoded(encoded_values, index).unwrap();
        values.push(value);
        index += new_index;
    }

    to_json(&values)
}

fn decode_bencoded(encoded_values: &[u8], index: usize) -> Result<(BenValue, usize), &'static str> {
    if encoded_values[index].is_ascii_digit() {
        decode_string(&encoded_values[index..])
    } else if encoded_values[index] == b'i' {
        decode_integer(&encoded_values[index..])
    } else if encoded_values[index] == b'l' {
        decode_list(&encoded_values[index..])
    } else {
        Err("Invalid format: expected a digit, 'i', 'l', or 'd'")
    }
}

fn to_json(values: &[BenValue]) -> Value {
    let mut json_values = Vec::new();

    for value in values {
        match value {
            BenValue::BenString(string) => json_values.push(Value::String(string.to_string())),
            BenValue::BenInteger(integer) => json_values.push(Value::Number((*integer).into())),
            BenValue::BenList(list) => json_values.push(to_json(list)),
        }
    }

    Value::Array(json_values)
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

fn decode_list(encoded_values: &[u8]) -> Result<(BenValue, usize), &'static str> {
    let mut values: Vec<BenValue> = Vec::new();
    let mut current_index: usize = 1;

    if encoded_values[current_index] == b'e' {
        return Ok((BenValue::BenList(values), current_index + 1));
    }

    loop {
        let decoded = decode_bencoded(&encoded_values, current_index);

        if decoded.is_err() {
            let err = &decoded.err();
            panic!("Error parsing list: {}", err.unwrap());
        } else {
            let (value, size) = decoded.unwrap();

            values.push(value);
            current_index += size;
        }

        if encoded_values[current_index] == b'e' {
            break;
        }
    }

    Ok((BenValue::BenList(values), current_index + 1))
}
