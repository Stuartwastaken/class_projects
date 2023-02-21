use std::net::{TcpStream, Shutdown};
use config::Config;
use std::process::exit;
use std::io::Write;
use rand::Rng;

/* send_log - writes messages to the global logger
 *
 * Usage: 1. connect to logger and create log object with log_connect
 *        2. send log messages with log_send
 *        3. terminate logger connection with log_disconnect
 *
 * For example, in user code:
 *
 *  let logger = log_connect( "../config.toml" );
 *  log_send( &logger, "This is a log message to write to the global log." );
 *  log_send( &logger, "And another message to write to the global log." );
 *  log_disconnect( &logger );
 * */

//This should be considered an opaque type- the user should never
//interact with the internals of struct Log. See the usage idiom
//up above.
pub struct Log {
    stream: TcpStream,
}

pub fn log_connect( config_path: &str ) -> Log {
    let settings = match Config::builder()
        .add_source(config::File::with_name(config_path))
        .build() {
            Ok(x) => x,
            Err(x) => {println!("Could not open configuration file: {x}"); exit(-1)},
        }; 

    let log_ip = settings.get_string("log_ip").unwrap();
    let log_port = settings.get_string("log_port").unwrap();

    let stream = match TcpStream::connect(format!("{}:{}", log_ip, log_port)) {
        Ok(s) => s,
        Err(e) => { println!("Error connecting to logging server: {}", e); exit(-1) }
    };

    Log { stream }
}

pub fn log_disconnect( log: &mut Log ){
    log.stream.shutdown(Shutdown::Both).unwrap();
}

pub fn log_send( log: &mut Log, msg: &str ){
    if let Err(e) = log.stream.write_all(msg.as_bytes()) {
        println!("Error sending message to logging server: {}", e);
        exit(-1);
    }
}

