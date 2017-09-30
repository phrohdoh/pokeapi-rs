#[macro_use(value_t)]
extern crate clap;
extern crate pokeapi;

use clap::{Arg, ArgGroup, App};

use pokeapi::errors::{self, ResultExt};
use pokeapi::{Pokemon, QueryType};

fn main() {
    let matches = App::new("pokecli")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Displays information about Pokemon creatures")
        .arg(Arg::with_name("name")
             .long("name")
             .value_name("NAME")
             .help("Get information for Pokemon named NAME")
             .takes_value(true))
        .arg(Arg::with_name("id")
             .long("id")
             .value_name("ID")
             .help("Get information for Pokemon with id ID")
             .takes_value(true))
        .group(ArgGroup::with_name("req_args")
              .arg("id")
              .arg("name")
              .required(true))
        .get_matches();

    if let Err(ref e) = run(matches) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "Error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "Caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated.
        // Run this with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "{:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run(args: clap::ArgMatches) -> errors::Result<()> {
    let query_type = if args.is_present("id") {
        let id = value_t!(args, "id", u32).chain_err(|| errors::ErrorKind::InvalidArgument("'id' should be an integer in the range 1 to 802".into()))?;
        QueryType::Id(id)
    } else if let Some(name) = args.value_of("name") {
        QueryType::Name(name)
    } else {
        unreachable!()
    };

    match Pokemon::get(query_type) {
        Ok(pokemon) => println!("{:?}", pokemon),
        Err(e) => return Err(e),
    }

    Ok(())
}