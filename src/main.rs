use std::{env, fs::File, io::Read};

fn main() {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have enough arguments
    if args.len() < 3 {
        eprintln!("Usage: {} -c <filename>", args[0]);
        std::process::exit(1);
    }

    // Check if the first argument is "-c"
    if args[1] != "-c" {
        eprintln!("Unsupport option: {}", args[1]);
        std::process::exit(1);
    }

    // Read the file name from the arguments
    let filename = &args[2];

    // Open the file and read its content
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        }
    };

    let mut content = Vec::new();
    if let Err(error) = file.read_to_end(&mut content) {
        eprintln!("Error reading file {}: {}", filename, error);
        std::process::exit(1);
    }

    // Output the number of bytes in the file
    println!("{} {}", content.len(), filename);
}
