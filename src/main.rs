use std::io;
use std::env;
use std::fs;
mod tokenizer;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cmd_args: Vec<String> = vec!();
    let mut file_path: String = String::new(); 
    args.iter().enumerate().for_each(|arg|{
        if arg.0 == args.len() - 1{
            file_path = arg.1.to_string()
        }else{
            cmd_args.push(arg.1.to_string())
        } 
    });
    println!("Euler Virtual Machine.
        Executing file {}
    ", file_path);
    match fs::read_to_string(file_path) {
        Ok(x) => println!("{:?}", tokenizer::tokenize(x)),
        Err(y) => panic!("Cannot read file, error is {}", {y})
    };
}



