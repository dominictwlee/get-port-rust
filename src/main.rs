extern crate clap;

use clap::{App, Arg};

use get_port_rust::find_first_port;

fn main() {
    let matches = App::new("Get Port")
        .version("0.1.0")
        .author("Dominic Lee <dominictwlee@gmail.com>")
        .about("Get an available port")
        .arg(
            Arg::with_name("ports")
                .min_values(1)
                .default_values(&["0"])
                .about("Preferred ports"),
        )
        .arg(
            Arg::with_name("host")
                .short('h')
                .long("host")
                .default_value("127.0.0.1")
                .number_of_values(1)
                .about("The host on which port resolution should be performed. Can be either an IPv4 or IPv6 address."),
        )
        .get_matches();

    let preferred_ports: Vec<&str> = matches.values_of("ports").unwrap().collect();
    let host = matches.value_of("host").unwrap();

    let result = find_first_port(preferred_ports, host);

    println!("{}", result);
}
