//Test the send_log and log_component features of lab 1

use send_log::{log_connect, log_send, log_disconnect};
use std::env;
use std::{thread, time};
use rand::Rng;

fn main(){

	let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run <num_connections> <log_messages>");
        return;
    }
    let connections = args[1].parse::<i32>().unwrap();
    let num_messages = args[2].parse::<i32>().unwrap();

    let mut children = vec![];
    
	for i in 0..connections {
		children.push(
            thread::spawn( move || {thread_body(i, num_messages);} )
            );
	}

    for child in children {
        let _ = child.join();
    }

}


fn thread_body( id: i32, num_messages: i32 ){

    let current_dir = std::env::current_dir().unwrap();
    let config_path_buf = current_dir.join("../config.toml");
    let config_path = config_path_buf.to_str().unwrap();

    //Connect to the logging server
    let mut log = log_connect(config_path);
    let mut rng = rand::thread_rng();

    for i in 0..num_messages{
        //Pick a random duration between 50 and 100 milliseconds
        let delay = rng.gen_range(10..50);
        let duration = time::Duration::from_millis(delay);
        let message = format!("Message {i} from sender {id}\n");
        
        thread::sleep( duration );

        println!("{id} sending \"{message}\"");
        log_send( &mut log, &message );
    }

    log_disconnect( &mut log );
}

