/*
    moves.rs
    author: xeno (Xavier T.)
    
    This file handles the generation of legal moves
*/

use crate::{attacks::{BETWEEN, BISHOP_ATK, KING_ATKS, KNIGHT_ATKS, PAWN_ATKS, QUEEN_ATK, ROOK_ATK, attacked_by, attackers}, board::{BISHOP, BLACK, Bitboard, Board, KING, KNIGHT, Move, QUEEN, ROOK, WHITE, PAWN, board_cell_mask, color_board}};


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
    let ctx = get_legal_context(board);
    let mut out = Vec::new();

    // The below section handles king moves
    let mut kingmoves = KING_ATKS[ctx.kingcell as usize] & !ctx.attacked & !ctx.me; 

    while kingmoves != 0 { //same method used in attacks/attacked_by, look there for info
        let cell = kingmoves.trailing_zeros() as usize;
        out.push(Move {from: ctx.kingcell, to: cell as u32, promotion: None});
        kingmoves &= kingmoves - 1;
    }

    if ctx.check_mask == 0 {return out;} //early return when double checked (or further cooked)

    let mut knights = board.boards[board.next][KNIGHT];
    let mut queens = board.boards[board.next][QUEEN];
    let mut bishops = board.boards[board.next][BISHOP];
    let mut rooks = board.boards[board.next][ROOK];
    let mut pawns = board.boards[board.next][PAWN];

    while knights != 0 {
        let knight = knights.trailing_zeros() as usize;
        let mut moves = KNIGHT_ATKS[knight] & !ctx.me &  & ctx.pin_masks[knight] & ctx.check_mask;

        while moves != 0 {
            let cell = moves.trailing_zeros();
            out.push(Move {from: knight as u32, to: cell, promotion: None });
            moves &= moves - 1;
        }
        knights &= knights - 1;
    }
    while queens != 0 {
        let queen = queens.trailing_zeros();
        let mut moves = QUEEN_ATK(queen, ctx.all) & !ctx.me &  & ctx.pin_masks[queen as usize] & ctx.check_mask;

        while moves != 0 {
            let cell = moves.trailing_zeros();
            out.push(Move {from: queen as u32, to: cell, promotion: None });
            moves &= moves - 1;
        }
        queens &= queens - 1;
    }
    while bishops != 0 {
        let bishop = bishops.trailing_zeros();
        let mut moves = BISHOP_ATK(bishop, ctx.all) & !ctx.me &  & ctx.pin_masks[bishop as usize] & ctx.check_mask;

        while moves != 0 {
            let cell = moves.trailing_zeros();
            out.push(Move {from: bishop as u32, to: cell, promotion: None });
            moves &= moves - 1;
        }
        bishops &= bishops - 1;
    }
    while rooks != 0 {
        let rook = rooks.trailing_zeros();
        let mut moves = ROOK_ATK(rook, ctx.all) & !ctx.me &  & ctx.pin_masks[rook as usize] & ctx.check_mask;

        while moves != 0 {
            let cell = moves.trailing_zeros();
            out.push(Move {from: rook as u32, to: cell, promotion: None });
            moves &= moves - 1;
        }
        rooks &= rooks - 1;
    }

    while pawns != 0 { //pawns have to be handled differently
        let pawn = pawns.trailing_zeros();
        let mut moves = PAWN_ATKS[board.next][pawn as usize] & ctx.enemy;
        if board.next == WHITE {
            if ctx.all & board_cell_mask(pawn) >> 8 == 0 {
                moves |= board_cell_mask(pawn) >> 8;
                if ctx.all & board_cell_mask(pawn) >> 16 == 0 && (pawn/8 == 6) {
                    moves |= board_cell_mask(pawn) >> 16;
                } 
            }
        }
        if board.next == BLACK {
            if ctx.all & board_cell_mask(pawn) << 8 == 0 {
                moves |= board_cell_mask(pawn) << 8;
                if ctx.all & board_cell_mask(pawn) << 16 == 0 && (pawn/8 == 1) {
                    moves |= board_cell_mask(pawn) << 16;
                } 
            }
        }
        moves = moves & !ctx.me &  & ctx.pin_masks[pawn as usize] & ctx.check_mask;
        while moves != 0 {
            let cell = moves.trailing_zeros();
            if (board.next == WHITE && cell/8 == 0) || (board.next == BLACK && cell/8 == 7) {
                out.push(Move {from: pawn as u32, to: cell, promotion: Some(QUEEN) });
                out.push(Move {from: pawn as u32, to: cell, promotion: Some(ROOK) });
                out.push(Move {from: pawn as u32, to: cell, promotion: Some(BISHOP) });
                out.push(Move {from: pawn as u32, to: cell, promotion: Some(KNIGHT) });
            } else {
                out.push(Move {from: pawn as u32, to: cell, promotion: None });
            }
            
            moves &= moves - 1;
        }
        pawns &= pawns - 1;
    }
     
    out
}