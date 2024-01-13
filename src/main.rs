use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path to a SystemVerilog module
    file_path: String,

    /// Connects ports explicitly by name
    #[arg(short, long)]
    by_name: bool,

    /// Connects ports implicitly
    #[arg(short, long)]
    implicit: bool,
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
}

fn _port_names(_arg: &str) -> String {
    unimplemented!();
}

fn _port_types(_arg: &str) -> String {
    unimplemented!();
}

fn _port_directions(_arg: &str) -> String {
    unimplemented!();
}
