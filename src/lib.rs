use std::fs::File;
use std::io::{Error, ErrorKind, BufRead, BufReader};

pub fn pick_json (filepath: &str, property: &str) -> Result<String, Error> {
  let f = File::open(filepath)?;
  let mut f = BufReader::new(f);

  let mut is_prop = true;
  let mut found = false;
  let mut escaped = false;
  let mut is_reading_text = false;

  let mut buffer = vec![];
  let mut data: Vec<u8> = vec![];
  while f.read_until(b',', &mut buffer)? > 0 {
    // println!("BUFFER: {:?}", buffer.iter().map(|b| *b as char).collect::<Vec<char>>());

    let mut i = 0;
    'outer: while i < buffer.len() {
      let non_data_byte = buffer[i];

      if !is_reading_text {
        if non_data_byte == b':' {
          is_prop = false;
          i += 1;
          continue;
        } else if non_data_byte == b',' {
          is_prop = true;
          i += 1;
          continue;
        } else if
          non_data_byte == b'\n' ||
          non_data_byte == b'{' ||
          non_data_byte == b'}' ||
          non_data_byte == b'[' ||
          non_data_byte == b']' ||
          non_data_byte == b' ' ||
          non_data_byte == b'\t' ||
          non_data_byte == b'\r' {
          i += 1;
          continue;
        }
      }

      while i < buffer.len() {
        let byte = buffer[i];

        // println!("char[{}](rt: {} | prop: {} | found: {}): {:?} => {:?}", i, is_reading_text, is_prop, found, byte as char, data.iter().map(|b| *b as char).collect::<Vec<char>>());

        if escaped {
          // println!("\tscapped");
          data.push(byte);
          escaped = false;
          i += 1;
          continue
        }

        match byte {
          b'\\' => escaped = true,
          b':' | b',' | b'\n' | b'}' | b']' | b' ' | b'\t' | b'\r' if !is_reading_text => {
            let data_value = String::from_utf8_lossy(&data);

            if found {
              // println!("FOUND PICKED VALUE: {}", data_value);
              return Ok(data_value.to_string());
            }

            data.clear();
            is_prop = true;
            i += 1;
            continue 'outer
          },
          b'"' if is_reading_text => {
            if is_prop {
              // remove quotation
              data.remove(0);
            } else {
              data.push(byte);
            }

            let data_value = String::from_utf8_lossy(&data);

            if found {
              // println!("FOUND PICKED VALUE: {}", data_value);
              return Ok(data_value.to_string());
            }

            found = found || (is_prop && (data_value == property));

            data.clear();
            i += 1;
            is_reading_text = false;
            continue 'outer
          },
          _ => {
            data.push(byte);
            is_reading_text = is_reading_text || byte == b'"';
          },
        }

        i += 1;
      }

      i += 1;
    }

    buffer.clear()
  }


  Err(Error::from(ErrorKind::NotFound))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    assert_eq!(pick_json("tests/simple.json", "text").unwrap(), String::from("\"hi\""));
  }

  #[test]
  fn simple_min() {
    assert_eq!(pick_json("tests/simple.min.json", "text").unwrap(), String::from("\"hi\""));
  }

  #[test]
  fn types_text() {
    assert_eq!(pick_json("tests/types.json", "text").unwrap(), String::from("\"hi\""));
  }

  #[test]
  fn types_number() {
    assert_eq!(pick_json("tests/types.json", "number").unwrap(), String::from("1"));
  }

  #[test]
  fn types_boolean() {
    assert_eq!(pick_json("tests/types.json", "boolean").unwrap(), String::from("true"));
  }

  #[test]
  fn types_emoji() {
    assert_eq!(pick_json("tests/types.json", "emoji").unwrap(), String::from("\"🔥\""));
  }

  #[test]
  fn types_min() {
    assert_eq!(pick_json("tests/types.min.json", "text").unwrap(), String::from("\"hi\""));
    assert_eq!(pick_json("tests/types.min.json", "number").unwrap(), String::from("1"));
    assert_eq!(pick_json("tests/types.min.json", "boolean").unwrap(), String::from("true"));
    assert_eq!(pick_json("tests/types.min.json", "emoji").unwrap(), String::from("\"🔥\""));
  }

  #[test]
  fn weird_escaped_n() {
    assert_eq!(pick_json("tests/weird.json", "escaped_n").unwrap(), String::from("\"hi\\n\""));
  }

  #[test]
  fn weird_escaped_n_and_r() {
    assert_eq!(pick_json("tests/weird.json", "escaped_n_and_r").unwrap(), String::from("\"hi\\n\\\\r\""));
  }

  #[test]
  fn weird_escaped_n_and_r_and_keys() {
    assert_eq!(pick_json("tests/weird.json", "escaped_n_and_r_and_keys").unwrap(), String::from("\"hi\\n\\\\r,:\""));
  }

  #[test]
  fn weird_number() {
    assert_eq!(pick_json("tests/weird.json", "number").unwrap(), String::from("1"));
  }

  #[test]
  fn weird_boolean() {
    assert_eq!(pick_json("tests/weird.json", "boolean").unwrap(), String::from("true"));
  }

  #[test]
  fn weird_emoji() {
    assert_eq!(pick_json("tests/weird.json", "emoji").unwrap(), String::from("\"🔥\""));
  }
}