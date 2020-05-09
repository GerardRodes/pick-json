use std::fs::File;
use std::io::{Error, ErrorKind, BufRead, BufReader};

pub fn pick_json (filepath: &str, property: &str) -> Result<String, Error> {
  let f = File::open(filepath)?;
  let mut f = BufReader::new(f);

  let mut is_prop = true;
  let mut found = false;

  loop {
    let mut buffer = vec![];
    let mut data: Vec<u8> = vec![];

    f.read_until(b',', &mut buffer)?;

    if buffer.len() == 0 {
      break
    }

    let mut i = 0;
    'outer: while i < buffer.len() {
      let non_data_byte = buffer[i];

      match non_data_byte {
        b':' => is_prop = false,
        b',' => is_prop = true,
        b'\n' | b'{' | b'}' | b'[' | b']' | b' ' | b'\t' | b'\r' => (),
        _ => {
          let mut escaped = false;

          while i < buffer.len() {
            let byte = buffer[i];

            if escaped {
              // println!("SCAPPED CHAR: {:?}", byte as char);
              data.push(byte);
              escaped = false;
              continue
            }

            match byte {
              b'\\' => escaped = true,
              b':' | b',' | b'}' | b'\n' | b'\r' => {
                if is_prop {
                  // remove quotations
                  data.remove(0);
                  data.pop();
                }

                let data_value = String::from_utf8_lossy(&data);

                if found {
                  // println!("FOUND PICKED VALUE: {}", data_value);
                  return Ok(data_value.to_string());
                }

                found = found || (is_prop && (data_value == property));

                // println!("READED VALUE: {}, is_prop: {:?}, is_value: {:?}", data_value, is_prop, is_value);


                data.clear();
                continue 'outer
              },
              _ => data.push(byte),
            }

            i += 1;
          }
        }
      }

      i += 1;
    }
  }


  Err(Error::from(ErrorKind::NotFound))
}