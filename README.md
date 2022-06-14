# dwebpa

Dwebpa is a simple program that utilizes webp -package to convert all the files in the working directory from .webp to .png

You need to have dwebp installed (sudo apt install webp on Ubuntu)

## Installation

Installation requires Rust to be installed (https://www.rust-lang.org/tools/install). You can get this tool to work with the following commands written inside the repository directory:

```bash
cargo build --release
sudo cp ./target/release/dwebpa /usr/bin/
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
