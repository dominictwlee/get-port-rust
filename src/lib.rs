use std::io::{Error, ErrorKind};
use std::net::TcpListener;

pub fn find_first_port(ports: Vec<&str>, host: &str) -> Result<String, Error> {
    let mut result = String::new();
    let mut err = Error::new(ErrorKind::Other, "Could not find any open ports");

    for p in ports.iter() {
        let address = format!("{}:{}", host, p);
        let listener = TcpListener::bind(&address);

        match listener {
            Ok(l) => {
                result = l.local_addr().unwrap().port().to_string();
                break;
            }
            Err(e) => {
                err = e;
            }
        };
    }

    if result.is_empty() {
        Err(err)
    } else {
        Ok(result)
    }
}
