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
            fs::File::create(&filename).unwrap();
            println!("File '{}' created successfully.", filename);
        }
        FileOperation::Rename(old_name, new_name) => {
            // TODO: Implement file renaming logic
            fs::rename(&old_name, &new_name).unwrap();
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
            println!("Enter a new name: ");
            let mut input_string = String::new();
            io::stdin().read_line(&mut input_string).unwrap();
            perform_operation(FileOperation::Create(input_string.trim().to_string()));
        }
        "2" => {
            // TODO: Prompt for old and new filenames and call perform_operation
            println!("Enter old file name: ");
            let mut old_name = String::new();
            io::stdin().read_line(&mut old_name).unwrap();
            println!("Enter new file name: ");
            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name).unwrap();
            perform_operation(FileOperation::Rename(old_name.trim().to_string(), new_name.trim().to_string()));

        }   
        _ => println!("Invalid choice"),
    }
}