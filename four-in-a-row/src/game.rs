use crate::player::Player;
use std::fmt::Display;

const EMPTY_CELL: char = '.';

pub enum GameState {
    Playing,
    Win(usize),
    Draw,
}

#[derive(Clone, Copy)]
pub struct Game {
    players: [Player; 2],
    turn: usize,
    // TODO: switch to bitboards
    grid: [[char; 7]; 6],
    last_move: usize,
}

impl Game {
    pub fn new(players: [Player; 2]) -> Self {
        Game {
            players,
            turn: 0,
            grid: [[EMPTY_CELL; 7]; 6],
            last_move: 255,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    pub fn main_loop(&mut self, log: bool) {
        if log {
            println!("{self}");
        }

        while !self.is_terminal() {
            let col = self.players[self.turn].get_move(self);
            self.do_move(col);

            if log {
                println!("{self}");
            }
        }

        if log {
            println!("Player {} won!", 1 - self.turn);
        }
    }

    pub fn do_move(&mut self, col: usize) {
        let symbol = match self.turn {
            0 => 'x',
            _ => 'o',
        };

        let row = (0..self.grid.len())
            .find(|&i| self.grid[i][col] == EMPTY_CELL)
            .unwrap();

        assert_eq!(self.grid[row][col], EMPTY_CELL);
        assert!(row == 0 || self.grid[row - 1][col] != EMPTY_CELL);

        self.grid[row][col] = symbol;
        self.turn = 1 - self.turn;
        self.last_move = col;
    }

    pub fn last_move(&self) -> usize {
        self.last_move
    }

    pub fn turn(&self) -> usize {
        self.turn
    }

    pub fn is_filled_col(&self, col: usize) -> bool {
        let (nrows, _) = self.shape();
        self.grid[nrows - 1][col] != EMPTY_CELL
    }

    pub fn get_state(&self) -> GameState {
        let (nrows, ncols) = self.shape();

        for row in 0..nrows {
            for col in 0..ncols {
                if self.grid[row][col] == EMPTY_CELL {
                    continue;
                }

                // check horizontally
                if col <= ncols - 4
                    && (col..(col + 3)).all(|i| self.grid[row][i] == self.grid[row][i + 1])
                {
                    return GameState::Win(1 - self.turn);
                }

                // check vertically
                if row <= nrows - 4
                    && (row..(row + 3)).all(|i| self.grid[i][col] == self.grid[i + 1][col])
                {
                    return GameState::Win(1 - self.turn);
                }

                // check diagonally
                if row <= nrows - 4
                    && col <= ncols - 4
                    && (1..4).all(|i| self.grid[row][col] == self.grid[row + i][col + i])
                {
                    return GameState::Win(1 - self.turn);
                }
                if row >= 3
                    && col <= ncols - 4
                    && (1..4).all(|i| self.grid[row][col] == self.grid[row - i][col + i])
                {
                    return GameState::Win(1 - self.turn);
                }
            }
        }

        if self.grid[nrows - 1].iter().all(|&cell| cell != EMPTY_CELL) {
            GameState::Draw
        } else {
            GameState::Playing
        }
    }

    pub fn is_terminal(&self) -> bool {
        !matches!(self.get_state(), GameState::Playing)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_, ncols) = self.shape();

        for i in 0..self.shape().1 {
            write!(f, "{}", i)?;
        }

        writeln!(f)?;
        writeln!(f, "{}", "-".repeat(ncols))?;

        for row in self.grid.iter().rev() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }

        writeln!(f, "{}", "-".repeat(ncols))?;

        for i in 0..self.shape().1 {
            write!(f, "{}", i)?;
        }

        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player;

    use super::Game;

    #[test]
    fn test_terminal_board_horizontal() {
        let mut game = Game::new([Player::Human; 2]);

        for i in 3..7 {
            assert!(!game.is_terminal());
            game.do_move(i);
            game.do_move(i);
        }

        assert!(game.is_terminal());
    }

    #[test]
    fn test_terminal_board_vertical() {
        let mut game = Game::new([Player::Human; 2]);

        for _ in 0..4 {
            assert!(!game.is_terminal());
            game.do_move(3);
            game.do_move(5);
        }

        assert!(game.is_terminal());
    }

    #[test]
    fn test_terminal_board_diagonal() {
        let mut game = Game::new([Player::Human; 2]);
        let moves = [0, 1, 2, 3, 1, 2, 3, 3, 2, 5, 3];

        for col in moves {
            assert!(!game.is_terminal());
            game.do_move(col);
        }

        assert!(game.is_terminal());
    }
}
