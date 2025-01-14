use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check whether a filename was inputted:
    if args.len() == 1 {
        println!("Please provide a file to open.");
        return;
    } else if args.len() > 2 {
        println!("Please provide only one file to open.");
        return;
    }
    // Read in the contents of the file.
    let filename = &args[1];
    println!("Filename = {filename}");
}
