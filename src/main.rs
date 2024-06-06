use std::thread;
use std::time::Duration;

use console::Term;
use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs;
use std::env;


fn main() {

    let term = Term::stdout();
    thread::sleep(Duration::from_millis(2000));
    let mut prompt: bool = true;
    let mut is_editing: bool = false;
    let mut input:String = String::new();

    let mut split_words: Vec<&str> = input.trim().split_whitespace().collect();
    let mut file;
    let mut editing_file = String::new();

    while prompt {
        if !is_editing {
            input.clear();
            term.write_line("Enter a message").expect("TODO: panic message");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            split_words = input.trim().split_whitespace().collect();
            if split_words.len() < 1 { continue }
            match split_words[0] {
                "echo" => term.write_line(if split_words.len() > 1 { split_words[1] } else { " " }).expect("panic"),
                "exit" => {
                    prompt = false;
                    term.write_line("Exiting command line").expect("TODO: panic message");
                },
                "create" => { File::create(if split_words.len() > 1 { split_words[1] } else { "test" }.to_owned() + ".txt", ).expect("TODO: panic message"); },
                "grep" => term.write_line(&grep_text(split_words[1], split_words[2].to_string()).to_string()).expect("panic"),
                "edit" => {editing_file = edit_file(split_words).parse().unwrap(); is_editing = true},
                _ => term.write_line("nothing").expect("panic"),
            }
        } else {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim() == "exit"{
                is_editing = false;
                continue
            }
            input = edit_input_string(&input).parse().unwrap();
            file = OpenOptions::new().append(true).create(true).open(editing_file.to_owned() + ".txt").expect("panic");
            file.write_all(input.as_ref()).expect("panic");

            if input == "exit" {
                is_editing = false
            }
        }
    }
    term.clear_line().expect("TODO: panic message");
}

fn grep_text(file : &str, test: String) -> u32{
    println!("opening file + {file}.txt");
    let contents = fs::read_to_string(file.to_owned()+".txt").expect("panic");
    println!("{}", contents);
    match contents.find(&test){
        None => {0}
        Some(u) => {
            println!("{}", &contents[u..contents.len()-1]);
            u.try_into().unwrap()
        }
    }
}

fn grep_text_updated(file : &str, test: String){
    println!("opening file + {file}.txt");
    let contents = fs::read_to_string(file.to_owned()+".txt").expect("panic");


}



fn edit_file(input : Vec<&str>) -> &str {
    if input.len() < 2 {
        println!("No file entered");
    } else {
        println!("file found : {}.txt", input[1]);
    }
    input[1]
}

fn edit_input_string(input : &String) -> String {
    input[1..input.len()-2].to_string() + "\n"
}




