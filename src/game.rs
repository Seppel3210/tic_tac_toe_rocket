use rocket::serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Player {
    X,
    O,
}

impl Player {
    fn next_player(&mut self) {
        *self = match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}

const WIDTH: usize = 3;

#[derive(Clone, Copy, Serialize)]
pub struct TicTacToe {
    field: [[Option<Player>; WIDTH]; WIDTH],
    active_player: Player,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            field: [[None; WIDTH]; WIDTH],
            active_player: Player::X,
        }
    }

    pub fn place_cell(&mut self, x: u8, y: u8) -> Result<bool, PlacementError> {
        let cell = self
            .field
            .get_mut(y as usize)
            .ok_or(PlacementError::OutOfBounds)?
            .get_mut(x as usize)
            .ok_or(PlacementError::OutOfBounds)?;
        if !cell.is_none() {
            return Err(PlacementError::AlreadyFilled);
        }

        *cell = Some(self.active_player);
        let won = self.won();
        self.active_player.next_player();
        Ok(won)
    }

    pub fn won(&self) -> bool {
        let mut rows = [0; WIDTH];
        let mut cols = [0; WIDTH];
        let mut diagonal_0 = 0;
        let mut diagonal_1 = 0;
        for (x, y, _) in self
            .field
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (x, y, cell)))
            .filter(|(_, _, player)| player == &&Some(self.active_player))
        {
            cols[x] += 1;
            rows[y] += 1;
            if y == x {
                diagonal_0 += 1;
            }
            if y == 2 - x {
                diagonal_1 += 1;
            }
        }
        rows.into_iter()
            .chain(cols)
            .chain([diagonal_0, diagonal_1])
            .any(|x| x == 3)
    }

    pub fn active_player(&self) -> Player {
        self.active_player
    }
}

pub enum PlacementError {
    OutOfBounds,
    AlreadyFilled,
}

impl PlacementError {
    pub fn message(&self) -> &'static str {
        match self {
            PlacementError::OutOfBounds => "position is out of bounds",
            PlacementError::AlreadyFilled => "cell is already filled",
        }
    }
}
