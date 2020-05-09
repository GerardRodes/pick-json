# pick json
Read a single one attribute of a json and stop there

## how does it work?
instead of reading the whole json it stops reading bytes once found the picked attribute and its value is returned

## run it
```shell
$ cargo build --release
$ target/release/pick-json tests/types.json emoji
"🔥"
```