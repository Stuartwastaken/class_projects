use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use config::{Config, File};

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    //GRADING NOTE: This config file path does not work
    let config_path_buf = current_dir.join("../config.toml");
    let config_path = config_path_buf.to_str().unwrap();

    let settings = match Config::builder()
        .add_source(File::with_name(config_path))
        .build() {
            Ok(x) => x,
            Err(x) => panic!("Could not open configuration file: {}", x),
        };

    let log_ip = settings.get_string("log_ip").unwrap();
    let log_port = settings.get_int("log_port").unwrap();
    let log_file = settings.get_string("log_file").unwrap();

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .expect("Error opening log file");

    let listener = TcpListener::bind(format!("{}:{}", log_ip, log_port))
        .expect("Error binding to socket");

    let mut connections: Vec<TcpStream> = vec![];

    loop {
        // Accept new connections and add them to our list of connections
        if let Ok((stream, _)) = listener.accept() {
            println!("New connection from {}", stream.peer_addr().unwrap());
            stream.set_nonblocking(true).expect("Error setting non-blocking mode");
            connections.push(stream);
        }

        // Read data from all connected clients
        for mut stream in connections.iter_mut() {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    let message = std::str::from_utf8(&buffer[..n]).expect("Invalid UTF-8");
                    println!("Received message: {}", message);
                    log_file.write_all(message.as_bytes()).expect("Error writing to log file");
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    println!("Error reading from stream: {}", e);
                    panic!("Error reading from stream");
                }
                _ => {}
            }
        }

        // Sleep for a short period to avoid busy waiting
        thread::sleep(Duration::from_millis(100));
    }
}
