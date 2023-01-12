use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
/*
This just a simple command line todo in Rust, I'm just testing Rust, and I wanted to have my first open source project.

the cons are:
- It takes forever to compile if you have some random libraries installed.
- Rust is complicated, it's not worth to program in it if you are in a hurry and don't have time.


the pros are
- Rust supports cross platform?
- Super fast language (Compiled to Binary)
- Memory safety (No stupid garbage collector there won't be CPU spikes if you use too much memory)
- Made few years ago (10 I think)
- It was recently added to the Linux Kernel :)

If Rust was a bit more simpler, I'd use it as my main language,

The license is MIT so you can do whatever you want but use it at your own risk and don't ask for support, I'm a junior IT student, and we learn HTML not Rust.

In a nutshell: Fix the bugs by yourself and make a pull and never ask me to fix them.
- NullByte3

*/

fn main() {
    let mut file = match File::open("todo.txt") {
        Ok(file) => file,
        Err(_) => {
            let path = Path::new("todo.txt");
            match File::create(&path) {
                Ok(file) => file,
                Err(e) => panic!("Failed to create {}: {}", path.display(), e),
            }
        }
    };

    loop {
        println!("1. View todo list");
        println!("2. Add todo item");
        println!("3. Remove todo item");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap();

        match choice {
            1 => view_todo_list(&mut file),
            2 => add_todo_item(&mut file),
            3 => remove_todo_item(&mut file),
            4 => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn view_todo_list(file: &mut File) {
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn add_todo_item(file: &mut File) {
    println!("Enter todo item:");
    let mut item = String::new();
    io::stdin().read_line(&mut item).unwrap();
    file.write_all(item.as_bytes()).unwrap();
}

fn remove_todo_item(file: &mut File) {
    println!("Enter todo item to remove:");
    let mut item = String::new();
    io::stdin().read_line(&mut item).unwrap();
    let item = item.trim();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let new_contents = contents.replace(item, "");
    let mut file = File::create("todo.txt").unwrap();
    file.write_all(new_contents.as_bytes()).unwrap();
}
