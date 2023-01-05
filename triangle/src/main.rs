use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Number of * elements in the top row
    top_number: usize,
}

#[derive(clap::Subcommand)]
enum Action {
    Top,
    Bottom
}

fn main() {
    let cmd = clap::Command::new("triangle").bin_name("triangle")
        .subcommand_required(true)
        .subcommand(clap::command!("top").arg(
            clap::Arg::new("NUMBER")
                .value_parser(clap::value_parser!(usize))
        ))
        .subcommand(clap::command!("bottom").arg(
            clap::Arg::new("NUMBER")
                .value_parser(clap::value_parser!(usize))
        ));
    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("top", matches)) => matches,
        Some(("bottom", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here")
    };
    let number = matches.get_one::<usize>("NUMBER").expect("Please provide a NUMBER");
    get_top_triangle(number);
}

fn get_top_triangle(number: &usize) {
    let mut i = *number;
    while i > 0 {
        let row = "*".repeat(i);
        println!("{}", row);
        i -= 1;
    }
}
