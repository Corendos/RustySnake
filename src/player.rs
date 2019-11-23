use std::collections::HashSet;
use crate::utils::{Vec2D, Rectangle, Direction};

#[derive(Debug)]
pub struct Player {
    pub direction: Direction,
    pub wanted_direction: Option<Direction>,
    pub body: Vec<SnakeBodyPart>,
    pub body_positions: HashSet<Vec2D>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            direction: Direction::Up,
            wanted_direction: None,
            body: Player::default_position(),
            body_positions: HashSet::with_capacity(5)
        }
    }

    pub fn reset(&mut self) {
        self.direction = Direction::Right;
        self.wanted_direction = None;
        self.body = Player::default_position();
        self.recompute_body_positions();
    }

    pub fn can_move(&self, playing_area: &Rectangle) -> bool {
        let dest = match self.direction {
            Direction::Up => &self.body[0].position + Vec2D::new(0, -1),
            Direction::Down => &self.body[0].position + Vec2D::new(0, 1),
            Direction::Right => &self.body[0].position + Vec2D::new(1, 0),
            Direction::Left => &self.body[0].position + Vec2D::new(-1, 0),
        };

        playing_area.contains(&dest) && !self.body_positions.contains(&dest)
    }

    pub fn r#move(&mut self) {
        let dest = &self.body[0].position + match self.direction {
            Direction::Up => Vec2D::new(0, -1),
            Direction::Down => Vec2D::new(0, 1),
            Direction::Right => Vec2D::new(1, 0),
            Direction::Left => Vec2D::new(-1, 0),
        };

        for i in (1..self.body.len()).rev() {
            if i == (self.body.len() - 1) {
                let tail_position = self.body[i].position;
                if self.body[i].is_big {
                    self.body.push(SnakeBodyPart::new(tail_position.x, tail_position.y));
                } else {
                    self.body_positions.remove(&tail_position);
                }
            }
            self.body[i].position = self.body[i - 1].position;
            self.body[i].is_big = self.body[i - 1].is_big;
        }

        self.body[0].position = dest;
        self.body[0].is_big = false;
        self.body_positions.insert(dest);
    }

    fn recompute_body_positions(&mut self) {
        self.body_positions.clear();
        self.body_positions.extend(self.body.iter().map(|body_part| { body_part.position }));
    }

    pub fn eat(&mut self) {
        self.body[0].is_big = true;
    }

    fn default_position() -> Vec<SnakeBodyPart> {
        vec![
            SnakeBodyPart::new(5, 5),
            SnakeBodyPart::new(5, 5),
            SnakeBodyPart::new(5, 5),
            SnakeBodyPart::new(5, 5),
            SnakeBodyPart::new(5, 5),
            SnakeBodyPart::new(5, 5),
            SnakeBodyPart::new(5, 5),
        ]
    }
}

#[derive(Debug)]
pub struct SnakeBodyPart {
    pub is_head: bool,
    pub is_big: bool,
    pub position: Vec2D,
}

impl SnakeBodyPart {
    pub fn new(x: i32, y: i32) -> SnakeBodyPart {
        SnakeBodyPart {
            is_head: false,
            is_big: false,
            position: Vec2D::new(x, y)
        }
    }

    pub fn _new_big(x: i32, y: i32) -> SnakeBodyPart {
        SnakeBodyPart {
            is_head: false,
            is_big: true,
            position: Vec2D::new(x, y)
        }
    }
}

impl From<(i32, i32)> for Vec2D {
    fn from(p: (i32, i32)) -> Self {
        p.into()
    }
}