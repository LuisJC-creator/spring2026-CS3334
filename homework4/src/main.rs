use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
            // TODO: Implement file creation logic
            println!("File '{}' created successfully.", filename);
        }
        FileOperation::Rename(old_name, new_name) => {
            // TODO: Implement file renaming logic
            println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
        }
    }
}

fn main() {
    println!("Choose an operation:");
    println!("1. Create a new file");
    println!("2. Rename an existing file");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            // TODO: Prompt for new filename and call perform_operation
        }
        "2" => {
            // TODO: Prompt for old and new filenames and call perform_operation
        }
        _ => println!("Invalid choice"),
    }
}