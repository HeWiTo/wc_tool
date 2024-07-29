use std::{
    env, 
    fs::File, 
    io::{
        BufRead, BufReader, Read
    }
};

fn main() {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have enough arguments
    if args.len() < 3 {
        eprintln!("Usage: {} -c <filename>", args[0]);
        std::process::exit(1);
    }

    // Read the file name from the arguments
    let option = &args[1];
    let filename = &args[2];

    // Open the file and create a buffered reader
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        }
    };

    // Check the given arguments (`-c` or `-l`) using match
    match option.as_str() {
        "-c" => {
            let mut content = Vec::new();
            let mut reader = BufReader::new(file);
            if let Err(error) = reader.read_to_end(&mut content) {
                eprintln!("Error reading file {}: {}", filename, error);
                std::process::exit(1);
            }
            println!("{} {}", content.len(), filename);
        }
        "-l" => {
            let reader = BufReader::new(file);
            let line_count = reader.lines().count();
            println!("{} {}", line_count, filename);
        }
        "-w" => {
            let reader = BufReader::new(file);
            let word_count = reader
                .lines()
                .map(|line| {
                    line.unwrap_or_else(|_| "".to_string())
                        .split_whitespace()
                        .count()
                })
                .sum::<usize>();
            println!("{} {}", word_count, filename);
        }
        _ => {
            eprintln!("Unsupported option: {}", option);
            std::process::exit(1);
        }
    }
    
}
