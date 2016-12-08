extern crate pcre;
extern crate enum_set;

use std::io;
use std::io::{stderr, Write};
use std::process;
use enum_set::{EnumSet};
use pcre::{Pcre, CompileOption};

fn main() {
    print!("Input a number: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let n = match input.trim().parse::<u32>() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid integer");
            process::exit(1);
        }
    };

    let options: EnumSet<CompileOption> = EnumSet::new();
    let mut pattern = match Pcre::compile_with_options(r"(\d).*\1", &options) {
        Ok(re) => re,
        Err(err) => {
            writeln!(stderr(), "The pattern could not be compiled: {}", err)
                .unwrap();
            return;
        }
    };
    let x: i32 = 10;
    let mut count = 1;
    for i in 1..(x.pow(n)) {
        if count % 10 == 0 {
            println!("");
        }

        let num_string = i.to_string();
        let matched = pattern.exec(&num_string);
        let dup = match matched {
            None => false,
            Some(_) => true
        };

        if !dup {
            print!("{} ", i);
            count += 1;
        }
    }
    println!("");
}
