use clap::Parser;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path to a SystemVerilog module
    file_path: String,
}

#[derive(Debug)]
struct ParenLocations {
    open: Vec<Option<usize>>,
    closed: Vec<Option<usize>>,
}

impl ParenLocations {
    /// Constructor - what happens to a Vector when nothing is pushed into it?
    fn new() -> ParenLocations {
        ParenLocations {
            open: Vec::new(),
            closed: Vec::new(),
        }
    }

    /// Finds first '(' in each line of the string slice and appends to self.open
    fn find_open_locations(&mut self, text: &str) {
        for line in text.lines() {
            let line_contents = line;
            let paren_location_open = line_contents.find("(");
            self.open.push(paren_location_open);
        }
    }

    /// Finds first ')' in each line of the string slice and appends to self.closed
    fn find_closed_locations(&mut self, text: &str) {
        for line in text.lines() {
            let line_contents = line;
            let paren_location_open = line_contents.find(")");
            self.closed.push(paren_location_open);
        }
    }
}

impl fmt::Display for ParenLocations {
    /// The code in the fmt function is essentially like __str__ in Python, except there are a ton
    /// of formatting 'traits' to implement for the different primitive types that ParenLocations
    /// may encapsulate. Binary, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex, & Write.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(open=('{}'),closed=('{}'))",
            self.open.iter().count(),
            self.closed.iter().count()
        )
    }
}

fn main() {
    let cli = Args::parse();
    let file_path: &str = &cli.file_path;

    // Open file & read contents into mutable string
    let file = File::open(file_path);
    let mut file_contents = String::new();
    let _ = match file {
        Ok(mut file) => file.read_to_string(&mut file_contents),
        Err(err) => {
            println!("[ERROR] - Problem with reading '{}' : {}", file_path, err);
            exit(1);
        }
    };

    print!("{}", file_contents);

    // What if a line has more than a single '(' or ')' character?
    let mut parens = ParenLocations::new();
    parens.find_open_locations(&file_contents);
    parens.find_closed_locations(&file_contents);

    println!("Display using Debug   : {:?}", parens);
    println!("Display using Display : {}", parens);
}
