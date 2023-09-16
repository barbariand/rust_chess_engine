use super::BoardPosition;
use super::File;
use super::Rank;
use crate::chess_engine::history::History;
use crate::chess_engine::pieces::Action;
use crate::chess_engine::pieces::Color;
use crate::chess_engine::pieces::Piece;
use std::fmt::Display;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Clone)]
pub struct BoardRank([Option<Piece>; 8]);
#[derive(Debug, Clone)]
pub struct Board {
    inner_board: [BoardRank; 8],
    turn: Color,
    history:History,
}

impl Board {
    pub fn has_piece(&self, pos: &BoardPosition) -> bool {
        self[&pos.rank][&pos.file].is_some()
    }
    pub fn is_piece_color(&self, pos:&BoardPosition,color:Color)->bool{
        self[&pos.rank][&pos.file].is_some_and(|p|p.color==color)
    }
    pub fn new() -> Board {
        Board {
            inner_board: [
                [
                    Some(Piece::new_rook(Color::Black)),
                    Some(Piece::new_knight(Color::Black)),
                    Some(Piece::new_bishop(Color::Black)),
                    Some(Piece::new_queen(Color::Black)),
                    Some(Piece::new_king(Color::Black)),
                    Some(Piece::new_bishop(Color::Black)),
                    Some(Piece::new_knight(Color::Black)),
                    Some(Piece::new_rook(Color::Black)),
                ],
                [Some(Piece::new_pawn(Color::Black)); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece::new_pawn(Color::White)); 8],
                [
                    Some(Piece::new_rook(Color::White)),
                    Some(Piece::new_knight(Color::White)),
                    Some(Piece::new_bishop(Color::White)),
                    Some(Piece::new_queen(Color::White)),
                    Some(Piece::new_king(Color::White)),
                    Some(Piece::new_bishop(Color::White)),
                    Some(Piece::new_knight(Color::White)),
                    Some(Piece::new_rook(Color::White)),
                ],
            ]
            .map(|v| BoardRank(v)),
            turn: Color::White,
            history:History(Vec::new())
        }
    }
    pub fn move_piece(&mut self,action: Action){
        action.execute(self);
        self.history.add(action);
    }
    pub fn get_movement_options() {}
    pub fn into_iter(&self)->Box<dyn Iterator<Item = Piece>+ '_> {
        Box::new(self.inner_board.iter().map(|a|a.0.into_iter().flatten().collect::<Vec<Piece>>()).flatten())
    }
}
impl IndexMut<&Rank> for Board {
    fn index_mut(&mut self, index: &Rank) -> &mut Self::Output {
        self.inner_board
            .get_mut(<&Rank as Into<i8>>::into(index) as usize)
            .expect("Rank is bigger than board")
    }
}
impl IndexMut<&BoardPosition> for Board{
    fn index_mut(&mut self, index: &BoardPosition) -> &mut Self::Output {
        &mut self[&index.rank][&index.file]
    }
}
impl Index<&Rank> for Board {
    type Output = BoardRank;
    fn index(&self, index: &Rank) -> &Self::Output {
        self.inner_board
            .get(<&Rank as Into<i8>>::into(index) as usize)
            .expect("Rank is bigger than board")
    }
}
impl Index<&File> for BoardRank {
    type Output = Option<Piece>;
    fn index(&self, index: &File) -> &Self::Output {
        self.0
            .get(<&File as Into<i8>>::into(index) as usize)
            .expect("File is bigger than board")
    }
}
impl IndexMut<File> for BoardRank {
    fn index_mut(&mut self, index: File) -> &mut Self::Output {
        self.0
            .get_mut(<File as Into<i8>>::into(index) as usize)
            .expect("File is bigger than board")
    }
}
impl IndexMut<&File> for BoardRank {
    fn index_mut(&mut self, index: &File) -> &mut Self::Output {
        self.0
            .get_mut(<&File as Into<i8>>::into(index) as usize)
            .expect("File is bigger than board")
    }
}
impl Index<Rank> for Board {
    type Output = BoardRank;
    fn index(&self, index: Rank) -> &Self::Output {
        self.inner_board
            .get(<Rank as Into<i8>>::into(index) as usize)
            .expect("Rank is bigger than board")
    }
}
impl Index<File> for BoardRank {
    type Output = Option<Piece>;
    fn index(&self, index: File) -> &Self::Output {
        self.0
            .get(<File as Into<i8>>::into(index) as usize)
            .expect("File is bigger than board")
    }
}
impl Index<&BoardPosition> for Board {
    type Output = Option<Piece>;
    fn index(&self, index: &BoardPosition) -> &Self::Output {
        &self[index.rank][index.file]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in &self.inner_board {
            writeln!(f, "{}", rank)?;
        }
        Ok(())
    }
}
impl Display for BoardRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for piece in self.0 {
            match piece {
                Some(s) => write!(f, "{: ^15}|", s)?,
                None => write!(f, "{:>3}|", " ")?,
            }
        }
        Ok(())
    }
}

