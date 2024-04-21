// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        };
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for instruction in instructions.chars() {
            match instruction {
                'R' => self = self.turn_right(),
                'L' => self = self.turn_left(),
                'A' => self = self.advance(),
                _ => continue,
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
