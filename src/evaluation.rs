/*
    evaluation.rs
    author: xeno (Xavier T.)

    This file handles evaluating how good a position is.
*/

use crate::{attacks::{attackers}, board::{BLACK, WHITE, Board, KING, Move, color_board}};

//always uses the same standard regardless of side.
//+ is white, - is black.
pub(crate) fn evaluate(board: &Board, moves: &[Move]) -> i32 {
    if moves.is_empty() {
        if attackers(board, 1-board.next, color_board(board, WHITE) | color_board(board, BLACK), board.boards[board.next][KING].trailing_zeros()) != 0 {
            let matescore = 100000;
            if board.next == WHITE {return -matescore;} else {return matescore;}
        }
        return 0;
    }
    let mut out: i32 = 0;
    let pvalues = [100,325,350,500,1000,0];

    for i in 0..6 {
        out += (board.boards[WHITE][i].count_ones() as i32) * pvalues[i];
        out -= (board.boards[BLACK][i].count_ones() as i32) * pvalues[i];
    }
    out
}