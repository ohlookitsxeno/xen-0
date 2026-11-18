/*
    main.rs

    This file handles the main uci interface between clients and the engine.
*/

//imports
use std::io::{self};
use std::sync::mpsc::{TryRecvError, channel};
use std::thread;
use std::time::Duration;


fn main() -> io::Result<()> {
    //the main loop that controls this entire program.
    //throws io into a background thread that reads and parses its commands to control searching, etc.

    //this allows sharing text between threads
    let (sender, receiver) = channel();

    thread::spawn(move || {
        //the background thread that reads io and sends it to the main thread(s)
        for line in io::stdin().lines() {
            match line {
                Ok(msg) => {if sender.send(msg).is_err() {break;}}
                _ => {break;}
            }
        }
    });

    loop {

        //This recieves the text from the background thread, and parses it into cmd if it exists. 
        let cmd = match receiver.try_recv() {
            Ok(txt) => {
                let trimmed = txt.trim();
                if trimmed == "quit" {break;}
                Some(trimmed.to_string())
            }
            Err(TryRecvError::Empty) => {None}
            Err(TryRecvError::Disconnected) => {break;}
        };

        //if valid command, goes into main command match loop
        match cmd {
            Some(text) => {println!("{text}")}
            None => {}
        }
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}