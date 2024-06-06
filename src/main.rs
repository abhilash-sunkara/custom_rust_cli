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
                "grep" => grep_text(split_words[1], split_words[2], split_words[3]),
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


fn grep_text(file : &str, test: &str, dir:&str){
    let contents = fs::read_to_string(file.to_owned()+".txt").expect("panic");
    let mut start_index = 0;
    let mut end_index = 2;
    let mut line_list: Vec<String> = Vec::new();
    for (i, c) in contents.chars().enumerate() {
        if c == 0xA as char {
            end_index = i;
            line_list.push(contents[start_index..end_index].to_string());
            start_index = i;
        }
    }
    match dir {
        "-c" => {let mut count = 0;
                for s in line_list{
                    if s.contains(test){
                        count += 1;
                    }
                }
                println!("Number of occurrences : {}", count);
                }
        "-l" => {for s in line_list{
                    if s.contains(test){
                        println!("{}", s.trim());
                    }
                }
                },
        "-v" => {for s in line_list{
                    if !s.contains(test){
                        println!("{}", s.trim());
                    }
                }
        },
        &_ => {}
    }

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




