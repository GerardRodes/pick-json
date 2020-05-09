use std::io::Error;
use std::env;
mod lib;

fn main () -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();

  println!("{}", lib::pick_json(&args[1], &args[2])?);

  Ok(())
}