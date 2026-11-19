/*
    attacks.rs
    author: xeno (Xavier T.)
    
    This file handles finding attacks
*/
use crate::board::{BISHOP, BLACK, Bitboard, Board, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE, board_cell_mask, board_has};

//these attack const functions precalculate every possible move/attack value ahead of time
//to make move generation really fast at runtime
pub(crate) const PAWN_ATKS: [[Bitboard; 64]; 2] = PAWN_ATK();
pub(crate) const KING_ATKS: [Bitboard; 64] = KING_ATK();
pub(crate) const KNIGHT_ATKS: [Bitboard; 64] = KNIGHT_ATK();

//just so its o(1) during runtime
pub(crate) const BETWEEN: [[Bitboard; 64]; 64] = gen_between();

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

//simple function that pulls every cell in between two. This gets cached so its never run during runtime
const fn gen_between() -> [[Bitboard; 64];64] {
    let mut out = [[0u64; 64];64];

    let mut cell = 0;


    while cell < 64 {
        let mut cell2 = cell + 1;
        while cell2 < 64 {
            let x1 = (cell % 8) as i32; let x2 = (cell2 % 8) as i32;
            let y1 = (cell / 8) as i32; let y2 = (cell2 / 8) as i32;

            let dx = x2-x1; let dy=y2-y1;

            if dx == 0 || dy == 0 || dx.abs() == dy.abs() {
                let dirx = dx.signum(); let diry = dy.signum();

                let mut crossx = x1 + dirx; let mut crossy = y1 + diry;

                while crossx != x2 || crossy != y2 {
                    out[cell][cell2] |= 1u64 << (crossy*8+crossx);
                    out[cell2][cell] |= 1u64 << (crossy*8+crossx);
                    crossx += dirx; crossy += diry;
                }
            }
            cell2 += 1;
        }
        cell += 1;
    }

    out
}

pub(crate) fn attacked_by(board: &Board, color: usize, all: Bitboard) -> Bitboard {
    let mut out = 0;

    //source: https://chessprogramming.org/Square_Mapping_Considerations
    //masks directly represent sections of board
    let a_file = 0x0101010101010101 as u64;
    let h_file = 0x8080808080808080 as u64;

    if color == WHITE { //uses bit shifts to quickly calculate all pawn attacks in one go
        out |= (board.boards[WHITE][PAWN] & !a_file) >> 9; //up left
        out |= (board.boards[WHITE][PAWN] & !h_file) >> 7; //up right
    }else{
        out |= (board.boards[BLACK][PAWN] & !a_file) << 7; //up left
        out |= (board.boards[BLACK][PAWN] & !h_file) << 9; //up right
    }

    let mut diag = board.boards[color][QUEEN] | board.boards[color][BISHOP];
    let mut cart = board.boards[color][QUEEN] | board.boards[color][ROOK];

    while diag != 0 { //trailing_zeros is much faster than looping
        //simply calculates the bishop attacks for each bishop/queen
        let cell = diag.trailing_zeros(); //grabs lowest bit
        out |= BISHOP_ATK(cell, all);
        diag &= diag - 1; //sets lowest bit to 0
    }
    while cart != 0 { //same as above, but for rooks/queens
        let cell = cart.trailing_zeros();
        out |= ROOK_ATK(cell, all);
        cart &= cart - 1;
    }

    let mut knights = board.boards[color][KNIGHT];
    while knights != 0 { //same as above but for knights
        let cell = knights.trailing_zeros() as usize;
        out |= KNIGHT_ATKS[cell];
        knights &= knights - 1;
    }
    let mut kings = board.boards[color][KING];
    while kings != 0 { //same as above but for knights
        let cell = kings.trailing_zeros() as usize;
        out |= KING_ATKS[cell];
        kings &= kings - 1;
    }
    out
}

pub(crate) fn attackers(board: &Board, color: usize, all: Bitboard, cell: u32) -> Bitboard {
    let mut out = 0;
    let c: usize = cell as usize;
    out |= KING_ATKS[c] & board.boards[color][KING];
    out |= KNIGHT_ATKS[c] & board.boards[color][KNIGHT];
    out |= PAWN_ATKS[1-color][c] & board.boards[color][PAWN];
    out |= ROOK_ATK(cell, all) & (board.boards[color][ROOK] | board.boards[color][QUEEN]);
    out |= BISHOP_ATK(cell, all) & (board.boards[color][BISHOP] | board.boards[color][QUEEN]);
    out
}