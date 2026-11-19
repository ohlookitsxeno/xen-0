/*
    moves.rs
    author: xeno (Xavier T.)
    
    This file handles the generation of legal moves
*/

use crate::{attacks::{BETWEEN, BISHOP_ATK, ROOK_ATK, attacked_by, attackers}, board::{BISHOP, Bitboard, Board, KING, Move, QUEEN, ROOK, board_cell_mask, color_board}};


// tied to a board, this describes everything necessary to directly calculate valid moves
struct legalcontext {
    kingcell: u32, //
    checkers: Bitboard, //
    check_mask: Bitboard, //
    pin_masks: [Bitboard; 64],
    me: Bitboard, //
    enemy: Bitboard, //
    all: Bitboard, //
    attacked: Bitboard, //
}

// this slowly builds legalcontext given a board
fn get_legal_context(board: &Board) -> legalcontext {
    let ctxking = board.boards[board.next][KING].trailing_zeros();

    let ctxme = color_board(board, board.next);
    let ctxenemy = color_board(board, 1-board.next);
    let ctxall = ctxme | ctxenemy;

    // attacked excludes king so rays (from bishops/rooks/queen) pass through it
    let ctxattacked = attacked_by(board, 1-board.next, ctxall ^ board_cell_mask(ctxking));
    let ctxcheckers = attackers(board, 1-board.next, ctxall, ctxking);

    let ctxcheck_mask = match ctxcheckers.count_ones() {
        1 => {ctxcheckers | BETWEEN[ctxking as usize][ctxcheckers.trailing_zeros() as usize]},
        0 => {!0u64}, //fun way to write 64 1's
        _ => {0}
    };

    //for notes on this pinmasking: https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)
    let mut ctxpinmasks = [!0u64; 64]; //1 for all non-pin cells

    // all enemy pieces that can pin the king
    let diag = board.boards[1-board.next][BISHOP] | board.boards[1-board.next][QUEEN];
    let cart = board.boards[1-board.next][ROOK] | board.boards[1-board.next][QUEEN];

    //filter by direction and pieces
    let mut pinners = ROOK_ATK(ctxking, ctxenemy) & cart;
    pinners |= BISHOP_ATK(ctxking, ctxenemy) & diag;

    while pinners != 0 { //same method used in attacks/attacked_by, look there for info
        let rayhit = pinners.trailing_zeros() as usize;
        let ray = BETWEEN[ctxking as usize][rayhit as usize];
        let blockers = ray & ctxall;
        //if only piece in ray is mine, its pinned
        if (blockers & ctxme != 0) && blockers.count_ones() == 1 {
            //includes enemy square in ray (it can be taken)
            ctxpinmasks[blockers.trailing_zeros() as usize] = ray | board_cell_mask(rayhit as u32);
        }
        pinners &= pinners - 1;
    }



    legalcontext { 
        kingcell: ctxking, checkers: ctxcheckers, check_mask: ctxcheck_mask, 
        pin_masks: ctxpinmasks, me: ctxme, enemy: ctxenemy, all: ctxall, attacked: ctxattacked }
}

fn legal_moves(board: &Board) -> Vec<Move> {
    let legalctx = get_legal_context(board);
}