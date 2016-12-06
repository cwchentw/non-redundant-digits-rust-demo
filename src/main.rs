use std::io;
use std::io::Write;
use std::process;
use std::collections::HashSet;

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

    let x: i32 = 10;
    let mut count = 1;
    for i in 1..(x.pow(n)) {
        if count % 10 == 0 {
            println!("");
        }

        let num_string = i.to_string();
        let mut chars = num_string.chars();
        let mut hash = HashSet::new();
        let mut c = chars.next();
        while c != None {
            hash.insert(c);
            c = chars.next();
        }
        let mut digit = 0;
        let mut j = i;
        while j > 0 {
            j = j / 10;
            digit += 1;
        }
        let passed = hash.len() as i32 >= digit;
        if passed {
            print!("{} ", i);
            count += 1;
        }
    }
    println!("");
}
