use ggez::mint::Point2;

impl Into<Point2<f32>> for Vec2D {
    fn into(self: Self) -> Point2<f32> {
        Point2::<f32> {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl std::ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Vec2D) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<Vec2D> for &Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Vec2D) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Vec2D {
    pub fn new(x: i32, y: i32) -> Vec2D {
        Vec2D {x, y}
    }

    pub fn default() -> Vec2D {
        Vec2D::new(0, 0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Rectangle {
    pub min: Vec2D,
    pub max: Vec2D,
}

impl Rectangle {
    pub fn new(x_min: i32, y_min: i32, x_max: i32, y_max: i32) -> Rectangle {
        Rectangle {
            min: Vec2D::new(x_min, y_min),
            max: Vec2D::new(x_max, y_max),
        }
    }

    pub fn contains(&self, point: &Vec2D) -> bool {
        point.x >= self.min.x &&
        point.x <= self.max.x &&
        point.y >= self.min.y &&
        point.y <= self.max.y
    }

    pub fn width(&self) -> u32 {
        (self.max.x - self.min.x) as u32 + 1
    }

    pub fn height(&self) -> u32 {
        (self.max.y - self.min.y) as u32 + 1
    }
}

pub mod constants {
    pub const CELL_SIZE: u32 = 40;
    pub const BODY_SIZE: u32 = 30;
    pub const BIG_BODY_SIZE: u32 = 36;
    pub const FOOD_SIZE: u32 = 20;
    pub const OFFSET: u32 = (CELL_SIZE - BODY_SIZE) / 2;
    pub const BIG_OFFSET: u32 = (CELL_SIZE - BIG_BODY_SIZE) / 2;
    pub const FOOD_OFFSET: u32 = (CELL_SIZE - FOOD_SIZE) / 2;
    pub const MOVE_DELAY: f32 = 0.25;
}