extern crate nfd;

use nfd::Response;

fn main() {
    let result = nfd::dialog().open_multiple().unwrap_or_else(|e| {
        panic!(e);
    });

    match result {
        Response::OkayMultiple(files) => println!("File path = {:?}", files),
        Response::Cancel => println!("User canceled"),
        _ => (),
    }
}
