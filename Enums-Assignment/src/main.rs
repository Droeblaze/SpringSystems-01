use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            Command::new("ls")
                .arg(&path)
                .status()
                .expect("Failed to execute ls");
        }
        FileOperation::Display(path) => {
            Command::new("cat")
                .arg(&path)
                .status()
                .expect("Failed to execute cat");
        }
        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to create file");

            if output.status.success() {
                println!("File '{}' created successfully.", path);
            } else {
                eprintln!("Failed to create file '{}'.", path);
            }
        }
        FileOperation::Remove(path) => {
            let output = Command::new("rm")
                .arg(&path)
                .output()
                .expect("Failed to execute rm");

            if output.status.success() {
                println!("File '{}' removed successfully.", path);
            } else {
                eprintln!("Failed to remove file '{}'. Does it exist?", path);
            }
        }
        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}

fn display_menu() {
    println!("\nFile Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
    println!();
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        display_menu();
        let choice = read_input("Enter your choice (0-5): ");

        let operation = match choice.as_str() {
            "0" => {
                println!("Goodbye!");
                break;
            }
            "1" => {
                let path = read_input("Enter directory path: ");
                FileOperation::List(path)
            }
            "2" => {
                let path = read_input("Enter file path: ");
                FileOperation::Display(path)
            }
            "3" => {
                let path = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                FileOperation::Create(path, content)
            }
            "4" => {
                let path = read_input("Enter file path: ");
                FileOperation::Remove(path)
            }
            "5" => FileOperation::Pwd,
            _ => {
                println!("Invalid option '{}'. Please enter a number between 0 and 5.", choice);
                continue;
            }
        };

        perform_operation(operation);
    }
}