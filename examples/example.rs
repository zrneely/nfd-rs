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
