use clap::Parser;
use std::str::FromStr;

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

#[derive(Debug, PartialEq)]
enum Currency {
    EURO,
    DOLLAR,
    POUND,
    YEN,
}

impl FromStr for Currency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EURO" => Ok(Self::EURO),
            "DOLLAR" => Ok(Self::DOLLAR),
            "POUND" => Ok(Self::POUND),
            "YEN" => Ok(Self::YEN),
            _ => Err(()),
        }
    }
}

struct ExchangeRate {
    euro: f64,
    dollar: f64,
    pound: f64,
    yen: f64,
}

impl ExchangeRate {
    fn get(&self, currency: &Currency) -> f64 {
        match currency {
            Currency::EURO => self.euro,
            Currency::DOLLAR => self.dollar,
            Currency::POUND => self.pound,
            Currency::YEN => self.yen,
        }
    }
    fn convert(&self, from_currency: &Currency, to_currency: &Currency) -> f64 {
        self.get(to_currency) / self.get(from_currency)
    }
}

fn get_exchange_rate(from_currency: &Currency, to_currency: &Currency) -> f64 {
    let exchange_rate = ExchangeRate {
        euro: 1.0,
        dollar: 1.1,
        pound: 0.9,
        yen: 138.0,
    };
    exchange_rate.convert(from_currency, to_currency)
}

fn main() {
    let args = Args::parse();
    println!(
        "Convert {} {} to {}",
        args.currency_amount, args.from_currency, args.to_currency
    );
    let from_currency = Currency::from_str(&args.from_currency).unwrap();
    let to_currency = Currency::from_str(&args.to_currency).unwrap();

    let exchange_rate = get_exchange_rate(&from_currency, &to_currency);
    let to_currency_amount = exchange_rate * args.currency_amount;

    println!(
        "Converted {} {:?} to {} {:?} for an exchange rate of {}",
        args.currency_amount, from_currency, to_currency_amount, to_currency, exchange_rate
    )
}

#[test]
fn get_exchange_rate_of_euro_and_dollar() {
    let from_currency = Currency::EURO;
    let to_currency = Currency::DOLLAR;
    let expected = 1.1;

    let result = get_exchange_rate(&from_currency, &to_currency);

    assert_eq!(expected, result);
}
#[test]
fn get_exchange_rate_of_dollar_and_euro() {
    let from_currency = Currency::DOLLAR;
    let to_currency = Currency::EURO;
    let expected = 1.0 / 1.1;

    let result = get_exchange_rate(&from_currency, &to_currency);

    assert_eq!(expected, result);
}

#[test]
fn get_exchange_rate_of_dollar_and_pound() {
    let from_currency = Currency::DOLLAR;
    let to_currency = Currency::POUND;
    let expected = 0.9 / 1.1;

    let result = get_exchange_rate(&from_currency, &to_currency);

    assert_eq!(expected, result);
}
