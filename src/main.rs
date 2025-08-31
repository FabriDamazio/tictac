use std::{
    io::{Write, stdout},
    thread, time,
};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Tictac")
        .version("0.1.0")
        .author("Fabricio Damazio <fabridamazio@gmail.com>")
        .about("A Rust-based CLI timer for efficient task tracking")
        .arg(
            Arg::new("time")
                .required(true)
                .index(1)
                .help("Time to countdown in seconds")
                .value_parser(parse_time),
        )
        .arg(
            Arg::new("title")
                .required(false)
                .index(2)
                .help("The timer title"),
        )
        .get_matches();

    let title = matches.get_one::<String>("title");
    
    if let Some(t) = title {
       println!("Timer: {t}"); 
    }
    else {
        println!("Timer started");
    }

    let time = matches.get_one::<u64>("time").expect("Time is required");

    for i in (1..=*time).rev() {
        print!("\r{} seconds remaining", i);
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn parse_time(s: &str) -> Result<u64, String> {
    let s = s.trim();

    if s.is_empty() {
        return Err("Time cannot be empty".to_string());
    }

    s.parse::<u64>()
        .map_err(|e| format!("Invalid time format: {}", e))
}

