#[macro_use]
extern crate clap;
use clap::App;

const BEER: &str = "";
const DEFAULT_BEER_COUNT: u8 = 3;
const BEER_ASCII: &str = "
    _.._..,_,_
   (          )
    ]~,\"-.-~~[
  .=])' (;  ([
  | ]:: '    [
  '=]): .)  ([
    |:: '    |
     ~~----~~

";
const BEER_BOTTLED: &str = "
      ░░████████░░          ░░████████░░          ░░████████░░        
      ██▒▒░░▒▒▒▒██          ██▒▒░░▒▒▒▒██          ██▒▒░░▒▒▓▓██        
        ████████              ████████              ████████          
        ██    ██              ██    ██              ██    ██          
        ██    ██              ██    ██              ██    ██          
      ░░██    ██░░          ░░██    ██░░            ██    ██░░        
      ░░▒▒    ▒▒░░          ░░▒▒    ▒▒░░          ░░▒▒    ▓▓░░        
      ▓▓░░    ░░▓▓          ▓▓░░    ░░▓▓          ▓▓░░    ░░▓▓        
      ██░░░░░░░░██          ██░░░░░░░░██          ██░░░░░░░░██        
      ██░░  ░░░░██          ██░░  ░░░░██          ██░░  ░░░░██        
      ██░░  ░░░░██          ██░░  ░░░░██          ██░░  ░░░░██        
      ██░░  ░░░░██          ██░░  ░░░░██          ██░░  ░░░░██        
    ░░▓▓▒▒░░░░▒▒▓▓░░      ░░▓▓▒▒░░░░▒▒▓▓░░      ░░▓▓▒▒░░░░▒▒▓▓░░      
  ░░██░░  ░░░░░░░░██    ░░██░░  ░░░░░░░░██    ░░██░░  ░░░░░░░░██      
  ██░░  ░░░░░░░░░░░░██  ██░░  ░░░░░░░░░░░░██  ██░░  ░░░░░░░░░░░░██    
  ██░░  ░░░░░░░░░░░░██  ██░░  ░░░░░░░░░░░░██  ██░░  ░░░░░░░░░░░░██    
  ██░░  ░░░░░░▒▒▒▒▓▓██  ██░░░░░░░░░░▒▒▒▒▓▓██  ██░░░░░░░░░░▓▓▓▓▓▓██    
  ██░░  ░░░░░░▓▓░░░░██  ██░░░░░░░░░░▓▓░░░░██  ██░░░░░░░░░░▓▓░░░░██    
  ██░░  ░░░░░░▓▓░░░░██  ██░░  ░░░░░░▓▓░░░░██  ██░░  ░░░░░░▓▓░░░░██    
  ██░░  ░░░░░░▓▓░░░░██  ██░░  ░░░░░░▓▓░░░░██  ██░░  ░░░░░░▓▓░░░░██    
  ██░░  ░░░░░░▓▓░░░░██  ██░░  ░░░░░░▓▓░░░░██  ██░░  ░░░░░░▓▓░░░░██    
  ██░░  ░░░░░░▓▓░░░░██  ██░░  ░░░░░░▓▓░░░░██  ██░░  ░░░░░░▓▓░░░░██    
  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██    
  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██    
  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██    
  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██    
  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██    
  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██  ██░░  ░░░░░░▓▓    ██    
  ██░░  ░░░░░░▓▓▓▓▓▓██  ██░░  ░░░░░░▓▓▓▓▓▓██  ██░░  ░░░░░░▓▓▓▓▓▓██    
  ██░░░░░░░░░░░░░░░░██  ██░░░░░░░░░░░░░░░░██  ██░░░░░░░░░░░░░░░░██    
  ▓▓░░░░░░░░░░░░░░░░▓▓  ▓▓░░░░░░░░░░░░░░░░▓▓  ▓▓░░░░░░░░░░░░░░░░▓▓    
  ░░▓▓████████████▓▓░░░░░░▓▓████████████▓▓░░░░░░▓▓████████████▓▓░░░░░░
                                                                      
                                                                      
";

fn main() {
    let yaml = load_yaml!("../assets/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let ns_opt = matches.value_of("namespace");
    let prefix = ns_opt.map(|ns| {
        format!("Running in namespace {}:\n", ns)
    });

    let mut beer_count = DEFAULT_BEER_COUNT;
    if let ("get", Some(opts)) = matches.subcommand() {
        beer_count = opts.value_of("COUNT")
            .unwrap_or(&DEFAULT_BEER_COUNT.to_string())
            .parse::<u8>()
            .unwrap_or(DEFAULT_BEER_COUNT);
    }
    if matches.is_present("bottled") {
        print_bottled_beer(prefix.as_deref());
    } else if matches.is_present("ascii") {
        print_ascii_beer(beer_count, prefix.as_deref());
    } else {
        print_beers(beer_count, prefix.as_deref());
    }
}

fn print_beers(count: u8, prefix: Option<&str>) {
    let real_prefix = prefix.unwrap_or("");
    let beers = BEER.repeat(count as usize);
    println!("{}{}", real_prefix, beers);
}

fn print_ascii_beer(count: u8, prefix: Option<&str>) {
    let real_prefix = prefix.unwrap_or("");
    let beers = BEER_ASCII.repeat(count as usize);
    println!("{}{}", real_prefix, beers);
}

fn print_bottled_beer(prefix: Option<&str>) {
    let real_prefix = prefix.unwrap_or("");
    println!("{}{}", real_prefix, BEER_BOTTLED);
}
