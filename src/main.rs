#[macro_use]
extern crate clap;
use clap::App;

const BEER: &str = "";
const DEFAULT_BEER_COUNT: u8 = 3;

fn main() {
    let yaml = load_yaml!("../assets/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let ns_opt = matches.value_of("namespace");
    let prefix = ns_opt.map(|ns| {
        ["Running in namespace ", ns, ":\n"].join("")
    });

    let mut beer_count = DEFAULT_BEER_COUNT;
    if let ("get", Some(opts)) = matches.subcommand() {
        beer_count = opts.value_of("COUNT")
            .unwrap_or(&DEFAULT_BEER_COUNT.to_string())
            .parse::<u8>()
            .unwrap_or(DEFAULT_BEER_COUNT);
    }
    print_beers(beer_count, prefix.as_deref());
}

fn print_beers(count: u8, prefix: Option<&str>) {
    let real_prefix = prefix.unwrap_or("");
    let beers = BEER.repeat(count as usize);
    println!("{}{}", real_prefix, beers);
}
