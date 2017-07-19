mod display;
use std::option::*;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Debug, Copy, Clone)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

fn flatten<T>(ref vect: Vec<Vec<T>>) -> Vec<T> where T : Clone{
    vect.iter().flat_map(|ref x| x.iter().cloned()).collect()
}

#[derive(Debug)]
pub struct Board {
    squares: [[Option<(Color, PieceKind)>; 8]; 8]
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [
                [Some((Color::Black, PieceKind::Rook)), Some((Color::Black, PieceKind::Knight)), Some((Color::Black, PieceKind::Bishop)), Some((Color::Black, PieceKind::Queen)), Some((Color::Black, PieceKind::King)), Some((Color::Black, PieceKind::Bishop)), Some((Color::Black, PieceKind::Knight)), Some((Color::Black, PieceKind::Rook))],
                [Some((Color::Black, PieceKind::Pawn)), Some((Color::Black, PieceKind::Pawn)), Some((Color::Black, PieceKind::Pawn)), Some((Color::Black, PieceKind::Pawn)), Some((Color::Black, PieceKind::Pawn)), Some((Color::Black, PieceKind::Pawn)), Some((Color::Black, PieceKind::Pawn)), Some((Color::Black, PieceKind::Pawn))],
                [None, None, None, None, None, None, None, None],
                //[Some((Color::Black, PieceKind::Rook)), None, None, None, None, None, None, Some((Color::Black, PieceKind::Pawn))], // Testing Row
                [None, None, None, None, None, None, None, None], // Normal row
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [Some((Color::White, PieceKind::Pawn)), Some((Color::White, PieceKind::Pawn)), Some((Color::White, PieceKind::Pawn)), Some((Color::White, PieceKind::Pawn)), Some((Color::White, PieceKind::Pawn)), Some((Color::White, PieceKind::Pawn)), Some((Color::White, PieceKind::Pawn)), Some((Color::White, PieceKind::Pawn))],
                [Some((Color::White, PieceKind::Rook)), Some((Color::White, PieceKind::Knight)), Some((Color::White, PieceKind::Bishop)), Some((Color::White, PieceKind::Queen)), Some((Color::White, PieceKind::King)), Some((Color::White, PieceKind::Bishop)), Some((Color::White, PieceKind::Knight)), Some((Color::White, PieceKind::Rook))]
            ]
        }
    }

    pub fn piece(&self, i: i32, j: i32) -> Option<(Color, PieceKind)> {
        if self.inside_board(i,j) {
            self.squares[i as usize][j as usize]
        } else {
            None
        }
    }

    pub fn color(&self, i: i32, j: i32) -> Option<Color> {
        match self.piece(i,j) {
            Some((c,_)) => Some(c),
            None => None
        }
    }

    pub fn moves_for(&self, i: i32, j: i32) -> HashSet<(i32, i32)> {
        self.raw_moves_for(i,j)
            .into_iter()
            .filter(|&(ni,nj)| self.inside_board(ni,nj) && self.color(ni,nj) != self.color(i,j) )
            .collect()
    }

    pub fn move_piece(&mut self, i: i32, j: i32, ni: i32, nj: i32) {
        // Maybe this should go to game instead of board
        if self.moves_for(i,j).contains(&(ni,nj)) {
            self.squares[ni as usize][nj as usize] = self.squares[i as usize][j as usize];
            self.squares[i as usize][j as usize] = None;
        }
    }

    fn raw_moves_for(&self, i: i32, j: i32) -> Vec<(i32, i32)> {
        match self.piece(i,j) {
            Some(p) => match p {
                (_, PieceKind::Knight) => {
                    vec![(i-2, j-1), (i-2, j+1), (i-1, j-2), (i-1, j+2), (i+1, j-2), (i+1, j+2), (i+2, j-1), (i+2, j+1)]
                },
                (color, PieceKind::Pawn) => {
                    let (direction, initial_pos) = match color {
                        Color::Black => (1,1),
                        Color::White => (-1,6),
                    };

                    let mut moves = vec![(i+direction, j)];

                    if i == initial_pos {
                        moves.push((i+2*direction, j));
                    }

                    let attacks = vec![(i+direction, j-1), (i+direction, j+1)];

                    flatten(vec![
                        attacks
                            .into_iter()
                            .filter(|&(ni, nj)| self.inside_board(ni, nj) && self.color(ni,nj) != None && self.color(ni,nj) != self.color(i,j))
                            .collect(),
                        moves
                            .into_iter()
                            .filter(|&(ni, nj)| self.color(ni,nj) == None)
                            .collect(),
                    ])
                },
                (_, PieceKind::Rook) => {
                    self.rook_moves(i,j)
                },
                (_, PieceKind::Bishop) => {
                    self.bishop_moves(i,j)
                },
                (_, PieceKind::Queen) => {
                    flatten(vec![
                        self.rook_moves(i,j),
                        self.bishop_moves(i,j),
                    ])
                },
                (_, PieceKind::King) => {
                    self.king_moves(i,j)
                },
            },
            None => vec![]
        }
    }

    fn rook_moves(&self, i: i32, j: i32) -> Vec<(i32, i32)> {
        flatten(vec![
            self.expand_direction(i, j, 1,0),
            self.expand_direction(i, j, 0,1),
            self.expand_direction(i, j, -1,0),
            self.expand_direction(i, j, 0,-1),
        ])
    }

    fn bishop_moves(&self, i: i32, j: i32) -> Vec<(i32, i32)> {
        flatten(vec![
            self.expand_direction(i, j, 1,1),
            self.expand_direction(i, j, 1,-1),
            self.expand_direction(i, j, -1,1),
            self.expand_direction(i, j, -1,-1),
        ])
    }

    fn king_moves(&self, i: i32, j: i32) -> Vec<(i32, i32)> {
        vec![
            (i-1,j-1),
            (i-1,j),
            (i-1,j+1),
            (i,j-1),
            (i,j+1),
            (i+1,j-1),
            (i+1,j),
            (i+1,j+1),
        ]
    }

    fn expand_direction(&self, si: i32, sj: i32, di: i32, dj: i32) -> Vec<(i32, i32)> {
        let mut moves = vec![];
        let color = self.color(si,sj);
        let mut i = si+di;
        let mut j = sj+dj;

        while self.inside_board(i,j) && color != self.color(i,j) {
            moves.push((i,j));
            if self.color(i,j) != color && self.color(i,j) != None {
                // Then we are killing an enemy
                break;
            }
            i = i+di;
            j = j+dj;
        }

        moves
    }

    fn inside_board(&self, i: i32, j: i32) -> bool {
        i>= 0 && j >= 0 && i <= 7 && j <= 7
    }
}
