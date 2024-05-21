use colour::red;
use std::env;
use std::fs;
use std::process::exit;
mod tokenizer;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cmd_args: Vec<String> = vec![];
    let mut file_path: String = String::new();
    args.iter().enumerate().for_each(|arg| {
        if arg.0 == args.len() - 1 {
            file_path = arg.1.to_string()
        } else {
            cmd_args.push(arg.1.to_string())
        }
    });
    println!(
        "Euler Virtual Machine. V{}.
      Executing file {}
    ",
        env!("CARGO_PKG_VERSION"),
        file_path
    );
    match fs::read_to_string(file_path) {
        Ok(x) => println!("{:?}", tokenizer::tokenize(x)),
        Err(y) => {
            red!("Cannot read file. {} \n.", { y });
            exit(1)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
}
