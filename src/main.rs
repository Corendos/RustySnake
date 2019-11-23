mod player;
mod utils;

use player::Player;
use utils::{Vec2D, Rectangle, Direction};

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowSetup, WindowMode, NumSamples};
use ggez::event::{self, EventHandler};

use rand::Rng;

use utils::constants::*;

fn get_window_mode(game_conf: &SnakeGameConf) -> WindowMode {
    let width = game_conf.playing_area.width() * CELL_SIZE;
    let height = game_conf.playing_area.height() * CELL_SIZE;
    WindowMode::default()
        .dimensions(width as f32, height as f32)
}

fn main() {
    let window_setup = WindowSetup::default()
        .title("Snake")
        .samples(NumSamples::Four);

    let game_conf = SnakeGameConf {
        playing_area: Rectangle::new(0, 0, 9, 9),
    };

    let window_mode = get_window_mode(&game_conf);
    
    let (mut ctx, mut event_loop) = ContextBuilder::new("snake", "Corendos")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .expect("Failed to create context");
    
    let mut my_game = SnakeGame::new(&mut ctx);

    my_game.set_game_conf(game_conf);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited clearly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct SnakeGame {
    player: Player,
    conf: SnakeGameConf,
    last_time: f32,
    state: GameState,
    food: Option<Vec2D>,
}

struct SnakeGameConf {
    playing_area: Rectangle,
}

impl SnakeGame {
    pub fn new(_ctx: &mut Context) -> SnakeGame {
        SnakeGame {
            player: Player::new(),
            conf: SnakeGameConf::default(),
            last_time: 0.0,
            state: GameState::default(),
            food: None,
        }
    }

    fn set_game_conf(&mut self, game_conf: SnakeGameConf) {
        self.conf = game_conf;
    }

    fn handle_input(&mut self, ctx: &Context) {
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::W) {
            self.player.wanted_direction = Some(Direction::Up);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::S) {
            self.player.wanted_direction = Some(Direction::Down);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::A) {
            self.player.wanted_direction = Some(Direction::Left);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::D) {
            self.player.wanted_direction = Some(Direction::Right);
        }
    }

    fn draw_playing(&self, ctx: &mut Context) {
        let body_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, BODY_SIZE as f32, BODY_SIZE as f32),
            graphics::Color::from_rgb(255, 0, 0)
        ).unwrap();

        let big_body_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, BIG_BODY_SIZE as f32, BIG_BODY_SIZE as f32),
            graphics::Color::from_rgb(255, 0, 0)
        ).unwrap();

        let food_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, FOOD_SIZE as f32, FOOD_SIZE as f32),
            graphics::Color::from_rgb(0, 255, 0)
        ).unwrap();

        if let Some(position) = &self.food {
            let food_position = Vec2D::new(
                position.x * CELL_SIZE as i32 + FOOD_OFFSET as i32,
                position.y * CELL_SIZE as i32 + FOOD_OFFSET as i32
            );
            graphics::draw(ctx, &food_mesh, graphics::DrawParam::new().dest(food_position))
                .unwrap();
        }
        
        for body_part in self.player.body.iter() {
            if body_part.is_big {
                let body_position = Vec2D {
                    x: body_part.position.x * CELL_SIZE as i32 + BIG_OFFSET as i32,
                    y: body_part.position.y * CELL_SIZE as i32 + BIG_OFFSET as i32,
                };
                let draw_param = graphics::DrawParam::new().dest(body_position);
                graphics::draw(ctx, &big_body_mesh, draw_param).unwrap();
            } else {
                let body_position = Vec2D {
                    x: body_part.position.x * CELL_SIZE as i32 + OFFSET as i32,
                    y: body_part.position.y * CELL_SIZE as i32 + OFFSET as i32,
                };
                let draw_param = graphics::DrawParam::new().dest(body_position);
                graphics::draw(ctx, &body_mesh, draw_param).unwrap();
            }
        }
    }

    fn draw_game_over(&self, ctx: &mut Context) {
        let text = graphics::Text::new("Game Over");
        let (text_width, text_height) = text.dimensions(ctx);
        let (window_width, window_height) = graphics::drawable_size(ctx);
        let text_position = Vec2D::new(
            ((window_width as u32 - text_width) / 2) as i32,
            ((window_height as u32 - text_height) / 2) as i32
        );
        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::new().dest(text_position).color(graphics::Color::from_rgb(0, 0, 0))
        ).unwrap();
    }

    fn update_playing(&mut self, ctx: &mut Context) {
        if self.food.is_none() {
            self.generate_food(ctx);
        }
        self.handle_input(ctx);

        self.last_time += ggez::timer::delta(ctx).as_secs_f32();

        if self.last_time > MOVE_DELAY {
            self.last_time -= MOVE_DELAY;

            if let Some(direction) = self.player.wanted_direction {
                match direction {
                    Direction::Up => if self.player.direction != Direction::Down { self.player.direction = direction; }
                    Direction::Down => if self.player.direction != Direction::Up { self.player.direction = direction; }
                    Direction::Left => if self.player.direction != Direction::Right { self.player.direction = direction; }
                    Direction::Right => if self.player.direction != Direction::Left { self.player.direction = direction; }
                }
            }

            if self.player.can_move(&self.conf.playing_area) {
                self.player.r#move();
                if let Some(position) = &self.food {
                    if self.player.body[0].position == *position {
                        self.player.eat();
                        self.generate_food(ctx);
                    }
                }
            } else {
                self.state = GameState::GameOver;
            }
        }

    }

    fn update_game_over(&mut self, ctx: &mut Context) {
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::Space) {
            self.state = GameState::Playing;
            self.player.reset();
            self.last_time = 0.0;
        }
    }

    fn generate_food(&mut self, _ctx: &mut Context) {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(self.conf.playing_area.min.x, self.conf.playing_area.max.x);
        let mut y = rng.gen_range(self.conf.playing_area.min.y, self.conf.playing_area.max.y);

        let mut food_position = Vec2D::new(x, y);

        while self.player.body_positions.contains(&food_position) {
            x = rng.gen_range(self.conf.playing_area.min.x, self.conf.playing_area.max.x);
            y = rng.gen_range(self.conf.playing_area.min.y, self.conf.playing_area.max.y);

            food_position = Vec2D::new(x, y);
        }

        self.food = Some(food_position);
    }
}

impl EventHandler for SnakeGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            GameState::GameOver => self.update_game_over(ctx),
            GameState::Playing => self.update_playing(ctx),
        }
        Ok(())        
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        match self.state {
            GameState::GameOver => self.draw_game_over(ctx),
            GameState::Playing => self.draw_playing(ctx),
        };

        graphics::present(ctx)
    }
}

impl SnakeGameConf {
    fn default() -> SnakeGameConf {
        SnakeGameConf {
            playing_area: Rectangle::new(0, 0, 0, 0),
        }
    }
}

enum GameState {
    Playing,
    GameOver,
}

impl GameState {
    pub fn default() -> GameState {
        GameState::Playing
    }
}