extern crate nfd;

use nfd::*;

fn main() {
    let result = open_file_dialog(None, None);

    match result {
        NFDResult::Okay(file_path) => println!("File path = {:?}", file_path),
        NFDResult::Cancel => println!("User canceled"),
        NFDResult::Error(error) => println!("Error: {}", error),
    }
}
