use std::{
    env, 
    fs::File, 
    io::{
        BufReader, Read
    }
};

fn main() {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have enough arguments
    if args.len() < 2 {
        eprintln!("Usage: {} -c <filename>", args[0]);
        std::process::exit(1);
    }

    // Read the file name from the arguments
    let filename = if args.len() == 2 {
        &args[1]
    } else {
        &args[2]
    };

    // Open the file and create a buffered reader
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        }
    };

    let mut content = Vec::new();
    let mut reader = BufReader::new(file);
    if let Err(error) = reader.read_to_end(&mut content) {
        eprintln!("Error reading file {}: {}", filename, error);
        std::process::exit(1);
    }

    let content_string = String::from_utf8_lossy(&content);

    let line_count = content_string.lines().count();
    let word_count = content_string.split_whitespace().count();
    let byte_count = content.len();
    let char_count = content_string.chars().count();

    if args.len() == 2 {
        println!("{} {} {} {}", line_count, word_count, byte_count, filename);
    } else {
        match args[1].as_str() {
            "-c" => println!("{} {}", byte_count, filename),
            "-l" => println!("{} {}", line_count, filename),
            "-w" => println!("{} {}", word_count, filename),
            "-m" => println!("{} {}", char_count, filename),
            _ => {
                eprintln!("Unsupported option: {}", args[1]);
                std::process::exit(1);
            }
        }
    }
    
}
