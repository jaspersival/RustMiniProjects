use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Number of * elements in the top row
    top_number: usize,
}
fn main() {
    let args = Args::parse();
    let mut i = args.top_number;
    while i > 0 {
        let row = "*".repeat(i);
        println!("{}", row);
        i -= 1;
    }
}
