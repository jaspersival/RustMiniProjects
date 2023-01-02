use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Currency to convert from
    #[arg(short, long, default_value_t = String::from("EURO"))]
    from_currency: String,
    ///Currency to convert to
    #[arg(short, long)]
    to_currency: String,
    ///Amount of money to convert
    currency_amount: f64,
}

fn main() {
    let args = Args::parse();
    println!(
        "Convert {} of {} to {}",
        args.currency_amount, args.from_currency, args.to_currency
    );
}
