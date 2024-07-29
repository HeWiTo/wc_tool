use std::{
    env, 
    fs::File, 
    io::{
        self, BufRead, BufReader, Read
    }
};

fn main() {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have enough arguments
    if args.len() < 2 {
        eprintln!("Usage: {} <option> [filename]", args[0]);
        std::process::exit(1);
    }

    // Determine if we should read from a file or from stdin
    let option = &args[1];
    let input: Box<dyn BufRead> = if args.len() == 2 || args.len() == 3 {
        match args.len() {
            2 => Box::new(BufReader::new(io::stdin())),
            3 => {
                let filename = &args[2];
                let file = match File::open(filename) {
                    Ok(file) => file,
                    Err(error) => {
                        eprintln!("Error opening file {}: {}", filename, error);
                        std::process::exit(1);
                    }
                };
                Box::new(BufReader::new(file))
            }
            _ => unreachable!(),
        }
    } else {
        eprintln!("Usage: {} <option> [filename]", args[0]);
        std::process::exit(1);
    };

    // Read the content from the input
    let mut content = String::new();
    let mut reader = input;
    if let Err(error) = reader.read_to_string(&mut content) {
        eprintln!("Error reading input: {}", error);
        std::process::exit(1);
    };

    let line_count = content.lines().count();
    let word_count = content.split_whitespace().count();
    let byte_count = content.as_bytes().len();
    let char_count = content.chars().count();

    if args.len() == 2 {
        println!("{} {} {} {}", line_count, word_count, byte_count, "[stdin]");
    } else {
        let filename = &args[2];
        match args[1].as_str() {
            "-c" => println!("{} {}", byte_count, filename),
            "-l" => println!("{} {}", line_count, filename),
            "-w" => println!("{} {}", word_count, filename),
            "-m" => println!("{} {}", char_count, filename),
            _ => {
                eprintln!("Unsupported option: {}", option);
                std::process::exit(1);
            }
        }
    }
    
}
