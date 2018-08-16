#[macro_use(value_t)]
extern crate clap;
extern crate bitcoin;
extern crate serde_json;
extern crate zcash;

use bitcoin::builder::{WalletBuilder as BitcoinWalletBuilder};
use zcash::builder::{WalletBuilder as ZcashWalletBuilder};
use clap::{App, Arg};

fn main() {
    let matches = App::new("wagen")
       .version("v0.1.0")
       .about("Generate a wallet for any cryptocurrency

Supported Currencies: Bitcoin, Zcash (t-address)")
       .author("Argus Observer <ali@argus.observer>")
       .arg(Arg::with_name("currency")
            .required(true)
            .help("Name of the currency to generate a wallet for (e.g. bitcoin)"))
        .arg(Arg::with_name("network")
            .short("N")
            .long("network")
            .takes_value(true)
            .help("Network of wallet(s) to generate (e.g. mainnet, testnet)"))
       .arg(Arg::with_name("count") 
            .short("n")
            .long("count")
            .takes_value(true)
            .help("Number of wallets to generate"))
        .arg(Arg::with_name("compressed")
            .short("c")
            .long("compressed")
            .help("Enabling this flag generates a wallet which corresponds to a compressed public key"))
        .arg(Arg::with_name("json")
            .short("j")
            .long("json")
            .help("Enabling this flag prints the wallet in JSON format"))
       .get_matches();

    let currency = matches.value_of("currency").unwrap();
    let compressed = matches.is_present("compressed");
    let json = matches.is_present("json");
    let count = value_t!(matches.value_of("count"), usize).unwrap_or_else(|_e| 1);
    let testnet = match matches.value_of("network") {
        Some("mainnet") => false,
        Some("Mainnet") => false,
        Some("livenet") => false,
        Some("testnet") => true,
        Some("Testnet") => true,
        _ => false,
    };

    match currency {
        "bitcoin" => print_bitcoin_wallet(count, testnet, compressed, json),
        "zcash" => print_zcash_wallet(count, testnet, compressed, json),
        _ => panic!("Unsupported currency"),
    };
}

fn print_bitcoin_wallet(count: usize, testnet: bool, compressed: bool, json: bool) {
    let wallets = BitcoinWalletBuilder::build_many_from_options(compressed, testnet, count);
    if json {
        println!("{}", serde_json::to_string_pretty(&wallets).unwrap())
    } else {
        wallets.iter().for_each(|wallet| println!("{}", wallet));
    }
}

fn print_zcash_wallet(count: usize, testnet: bool, compressed: bool, json: bool) {
    let wallets = ZcashWalletBuilder::build_many_from_options(compressed, testnet, count);
    if json {
        println!("{}", serde_json::to_string_pretty(&wallets).unwrap())
    } else {
        wallets.iter().for_each(|wallet| println!("{}", wallet));
    }
}
