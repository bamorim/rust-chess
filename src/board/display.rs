// Implement display traits for all the data types

use std::fmt;

impl fmt::Display for super::Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &super::Color::White => write!(f,"W"),
            &super::Color::Black => write!(f,"B"),
        }
    }
}

impl fmt::Display for super::PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &super::PieceKind::King => write!(f,"K"),
            &super::PieceKind::Queen => write!(f,"Q"),
            &super::PieceKind::Rook => write!(f,"R"),
            &super::PieceKind::Bishop => write!(f,"B"),
            &super::PieceKind::Knight => write!(f,"N"),
            &super::PieceKind::Pawn => write!(f,"P"),
        }
    }
}

fn row_to_string(row: &[Option<(super::Color, super::PieceKind)>; 8]) -> String {
    let strings : Vec<_> = row.into_iter().map(|square| {
        match square {
            &None => "--".to_string(),
            &Some((ref c, ref k)) => format!("{}{}", c, k)
        }
    }).collect();
    return strings.join(" ");
}

fn row_strings(board: &[[Option<(super::Color, super::PieceKind)>; 8]; 8]) -> Vec<String> {
    return (0..8).zip(board).map(|(i, ref row)| format!("{} {}", i, row_to_string(row))).collect();
}

impl fmt::Display for super::Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indicators = "  0  1  2  3  4  5  6  7";
        let result_string = row_strings(&self.squares).join("\n");
        write!(f, "{}\n{}", indicators, result_string)
    }
}
