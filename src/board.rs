/*
    board.rs
    author: xeno (Xavier T.)
    
    This file handles the main board and move representation.
*/

pub(crate) type Bitboard = u64;

#[derive(Clone, Copy)]
pub(crate) struct Board {
    pub(crate) boards: [[Bitboard; 6]; 2], //b.boards[color][piece]
    pub(crate) next: usize,

    pub(crate) castle_wk: bool,
    pub(crate) castle_wq: bool,
    pub(crate) castle_bk: bool,
    pub(crate) castle_bq: bool,

    pub(crate) enp: Option<u32>,
    pub(crate) halfmoves: u32,
    pub(crate) fullmoves: u32,
}

#[derive(Clone, Copy)]
pub(crate) struct Move {
    pub(crate) from: u32,
    pub(crate) to: u32,
    pub(crate) promotion: Option<usize>,
}

pub(crate) const PAWN: usize = 0;
pub(crate) const KNIGHT: usize = 1;
pub(crate) const BISHOP: usize = 2;
pub(crate) const ROOK: usize = 3;
pub(crate) const QUEEN: usize = 4;
pub(crate) const KING: usize = 5;
pub(crate) const WHITE: usize = 0;
pub(crate) const BLACK: usize = 1;

//cell/board functions
pub(crate) const fn board_cell_mask(cell: u32) -> Bitboard {
    1u64 << cell
}
pub(crate) fn board_add(bitboard: &mut Bitboard, cell: u32) {
    *bitboard |= board_cell_mask(cell)
}
pub(crate) fn board_rm(bitboard: &mut Bitboard, cell: u32) {
    *bitboard &= !board_cell_mask(cell)
}
pub(crate) fn board_has(bitboard: Bitboard, cell: u32) -> bool {
    bitboard & board_cell_mask(cell) != 0
}

pub(crate) fn color_board(board: &Board, color: usize) -> Bitboard {
    let mut out = 0;
    for i in 0..6 {
        out |= board.boards[color][i]
    }
    out
}

pub(crate) fn text_cell(text: &str) -> Result<u32, String> {
    let bytes = text.as_bytes();

    if bytes.len() != 2 {
        return Err(String::from("cell definitions are 2 characters long"));
    }

    let xtxt = bytes[0];
    let ytxt = bytes[1];

    if xtxt < b'a' || xtxt > b'h' {
        return Err(String::from("invalid cell file/col"));
    }
    if ytxt < b'1' || ytxt > b'8' {
        return Err(String::from("invalid cell rank/row"));
    }

    let y = (b'8' - ytxt) as u32;
    let x = (xtxt - b'a') as u32;

    Ok(y * 8 + x)
}

pub(crate) fn apply_move(board: &mut Board, mv: Move) -> Result<(), String> {
    let mut piecetype = None;
    for i in 0..6 {
        if board_has(board.boards[board.next][i], mv.from) {
            piecetype = Some(i);
            break;
        }
    }
    let piecetype = piecetype.ok_or_else(|| String::from("no piece on from square"))?;

    board_rm(&mut board.boards[board.next][piecetype], mv.from);
    let enemy = 1 - board.next;

    let mut captured = false;
    for i in 0..6 {
        if board_has(board.boards[enemy][i], mv.to) {
            board_rm(&mut board.boards[enemy][i], mv.to);
            captured = true;
            break;
        }
    }

    if piecetype == PAWN && board.enp == Some(mv.to) {
        let cap_sq = if board.next == WHITE {
            mv.to + 8
        } else {
            mv.to - 8
        };
        board_rm(&mut board.boards[enemy][PAWN], cap_sq);
    }

    let placed = mv.promotion.unwrap_or(piecetype);
    board_add(&mut board.boards[board.next][placed], mv.to);

    if piecetype == KING {
        if (mv.from as i32 - mv.to as i32).abs() == 2 {
            let (rook_from, rook_to) = match mv.to {
                62 => (63, 61),
                58 => (56, 59),
                6 => (7, 5),
                2 => (0, 3),
                _ => return Err(String::from("invalid castling destination")),
            };
            board_rm(&mut board.boards[board.next][ROOK], rook_from);
            board_add(&mut board.boards[board.next][ROOK], rook_to);
        }
        if board.next == WHITE {
            board.castle_wk = false;
            board.castle_wq = false;
        }
        if board.next == BLACK {
            board.castle_bk = false;
            board.castle_bq = false;
        }
    }

    if mv.to == 56 || mv.from == 56 {
        board.castle_wq = false;
    }
    if mv.to == 63 || mv.from == 63 {
        board.castle_wk = false;
    }
    if mv.to == 0 || mv.from == 0 {
        board.castle_bq = false;
    }
    if mv.to == 7 || mv.from == 7 {
        board.castle_bk = false;
    }

    board.enp = if piecetype == PAWN && (mv.from as i32 - mv.to as i32).abs() == 16 {
        Some((mv.from + mv.to) / 2)
    } else {
        None
    };

    board.halfmoves = if piecetype == PAWN || captured {
        0
    } else {
        board.halfmoves + 1
    };

    if board.next == BLACK {
        board.fullmoves += 1;
    }

    board.next = 1 - board.next;

    Ok(())
}


//example of the fen format: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
pub(crate) fn fen_to_board(fen: &str) -> Result<Board, String> {
    let args: Vec<&str> = fen.split_whitespace().collect();

    if args.len() != 6 {
        return Err(String::from("FEN format invalid (needs 6 fields)"));
    }

    let rows: Vec<&str> = args[0].split("/").collect();

    if rows.len() != 8 {
        return Err(String::from("Fen format invalid (needs 8 rows)"));
    }

    let mut out = Board {
        boards: [[0; 6]; 2],
        next: WHITE,
        castle_wk: false,
        castle_wq: false,
        castle_bk: false,
        castle_bq: false,
        enp: None,
        halfmoves: 0,
        fullmoves: 1,
    };

    //This parses the entire board layout itself (first field of FEN)
    for (y, row) in rows.iter().enumerate() {
        let mut colidx: u32 = 0;
        for c in row.chars() {
            match c {
                '1'..='8' => {
                    colidx += c.to_digit(10).unwrap();
                }
                _ => {
                    if colidx >= 8 {
                        return Err(String::from("FEN Invalid (each row needs 8 columns)"));
                    }
                    let piece = match c.to_ascii_lowercase() {
                        'p' => PAWN,
                        'n' => KNIGHT,
                        'b' => BISHOP,
                        'r' => ROOK,
                        'q' => QUEEN,
                        'k' => KING,
                        _ => return Err(String::from("Unknown FEN piece")),
                    };
                    let color = if c.is_ascii_lowercase() { BLACK } else { WHITE };
                    let cell = (y as u32) * 8 + colidx;
                    board_add(&mut out.boards[color][piece], cell);
                    colidx += 1;
                }
            }
        }
        if colidx != 8 {
            return Err(String::from("FEN Invalid (each row needs 8 columns)"));
        }
    }

    out.next = match args[1] {
        "w" => 0,
        "b" => 1,
        _ => return Err(String::from("FEN Invalid (who moves?)")),
    };
    if args[2] != "-" {
        for c in args[2].chars() {
            match c {
                'K' => {
                    if out.castle_wk {
                        return Err(String::from("Duplicate castling!"));
                    }
                    out.castle_wk = true;
                }
                'k' => {
                    if out.castle_bk {
                        return Err(String::from("Duplicate castling!"));
                    }
                    out.castle_bk = true;
                }
                'Q' => {
                    if out.castle_wq {
                        return Err(String::from("Duplicate castling!"));
                    }
                    out.castle_wq = true;
                }
                'q' => {
                    if out.castle_bq {
                        return Err(String::from("Duplicate castling!"));
                    }
                    out.castle_bq = true;
                }
                _ => return Err(String::from("invalid castling")),
            }
        }
    }

    out.enp = if args[3] == "-" {
        None
    } else {
        let cell = text_cell(args[3])?;
        let y = cell / 8;
        if y != 2 && y != 5 {
            return Err(String::from("invalid en passant"));
        };
        Some(cell)
    };

    out.halfmoves = match args[4].parse::<u32>() {
        Ok(number) => number,
        Err(_) => return Err(String::from("invalid halfmove count")),
    };

    out.fullmoves = match args[5].parse::<u32>() {
        Ok(number) => {
            if number == 0 {
                return Err(String::from("moves needs to be at least 1"));
            } else {
                number
            }
        }
        Err(_) => return Err(String::from("invalid fullmove count")),
    };

    Ok(out)
}

pub(crate) fn uci_move(movetxt: &str) -> Result<Move, String> {
    if !movetxt.is_ascii() || movetxt.len() < 4 || movetxt.len() > 5 {
        return Err(String::from("uci move needs to be ascii, 4-5 chars"));
    }

    let from = text_cell(&movetxt[0..2])?;
    let to = text_cell(&movetxt[2..4])?;
    let promotion = match movetxt.as_bytes().get(4) {
        None => None,
        Some(b'n') => Some(KNIGHT),
        Some(b'b') => Some(BISHOP),
        Some(b'r') => Some(ROOK),
        Some(b'q') => Some(QUEEN),
        Some(_) => return Err(String::from("invalid promotion")),
    };
    Ok(Move {
        from,
        to,
        promotion,
    })
}

pub(crate) fn move_uci(mv: Move) -> String {
    let mut out = String::with_capacity(5);

    out.push(char::from(b'a' + (mv.from % 8) as u8));
    out.push(char::from(b'8' - (mv.from /8) as u8));
    out.push(char::from(b'a' + (mv.to % 8) as u8));
    out.push(char::from(b'8' - (mv.to /8) as u8));
    
    match mv.promotion {
        None => {},
        Some(ROOK) => {out.push('r');},
        Some(QUEEN) => {out.push('q');},
        Some(KNIGHT) => {out.push('n');},
        Some(BISHOP) => {out.push('b');},
        Some(_) => {}
    };

    out
}