use std::net::TcpListener;

pub fn find_first_port(ports: Vec<&str>, host: &str) -> String {
    let mut result = String::new();

    for (i, p) in ports.iter().enumerate() {
        let address = format!("{}:{}", host, p);
        let listener = TcpListener::bind(&address);

        match listener {
            Ok(l) => result = l.local_addr().unwrap().port().to_string(),
            Err(e) => {
                if i == ports.len() - 1 {
                    result = format!("Could not create TCP listener on {}: {}", address, e)
                } else {
                    continue;
                }
            }
        };
    }

    return result;
}
