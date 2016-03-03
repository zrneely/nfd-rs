# nfd-rs ![https://github.com/saurvs/nfd-rs/blob/master/LICENSE.md](https://img.shields.io/badge/license-MIT-blue.svg)

`nfd-rs` is a Rust binding to the library [nativefiledialog](https://github.com/mlabbe/nativefiledialog), that provides a convenient cross-platform interface to opening file dialogs on Linux, OS X and Windows.

Currently, only single file and save dialogs are supported, and the crate has been tested only on OS X. And yes, APIs may break with newer versions.

## Usage

* Add the dependency `nfd` in your ```Cargo.toml```
  ```toml
  [dependencies]
  nfd = { git = "https://github.com/saurvs/nfd-rs.git" }
  ```

* Open a single file dialog
  ```rust
  extern crate nfd;

  use nfd::*;

  fn main() {
    let filter_str = "";
    let default_path = "~";
    let result = open_file_dialog(filter_str, default_path);

    match result {
        NFDResult::Okay(file_path) => println!("File path = {:?}", file_path),
        NFDResult::Cancel => println!("User canceled"),
        NFDResult::Error(error) => println!("Error: {}", error),
    }
  }
  ```

## To build
Follow the instructions [here](https://github.com/mlabbe/nativefiledialog/blob/master/README.md) to build the libraries in C and an OS-specific language. Then, set the `NFD_LIB_DIR` environment variable to the path of the directory in which the libraries are stored. Now run `cargo build`.