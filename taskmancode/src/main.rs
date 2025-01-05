use std::io::{self, BufRead, Write};
use std::fs::File;
use std::fs::OpenOptions;

fn main() {
        let stdin = io::stdin();
            let mut command = String::new();

                let filename = "task.txt";
                    let mut task_file = match OpenOptions::new()
                                .write(true)
                                        .create(true)
                                        .append(true)
                                                .open(filename) {
                                       Ok(file) => file,
                                        Err(e) => {
                                    println!("Error opening or creating file: {}", e);
                                                                                                        return;
                                                                                                                }
                                                                        };

                        println!("What do you want to do with taskman?");
                            stdin.read_line(&mut command).unwrap();
                                command = command.trim().to_lowercase(); 

                                    if command == "new task" {
                                        println!("Enter task name:");
                                            let mut taskname = String::new();
                                                stdin.read_line(&mut taskname).unwrap();
                                                    taskname = taskname.trim().to_string(); 
                                                    writeln!(task_file, "task: {}", taskname).unwrap();
                                                    println!("Task added!");
                                                        } else if command == "check task" {
                                                            println!("Enter task name to check:");
                                                            let mut taskname = String::new();
                                                            stdin.read_line(&mut taskname).unwrap();
                                                            let task_to_check = "task: ".to_string() + taskname.trim();
                                                                let mut task_file = match File::open(filename) {
                                                                            Ok(file) => file,
                                                                                    Err(e) => {
                                                                                                println!("Error opening file: {}", e);
                                                                                                            return;
                                                                                                                    }
                                                                                                                        };
                                                                    let reader = io::BufReader::new(task_file);
                                                                        for line in reader.lines() {
                                                                                    let line = line.unwrap();
                                                                                            if line == task_to_check {
                                                                                                        println!("Task found!");
                                                                                                                    return;
                                                                                                                            }
                                                                                                                                }
                                                                            println!("Task not found.");
                                                                                }
                                                            else if command == "stop task"{
                                                            println!("Enter task name to stop:");
                                                            let mut taskname = String::new();
                                                            stdin.read_line(&mut taskname).unwrap();
                                                            let task_to_stop = "task: ".to_string() + taskname.trim();
                                                                let mut task_file = match File::open(filename) {
                                                                            Ok(file) => file,
                                                                                    Err(e) => {
                                                                                                println!("Error opening file: {}", e);
                                                                                                            return;
                                                                                                                    }
                                                                                                                        };
                                                                    let reader = io::BufReader::new(task_file);
                                                                        let mut lines = Vec::new();
                                                                            for line in reader.lines() {
                                                                                        let line = line.unwrap();
                                                                                                if line == task_to_stop {
                                                                                                            println!("Task stopped!");
                                                                                                                        continue;
                                                                                                                                }
                                                                                                                                        lines.push(line);
                                                                                                                                            }
                                                                                let mut task_file = match OpenOptions::new()
                                                                                            .write(true)
                                                                                                    .create(true)
                                                                                                            .truncate(true)
                                                                                                                    .open(filename) {
                                                                                                                Ok(file) => file,
                                                                                                                        Err(e) => {
                                                                                                                                    println!("Error opening or creating file: {}", e);
                                                                                                                                                return;
                                                                                                                                                    }
                                                                                                                                                        };
                                                                                    for line in lines {
                                                                                                writeln!(task_file, "{}", line).unwrap();
                                                                                                    }
                                                                                        }
                                                            else if command == "list tasks" {
                                                            }
                                                            else {
                                                            println!("Invalid command.");
                                                            
                                                                                                                            }
}

