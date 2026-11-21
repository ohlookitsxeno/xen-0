/*
    main.rs
    author: xeno (Xavier T.)

    This file handles the main uci interface between clients and the engine.
*/

//imports
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{TryRecvError, channel};
use std::thread;
use std::time::Duration;

use crate::board::move_uci;
use crate::search::SearchCommand::{Go, Ponderhit, Stop};
use crate::search::SearchLimits;
use crate::{search::{SearchCommand}};

//internal imports
mod board;
mod moves;
mod attacks;
mod evaluation;
mod search;

//the main starting FEN
const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


fn main() -> io::Result<()> {
    //the main loop that controls this entire program.
    //throws io into a background thread that reads and parses its commands to control searching, etc.

    //this allows sharing text between threads
    let (sender, receiver) = channel();
    let mut stdout = io::stdout().lock();

    //this allows communication between this and search thread
    let (searchtx, searchrx) = channel::<SearchCommand>();
    let (resulttx, resultrx) = channel::<Option<board::Move>>();
    

    let stopflag = Arc::new(AtomicBool::new(false));
    let workerstop = Arc::clone(&stopflag);

    let worker = std::thread::spawn(move || search::passive(searchrx, resulttx,workerstop));
    
    thread::spawn(move || {
        //the background thread that reads io and sends it to the main thread(s)
        for line in io::stdin().lines() {
            match line {
                Ok(msg) => {if sender.send(msg).is_err() {break;}}
                _ => {break;}
            }
        }
    });

    let mut mainboard = board::fen_to_board(STARTPOS).unwrap();
    let searching = false;
    let ponder = false;

    loop {

        //This recieves the text from the background thread, and parses it into cmd if it exists. 
        let cmd = match receiver.try_recv() {
            Ok(txt) => {
                let trimmed = txt.trim();
                if trimmed == "quit" {stopflag.store(true, Ordering::Relaxed); break;}
                Some(trimmed.to_string())
            }
            Err(TryRecvError::Empty) => {None}
            Err(TryRecvError::Disconnected) => {break;}
        };

         match resultrx.try_recv() {
            Ok(Some(mv)) => {
                writeln!(stdout, "bestmove {}", move_uci(mv))?;
                stdout.flush()?;
            }
            Ok(None) => {
                writeln!(stdout, "bestmove 0000")?;
                stdout.flush()?;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {break;}
        }

        //if valid command, goes into main command match loop
        match cmd {
            Some(text) => {
                let mut args = text.split_whitespace();
                match args.next() {
                    //the Main UCI commands and their actions go here!
                    Some("go") => {
                        //send signal to search thread
                        stopflag.store(false, Ordering::Relaxed);
                        let mut limits = SearchLimits::default();
                        while let Some(arg) = args.next() {
                            match arg {
                                "depth" => {
                                    limits.depth = args.next().unwrap_or("6").parse::<u32>().ok();
                                },
                                "wtime" => {
                                    limits.wtime = args.next().unwrap_or("1000").parse::<i64>().ok();
                                },
                                "btime" => {
                                    limits.btime = args.next().unwrap_or("1000").parse::<i64>().ok();
                                },
                                "winc" => {
                                    limits.winc = args.next().unwrap_or("1000").parse::<u32>().ok();
                                },
                                "binc" => {
                                    limits.binc = args.next().unwrap_or("1000").parse::<u32>().ok();
                                },
                                "movestogo" => {
                                    limits.movestogo = args.next().unwrap_or("100").parse::<u32>().ok();
                                },
                                "movetime" => {
                                    limits.movetime = args.next().unwrap_or("1000").parse::<u32>().ok();
                                },
                                "maxnodes" => {
                                    limits.maxnodes = args.next().unwrap_or("1000000").parse::<u64>().ok();
                                },
                                "infinite" => {
                                    limits.infinite = true;
                                },
                                _ => {}
                            }
                        }
                        let _ = searchtx.send(Go(mainboard, limits));
                        
                    }
                    Some("ponderhit") => {
                        //send signal to search thread
                        let _ = searchtx.send(Ponderhit());
                    }
                    Some("stop") => {
                        //send signal to search thread to stop
                        stopflag.store(true, Ordering::Relaxed);
                        let _ = searchtx.send(Stop());
                    }
                    Some("setoption") => {}
                    Some("ucinewgame") => {
                        mainboard = board::fen_to_board(STARTPOS).unwrap();
                    }
                    Some("position") => {
                        let newboard = match args.next() {
                            Some("startpos") => {
                                Some(board::fen_to_board(STARTPOS))
                            },
                            Some("fen") => {
                                let fenargs: Vec<&str> = args.by_ref().take(6).collect();
                                let parsedfen = fenargs.join(" ");
                                Some(board::fen_to_board(&parsedfen))
                            },
                            _ => {None}
                        };
                        match newboard {
                            Some(Ok(mut board)) => {
                                match args.next() {
                                    Some("moves") => {
                                        let mut ok = true;
                                        for movetxt in args {
                                            match board::uci_move(movetxt).and_then(|mv| board::apply_move(&mut board, mv))
                                            {
                                                Ok(()) => {}
                                                Err(msg) => {
                                                    eprintln!("invalid move {movetxt}! {msg}");
                                                    ok = false;
                                                    break;
                                                }
                                            }
                                        }
                                        if ok {
                                            mainboard = board;
                                        }
                                    }
                                    None => {
                                        mainboard = board;
                                    }
                                    Some(_) => {
                                        eprintln!("invalid argument after position")
                                    }
                                }
                            },
                            Some(Err(msg)) => {eprintln!("invalid position: {msg}")}
                            None => {}
                        }
                    }
                    Some("isready") => {writeln!(stdout, "readyok")?;stdout.flush()?;}
                    Some("uci") => {
                        writeln!(stdout, "id name xen-0 v1.0")?;
                        writeln!(stdout, "id author xeno (Xavier T.)")?;
                        writeln!(stdout, "uciok")?;
                        stdout.flush()?;
                    }
                    _ => {}
                }
            }
            None => {}
        }
        //thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}