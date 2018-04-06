extern crate termion;

use std::fmt;
// use std::convert::From;
use std::error::Error;
// use self::termion::{clear, cursor, color, style};


type NonoNum = u16;
pub type Clues = Vec<NonoNum>;


#[derive(Debug, Clone, PartialEq)]
pub enum Entry {
    Unsolved,
    Empty,
    Filled
}

// impl Default for Entry{
//     fn default() -> Entry {Entry::Unsolved}
// }


#[derive(Default, Debug, Clone, PartialEq)]
pub struct Nonogram{
    pub size_x: usize,
    pub size_y: usize,
    rows: Vec<Clues>,
    cols: Vec<Clues>,
    grid: Vec<Vec<Entry>>,
}

impl Nonogram{


    pub fn is_valid(&self) -> bool {
        if self.rows.len() != self.size_x || self.cols.len() != self.size_y {
            return false;
        }
        if self.grid.len() != self.size_x {
            return false;
        }
        for v in &self.grid {
            if v.len() != self.size_y { return false }
        }

        return true;
    }


    pub fn solve (&mut self) -> Result<(), Box<Error>> {

        // first do the obvious
        for r in 0..self.rows.len() {
            if self.rows[r] == vec![0] {
                for c in 0..self.cols.len() {
                    self.grid[c][r] = Entry::Empty;
                }
            }
        }
        for c in 0..self.cols.len() {
            if self.cols[c] == vec![0] {
                for r in 0..self.rows.len() {
                    self.grid[c][r] = Entry::Empty;
                }
            }
        }


        return Ok(());

    }
}

// TODO Formatting now allows only single digit clues
impl fmt::Display for Nonogram{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut max_clue_rows = 0;
        let mut max_clue_cols = 0;
        for v in &self.rows {
            if v.len() > max_clue_rows { max_clue_rows = v.len(); }
        }
        for v in &self.cols {
            if v.len() > max_clue_cols { max_clue_cols = v.len(); }
        }

        // Printing the Header
        println!("max_col:{} max_row:{} col.len:{} row.len{} \ncols:{:?} \nrows:{:?} \ngrid:{:?}\n",
                 max_clue_cols, max_clue_rows, self.cols.len(), self.rows.len(),
                 self.cols, self.rows, self.grid);

        for j in 0..max_clue_cols{
            for _i in 0..max_clue_rows {write!(f, " ")?;}
            write!(f, "┃")?;
            for v in &self.cols {
                if v.len() >= max_clue_cols - j {
                    write!(f, "{}", v[j - (max_clue_cols - v.len()) ])?;
                }
                else{
                    write!(f, " ")?;
                }
            }
            write!(f, "┃\n")?;
        }
        // Separator line
        for _i in 0..max_clue_rows {write!(f, "━")?;}
        write!(f, "╋")?;
        for _i in 1..self.cols.len() + 1 {write!(f, "━")?;}
        write!(f, "┫\n")?;

        //Printing the lines
        for line in 0..self.rows.len() {
            // clue column
            for i in 0..max_clue_rows{
                if self.rows[line].len() >= max_clue_rows - i {
                    write!(f, "{}", self.rows[line][i - (max_clue_rows - self.rows[i].len()) ])?;
                }
                else{
                    write!(f, " ")?;
                }

            }
            write!(f, "┃")?;

            for col in 0..self.cols.len() {
                // write!(f, "{:?}", self.grid[col][line])?;
                match self.grid[col][line] {
                    Entry::Empty => write!(f, " ")?,
                    Entry::Filled => write!(f, "#")?,
                    Entry::Unsolved => write!(f, ".")?,

                }
            }
            // Field
            write!(f, "┃\n")?;
        }



        // Bottom line
        for _i in 0..max_clue_rows {write!(f, "━")?;}
        write!(f, "┻")?;
        for _i in 1..self.cols.len() + 1 {write!(f, "━")?;}
        write!(f, "┛\n")
    }
}


pub fn new (rows: Vec<Clues>, cols : Vec<Clues>) -> Result<Nonogram, Box<Error>>  {
    let size_x = rows.len();
    let size_y = cols.len();
    let mut tmpline = Vec::new();
    for _i in 0..size_x {
        tmpline.push(Entry::Unsolved);
    }
    let mut grid = Vec::new();
    for _i in 0..size_y{
        grid.push(tmpline.clone());
    }
    let nono = Nonogram {size_x: size_x, size_y: size_y, cols: cols, rows: rows, grid: grid};
    return Ok(nono);

}


#[cfg(test)]
mod test {

    use super::*;

    fn valid_nono(){
        let col : Vec<Clues> = vec![vec![0], vec![1]];
        let row : Vec<Clues> = vec![vec![0], vec![1]];
        let mut grid = Vec::new();
        grid.push(vec![Entry::Empty, Entry::Empty]);
        grid.push(vec![Entry::Empty, Entry::Empty]);
        let nono = Nonogram {size_x: 2, size_y: 2, cols: col, rows: row, grid: grid};
        assert!(nono.is_valid() );
    }

}
