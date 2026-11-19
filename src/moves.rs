/*
    moves.rs
    author: xeno (Xavier T.)
    
    This file handles the generation of legal moves
*/

use crate::board::{Bitboard, Board, KING, Move, color_board};


// tied to a board, this describes everything necessary to directly calculate valid moves
struct legalcontext {
    kingcell: u32,
    checkers: Bitboard,
    check_mask: Bitboard,
    pin_masks: [Bitboard; 64],
    me: Bitboard,
    enemy: Bitboard,
    all: Bitboard,
    attacked: Bitboard,
}

// this slowly builds legalcontext given a board
fn get_legal_context(board: &Board) -> legalcontext {
    let ctxking = board.boards[board.next][KING].trailing_zeros();

    let ctxme = color_board(board, board.next);
    let ctxenemy = color_board(board, 1-board.next);
    let ctxall = ctxme | ctxenemy;

}

fn legal_moves(board: &Board) -> Vec<Move> {
    let legalctx = get_legal_context(board);
}