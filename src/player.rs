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

        for i in (0..self.body.len()).rev() {
            if i == (self.body.len() - 1) {
                let tail_position = self.body[i].position;
                if self.body[i].is_big {
                    let mut new_tail = SnakeBodyPart::new_tail(tail_position.x, tail_position.y);
                    new_tail.orientation = self.body[i].orientation;
                    self.body.push(new_tail);

                    self.body[i].is_tail = false;
                    self.body[i].orientation = self.body[i - 1].orientation;
                } else {
                    self.body_positions.remove(&tail_position);

                    self.body[i].orientation = (self.body[i - 1].orientation.0, None);
                }
                self.body[i].position = self.body[i - 1].position;
                self.body[i].is_big = self.body[i - 1].is_big;
            } else if i == 0 {
                self.body[i].position = dest;
                self.body[i].is_big = false;
                self.body[i].orientation = (None, Some(self.direction));

                self.body[i + 1].orientation.0 = Some(self.direction);

                self.body_positions.insert(dest);
            } else {
                self.body[i].position = self.body[i - 1].position;
                self.body[i].orientation = self.body[i - 1].orientation;
                self.body[i].is_big = self.body[i - 1].is_big;
            }
        }
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
            SnakeBodyPart::new_head_with_orientation(6, 5, Some(Direction::Right)),
            SnakeBodyPart::new_with_orientation(5, 5, Some(Direction::Right), Some(Direction::Right)),
            SnakeBodyPart::new_with_orientation(4, 5, Some(Direction::Right), Some(Direction::Right)),
            SnakeBodyPart::new_with_orientation(3, 5, Some(Direction::Right), Some(Direction::Right)),
            SnakeBodyPart::new_with_orientation(2, 5, Some(Direction::Right), Some(Direction::Right)),
            SnakeBodyPart::new_with_orientation(1, 5, Some(Direction::Right), Some(Direction::Right)),
            SnakeBodyPart::new_tail_with_orientation(0, 5, Some(Direction::Right)),
        ]
    }
}

#[derive(Debug)]
pub struct SnakeBodyPart {
    pub is_head: bool,
    pub is_tail: bool,
    pub is_big: bool,
    pub position: Vec2D,
    pub orientation: (Option<Direction>, Option<Direction>)
}

impl SnakeBodyPart {
    pub fn new(x: i32, y: i32) -> SnakeBodyPart {
        SnakeBodyPart {
            is_head: false,
            is_tail: false,
            is_big: false,
            position: Vec2D::new(x, y),
            orientation: (None, None)
        }
    }

    pub fn new_head(x: i32, y: i32) -> SnakeBodyPart {
        let mut body_part = SnakeBodyPart::new(x, y);
        body_part.is_head = true;
        body_part
    }

    pub fn new_tail(x: i32, y: i32) -> SnakeBodyPart {
        let mut body_part = SnakeBodyPart::new(x, y);
        body_part.is_tail = true;
        body_part
    }

    pub fn new_with_orientation(x: i32, y: i32, in_orientation: Option<Direction>, out_orientation: Option<Direction>) -> SnakeBodyPart {
        let mut body_part = SnakeBodyPart::new(x, y);
        body_part.orientation = (in_orientation, out_orientation);
        body_part
    }

    pub fn new_head_with_orientation(x: i32, y: i32, orientation: Option<Direction>) -> SnakeBodyPart {
        let mut body_part = SnakeBodyPart::new_with_orientation(x, y, None, orientation);
        body_part.is_head = true;
        body_part
    }

    pub fn new_tail_with_orientation(x: i32, y: i32, orientation: Option<Direction>) -> SnakeBodyPart {
        let mut body_part = SnakeBodyPart::new_with_orientation(x, y, orientation, None);
        body_part.is_tail = true;
        body_part
    }

    pub fn get_sprite_and_rotation(&self) -> Result<(SpriteType, f32), &'static str> {
        if self.is_tail {
            match self.orientation {
                (Some(Direction::Up), None) => Ok((SpriteType::Tail, 0.0)),
                (Some(Direction::Right), None) => Ok((SpriteType::Tail, std::f32::consts::FRAC_PI_2)),
                (Some(Direction::Down), None) => Ok((SpriteType::Tail, std::f32::consts::PI)),
                (Some(Direction::Left), None) => Ok((SpriteType::Tail, -std::f32::consts::FRAC_PI_2)),
                _ => Err("Error")
            }
        } else if self.is_head {
            match self.orientation {
                (None, Some(Direction::Up)) => Ok((SpriteType::Head, 0.0)),
                (None, Some(Direction::Right)) => Ok((SpriteType::Head, std::f32::consts::FRAC_PI_2)),
                (None, Some(Direction::Down)) => Ok((SpriteType::Head, std::f32::consts::PI)),
                (None, Some(Direction::Left)) => Ok((SpriteType::Head, -std::f32::consts::FRAC_PI_2)),
                _ => Err("Error")
            }
        } else {
            match self.orientation {
                (Some(Direction::Up), Some(Direction::Up)) => Ok((SpriteType::Straight, 0.0)),
                (Some(Direction::Right), Some(Direction::Right)) => Ok((SpriteType::Straight, std::f32::consts::FRAC_PI_2)),
                (Some(Direction::Down), Some(Direction::Down)) => Ok((SpriteType::Straight, std::f32::consts::PI)),
                (Some(Direction::Left), Some(Direction::Left)) => Ok((SpriteType::Straight, -std::f32::consts::FRAC_PI_2)),
                (Some(Direction::Up), Some(Direction::Left)) => Ok((SpriteType::Right, 0.0)),
                (Some(Direction::Up), Some(Direction::Right)) => Ok((SpriteType::Left, 0.0)),
                (Some(Direction::Right), Some(Direction::Down)) => Ok((SpriteType::Left, std::f32::consts::FRAC_PI_2)),
                (Some(Direction::Right), Some(Direction::Up)) => Ok((SpriteType::Right, std::f32::consts::FRAC_PI_2)),
                (Some(Direction::Down), Some(Direction::Left)) => Ok((SpriteType::Left, std::f32::consts::PI)),
                (Some(Direction::Down), Some(Direction::Right)) => Ok((SpriteType::Right, std::f32::consts::PI)),
                (Some(Direction::Left), Some(Direction::Up)) => Ok((SpriteType::Left, -std::f32::consts::FRAC_PI_2)),
                (Some(Direction::Left), Some(Direction::Down)) => Ok((SpriteType::Right, -std::f32::consts::FRAC_PI_2)),
                _ => Err("Error")
            }
        }
    }
}

impl From<(i32, i32)> for Vec2D {
    fn from(p: (i32, i32)) -> Self {
        p.into()
    }
}

pub enum SpriteType {
    Head,
    Tail,
    Straight,
    Left,
    Right,
}