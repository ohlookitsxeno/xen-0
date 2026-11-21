/*
    search.rs
    author: xeno (Xavier T.)

    This file handles the management and logic of searching
*/

use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::{board::{self, BLACK, Board, Move, WHITE, apply_move}, evaluation::evaluate, moves::legal_moves};

pub(crate) struct SearchLimits {
    pub(crate) movetime: Option<u32>,
    pub(crate) timeremaining: Option<u32>,
    pub(crate) depth: Option<u32>
}

pub(crate) enum SearchCommand {
    Go(Board, SearchLimits), //need to figure out what parameters I need
    Ponderhit(),
    Stop()
}

pub(crate) fn passive(rx: Receiver<SearchCommand>, tx: Sender<Option<Move>>) {
    loop {
        match rx.recv(){
        Ok(msg) => {
            match msg {
                //recieved board from main with limits
                SearchCommand::Go(b, lim) => {
                    let bestmove = search(b, lim.depth.unwrap_or(2) as usize);
                    // send bestmove back?
                    let _ = tx.send(bestmove);
                },
                SearchCommand::Ponderhit() =>{},
                SearchCommand::Stop() => {},
            }
        }
        Err(_) => {}
        }
    }
}

// for now just chooses a random move when it can
pub(crate) fn search(board: Board, depth: usize) -> Option<Move> {
    let moves = legal_moves(&board);

    if moves.is_empty(){
        return None;
    }

    let mut out = moves[0];

    

    //iterative deepening to target depth
    for d in 1..=depth {
        let mut bestscore = if board.next == WHITE {-1000000} else {1000000};
        for &m in &moves {
            let mut clone = board.clone();
            let _ = apply_move(&mut clone, m);

            let score = recsearch(clone, d-1);

            if board.next == WHITE && score > bestscore {bestscore = score; out = m;}
            if board.next == BLACK && score < bestscore {bestscore = score; out = m;}
        }
    }


    Some(out)
}


// simple minimax implementation
fn recsearch(board: Board, depth: usize) -> i32 {
    let moves = legal_moves(&board);

    if moves.is_empty() || depth == 0 {
        return evaluate(&board, &moves);
    }

    let mut out = if board.next == WHITE {-1000000} else {1000000};

    for m in moves {
        let mut clone = board.clone();
        let _ = apply_move(&mut clone, m);

        let eval = recsearch(clone, depth - 1);

        if board.next == WHITE && eval > out {out = eval;}
        if board.next == BLACK && eval < out {out = eval;}
    }
    out
}

/* 
pub(crate) fn passive(hold: bool, rx: std::sync::mpsc::Receiver<SearchCommand>) -> Option<()> { //need to add status
    // passive is the main worker/coordinator, waits for recv when idle
    // when hold is true (no searching), waits for input before calling active. returns nothing, or move when stopped
    // when hold is false (called from active), quickly checks for state update. returns status updat

    // hold = true, waits

    if hold {
        match rx.recv(){
        Ok(msg) => {
            match msg {
                //recieved board from main with limits
                SearchCommand::Go(b, lim) => {},
                SearchCommand::Ponderhit() =>{},
                SearchCommand::Stop() => {},
            }
        }
        Err(_) => {}
        }
    }else{
        match rx.try_recv(){
            Ok(msg) => {
                match msg {
                    //recieved board from main with limits
                    SearchCommand::Go(b, lim) => {},
                    SearchCommand::Ponderhit() =>{},
                    SearchCommand::Stop() => {},
                }
            },
            Err(TryRecvError::Empty) => {return Some(());},
            Err(TryRecvError::Disconnected) => {},
        }
    }
    
    
}

fn active(board: Board, maxdepth: usize){ //implementation of minimax
    //every n operations call the passive thread for updates
    let moves = legal_moves(&board);


}*/