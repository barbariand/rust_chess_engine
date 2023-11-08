use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let mut generated_str = String::from("pub static king_moves_bitmask: [u64; 64] = [");
    for i in 0..64 {
        //king
        generated_str += &format!("0b{:064b},", king_moves_bitmask(i));
    }
    generated_str += "];";
    generated_str += "pub static knight_moves_bitmask: [u64; 64] = [";
    for i in 0..64 {
        //king
        generated_str += &format!("0b{:064b},", knight_moves_bitmask(i));
    }
    generated_str += "];";
    generated_str += "pub static rook_moves_bitmask: [u64; 64] = [";
    for i in 0..64 {
        //king
        generated_str += &format!("0b{:064b},", rook_moves_bitmask(i));
    }
    generated_str += "];";
    generated_str += "pub static bishop_moves_bitmask: [u64; 64] = [";
    for i in 0..64 {
        //king
        generated_str += &format!("0b{:064b},", bishop_moves_bitmask(i));
    }
    generated_str += "];";
    generated_str += "pub static queen_moves_bitmask: [u64; 64] = [";
    for i in 0..64 {
        //king
        generated_str += &format!("0b{:064b},", queen_moves_bitmask(i));
    }
    generated_str += "];";
    generated_str += "pub static white_pawn_moves: [(u64,u64); 64] = [";
    for i in 0..56 {
        let a = white_pawn_moves(i);
        generated_str += &format!("(0b{:064b},0b{:064b}),", a.0, a.1);
    }
    for _ in 0..8 {
        generated_str += "(0,0),"
    }
    generated_str += "];";
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("{}", generated_str);
    let dest_path = Path::new(&out_dir).join("movetables.rs");
    fs::write(&dest_path, generated_str).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
fn king_moves_bitmask(pos: u8) -> u64 {
    if pos > 63 {
        panic!("Invalid position");
    }

    let mut moves: u64 = 0;

    // Move Right
    if pos % 8 != 7 {
        moves |= 1u64 << (pos + 1);
    }

    // Move Down
    if pos < 56 {
        moves |= 1u64 << (pos + 8);
    }

    // Move Down-Right
    if pos < 56 && pos % 8 != 7 {
        moves |= 1u64 << (pos + 9);
    }

    // Bit-reversal for 64 bits to make the top-left corner the least significant bit

    moves
}
fn knight_moves_bitmask(pos: u8) -> u64 {
    if pos > 63 {
        panic!("Invalid position");
    }

    let mut moves: u64 = 0;

    // Up 2, Right 1
    if pos < 48 && pos % 8 != 7 {
        moves |= 1u64 << (pos + 17);
    }

    // Up 2, Left 1
    if pos < 48 && pos % 8 != 0 {
        moves |= 1u64 << (pos + 15);
    }

    // Down 2, Right 1
    if pos >= 16 && pos % 8 != 7 {
        moves |= 1u64 << (pos - 15);
    }

    // Down 2, Left 1
    if pos >= 16 && pos % 8 != 0 {
        moves |= 1u64 << (pos - 17);
    }

    // Right 2, Up 1
    if pos < 56 && pos % 8 < 6 {
        moves |= 1u64 << (pos + 10);
    }

    // Right 2, Down 1
    if pos >= 8 && pos % 8 < 6 {
        moves |= 1u64 << (pos - 6);
    }

    // Left 2, Up 1
    if pos < 56 && pos % 8 > 1 {
        moves |= 1u64 << (pos + 6);
    }

    // Left 2, Down 1
    if pos >= 8 && pos % 8 > 1 {
        moves |= 1u64 << (pos - 10);
    }

    moves
}
fn rook_moves_bitmask(pos: u8) -> u64 {
    if pos > 63 {
        panic!("Invalid position");
    }

    let mut moves: u64 = 0;

    // Vertical moves
    for i in 0..8 {
        if pos / 8 != i {
            moves |= 1u64 << (pos % 8 + i * 8);
        }
    }

    // Horizontal moves
    for i in 0..8 {
        if pos % 8 != i {
            moves |= 1u64 << (i + (pos / 8) * 8);
        }
    }

    moves
}
fn bishop_moves_bitmask(pos: u8) -> u64 {
    if pos > 63 {
        panic!("Invalid position");
    }

    let mut moves: u64 = 0;
    let (mut row, mut col) = (pos / 8, pos % 8);

    // Up-Right
    while row < 7 && col < 7 {
        row += 1;
        col += 1;
        moves |= 1u64 << (row * 8 + col);
    }

    // Reset
    let (mut row, mut col) = (pos / 8, pos % 8);

    // Up-Left
    while row < 7 && col > 0 {
        row += 1;
        col -= 1;
        moves |= 1u64 << (row * 8 + col);
    }

    // Reset
    let (mut row, mut col) = (pos / 8, pos % 8);

    // Down-Right
    while row > 0 && col < 7 {
        row -= 1;
        col += 1;
        moves |= 1u64 << (row * 8 + col);
    }

    // Reset
    let (mut row, mut col) = (pos / 8, pos % 8);

    // Down-Left
    while row > 0 && col > 0 {
        row -= 1;
        col -= 1;
        moves |= 1u64 << (row * 8 + col);
    }

    moves
}
fn queen_moves_bitmask(pos: u8) -> u64 {
    rook_moves_bitmask(pos) | bishop_moves_bitmask(pos)
}
fn white_pawn_moves(square: u8) -> (u64, u64) {
    assert!(square < 64 && square / 8 != 7); // Not valid for the last rank

    let mut move_mask: u64 = 0;
    let mut attack_mask: u64 = 0;

    // Normal move (forward)
    if square / 8 != 7 {
        move_mask |= 1 << (square + 8);
    }

    // Attack moves (diagonal forward left and right)

    // Check if not on the left edge
    if square % 8 != 0 {
        attack_mask |= 1 << (square + 7);
    }

    // Check if not on the right edge
    if (square + 1) % 8 != 0 {
        attack_mask |= 1 << (square + 9);
    }

    (move_mask, attack_mask)
}
