/*
    attacks.rs
    author: xeno (Xavier T.)
    
    This file handles finding attacks
*/

use crate::board::{BLACK, Bitboard, WHITE, board_cell_mask, board_has};

//these attack const functions precalculate every possible move/attack value ahead of time
//to make move generation really fast at runtime

pub(crate) const PAWN_ATKS: [[Bitboard; 64]; 2] = PAWN_ATK();
pub(crate) const KING_ATKS: [Bitboard; 64] = KING_ATK();
pub(crate) const KNIGHT_ATKS: [Bitboard; 64] = KNIGHT_ATK();

//pawn attacks!
const fn PAWN_ATK() -> [[Bitboard; 64]; 2] {
    let mut out = [[0u64; 64];2];
    let mut cell = 0;
    
    while cell < 64 {
        let x = (cell % 8);
        let y = (cell/8);

        if y > 0 {
            let dy = y - 1;
            if x > 0 {out[WHITE][cell] |= 1u64 << (dy*8+x-1);}
            if x < 7 {out[WHITE][cell] |= 1u64 << (dy*8+x+1);}
        }
        if y < 7 {
            let dy = y + 1;
            if x > 0 {out[BLACK][cell] |= 1u64 << (dy*8+x-1);}
            if x < 7 {out[BLACK][cell] |= 1u64 << (dy*8+x+1);}
        }
        cell += 1;
    }
    out
}

const fn KING_ATK() -> [Bitboard; 64] {
    let mut out = [0u64; 64];
    let mut cell = 0;
    
    while cell < 64 {
        let x = (cell % 8) as i32;
        let y = (cell/8) as i32;

        let mut dx = x + 1; let mut dy = y;
        if dx < 8 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x - 1; dy = y;
        if dx >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x; dy = y + 1;
        if dy < 8 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x; dy = y - 1;
        if dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x + 1; dy = y + 1;
        if dx < 8 && dy < 8 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x - 1; dy = y - 1;
        if dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x + 1; dy = y - 1;
        if dx < 8 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x - 1; dy = y + 1;
        if dx >= 0 && dy < 8 {out[cell] |= 1u64 << (dy*8+dx);}

        cell += 1;
    }
    out
}

const fn KNIGHT_ATK() -> [Bitboard; 64] {
    let mut out = [0u64; 64];
    let mut cell = 0;
    
    while cell < 64 {
        let x = (cell % 8) as i32;
        let y = (cell/8) as i32;

        let mut dx = x + 1; let mut dy = y + 2;
        if dx < 8 && dy < 8 && dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x - 1;
        if dx < 8 && dy < 8 && dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dy = y - 2;
        if dx < 8 && dy < 8 && dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x + 1;
        if dx < 8 && dy < 8 && dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x + 2; dy = y+1;
        if dx < 8 && dy < 8 && dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dy = y - 1;
        if dx < 8 && dy < 8 && dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dx = x - 2;
        if dx < 8 && dy < 8 && dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}
        dy = y + 1;
        if dx < 8 && dy < 8 && dx >= 0 && dy >= 0 {out[cell] |= 1u64 << (dy*8+dx);}

        cell += 1;
    }
    out
}

pub(crate) fn ROOK_ATK(cell: u32, all: Bitboard) -> Bitboard {
    let mut out = 0;
    let x=(cell%8) as i32; let y = (cell/8) as i32;

    let mut dy = (y + 1) as i32;
    while dy < 8 {
        out |= 1u64 << (dy*8+x);
        if board_has(all, (dy*8+x) as u32) {break;}
        dy += 1;
    }
    dy = (y-1) as i32;
    while dy >= 0 {
        out |= 1u64 << (dy*8+x);
        if board_has(all, (dy*8+x)as u32) {break;}
        dy -= 1;
    }
    let mut dx = (x + 1) as i32;
    while dx < 8 {
        out |= 1u64 << (y*8+dx);
        if board_has(all, (y*8+dx) as u32) {break;}
        dx += 1;
    }
    dx = (x - 1) as i32;
    while dx >= 0 {
        out |= 1u64 << (y*8+dx);
        if board_has(all, (y*8+dx) as u32) {break;}
        dx -= 1;
    }
    out
}

pub(crate) fn BISHOP_ATK(cell: u32, all: Bitboard) -> Bitboard {
    let mut out = 0;
    let x=(cell%8) as i32; let y = (cell/8) as i32;

    let mut dy = (y + 1) as i32; let mut dx = (x+1) as i32;
    while dy < 8 && dx < 8 {
        out |= 1u64 << (dy*8+dx);
        if board_has(all, (dy*8+dx) as u32) {break;}
        dy += 1; dx+=1;
    }
    dy = (y - 1) as i32; dx = (x-1) as i32;
    while dy >=0 && dx >=0 {
        out |= 1u64 << (dy*8+dx);
        if board_has(all, (dy*8+dx) as u32) {break;}
        dy -= 1; dx-=1;
    }
    dy = (y + 1) as i32; dx = (x-1) as i32;
    while dy <8 && dx >=0 {
        out |= 1u64 << (dy*8+dx);
        if board_has(all, (dy*8+dx) as u32) {break;}
        dy += 1; dx-=1;
    }
    dy = (y - 1) as i32; dx = (x+1) as i32;
    while dy >= 0 && dx < 8 {
        out |= 1u64 << (dy*8+dx);
        if board_has(all, (dy*8+dx) as u32) {break;}
        dy -= 1; dx+=1;
    }

    out
}

pub(crate) fn QUEEN_ATK(cell: u32, all: Bitboard) -> Bitboard {
    ROOK_ATK(cell, all) | BISHOP_ATK(cell, all)
}