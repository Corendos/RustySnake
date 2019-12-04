mod player;
mod utils;

use player::{Player, SpriteType};
use utils::{Vec2D, Rectangle, Direction};

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowSetup, WindowMode, NumSamples};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;

use rand::Rng;

use utils::constants::*;

fn main() {
    let window_setup = WindowSetup::default()
        .title("Snake")
        .samples(NumSamples::Four);

    let game_conf = SnakeGameConf::new(10, 10);
    
    let (mut ctx, mut event_loop) = ContextBuilder::new("snake", "Corendos")
        .window_setup(window_setup)
        .window_mode(game_conf.compute_window_mode())
        .build()
        .expect("Failed to create context");
    
    let mut my_game = SnakeGame::new(&mut ctx, game_conf);

    my_game.load_resources(&mut ctx);

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
    resources: SnakeGameResources,
}

impl SnakeGame {
    pub fn new(_ctx: &mut Context, conf: SnakeGameConf) -> SnakeGame {
        SnakeGame {
            player: Player::new(),
            conf,
            last_time: 0.0,
            state: GameState::default(),
            food: None,
            resources: SnakeGameResources::default(),
        }
    }

    fn load_resources(&mut self, ctx: &mut Context) {
        let head_sprite = graphics::Image::new(ctx, "/snake_head.png")
            .expect("Failed to load the specified resource");
        self.resources.head_sprite = Some(head_sprite);

        let body_sprite = graphics::Image::new(ctx, "/snake_body.png")
            .expect("Failed to load the specified resource");
        self.resources.body_sprite = Some(body_sprite);

        let body_right_sprite = graphics::Image::new(ctx, "/snake_body_right.png")
            .expect("Failed to load the specified resource");
        self.resources.body_right_sprite = Some(body_right_sprite);

        let body_left_sprite = graphics::Image::new(ctx, "/snake_body_left.png")
            .expect("Failed to load the specified resource");
        self.resources.body_left_sprite = Some(body_left_sprite);

        let body_sprite_big = graphics::Image::new(ctx, "/snake_body_big.png")
            .expect("Failed to load the specified resource");
        self.resources.body_sprite_big = Some(body_sprite_big);

        let body_right_sprite_big = graphics::Image::new(ctx, "/snake_body_right_big.png")
            .expect("Failed to load the specified resource");
        self.resources.body_right_sprite_big = Some(body_right_sprite_big);

        let body_left_sprite_big = graphics::Image::new(ctx, "/snake_body_left_big.png")
            .expect("Failed to load the specified resource");
        self.resources.body_left_sprite_big = Some(body_left_sprite_big);

        let tail_sprite = graphics::Image::new(ctx, "/snake_tail.png")
            .expect("Failed to load the specified resource");
        self.resources.tail_sprite = Some(tail_sprite);

        let food_sprite = graphics::Image::new(ctx, "/food.png")
            .expect("Failed to load the specified resource");
        self.resources.food_sprite = Some(food_sprite);

        let body_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, BODY_SIZE as f32, BODY_SIZE as f32),
            graphics::Color::from_rgb(255, 0, 0)
        ).unwrap();
        self.resources.body_mesh = Some(body_mesh);

        let big_body_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, BIG_BODY_SIZE as f32, BIG_BODY_SIZE as f32),
            graphics::Color::from_rgb(255, 0, 0)
        ).unwrap();
        self.resources.big_body_mesh = Some(big_body_mesh);

        let food_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, FOOD_SIZE as f32, FOOD_SIZE as f32),
            graphics::Color::from_rgb(0, 255, 0)
        ).unwrap();
        self.resources.food_mesh = Some(food_mesh);
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
        //self.player.print_debug();

        self.draw_food(ctx);
        self.draw_snake(ctx);
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

    fn draw_food(&self, ctx: &mut Context) {
        if let Some(position) = &self.food {
            let food_position = Vec2D::new(
                position.x * CELL_SIZE as i32,
                position.y * CELL_SIZE as i32
            );
            graphics::draw(ctx, self.resources.food_sprite.as_ref().unwrap(), graphics::DrawParam::new().dest(food_position))
                .unwrap();
        }
    }

    fn draw_snake(&self, ctx: &mut Context) {
        for body_part in self.player.body.iter() {
            let (sprite_type, rotation) = body_part.get_sprite_and_rotation().unwrap();

            let draw_param = graphics::DrawParam::new()
                .rotation(rotation)
                .offset(Point2 {
                    x: 0.5,
                    y: 0.5,
                })
                .dest(Vec2D::new(
                    body_part.position.x * CELL_SIZE as i32 + (CELL_SIZE / 2) as i32,
                    body_part.position.y * CELL_SIZE as i32 + (CELL_SIZE / 2) as i32
                ));
            
            match (sprite_type, body_part.is_big) {
                (SpriteType::Head, _) => graphics::draw(ctx, self.resources.head_sprite.as_ref().unwrap(), draw_param),
                (SpriteType::Tail, _) => graphics::draw(ctx, self.resources.tail_sprite.as_ref().unwrap(), draw_param),
                (SpriteType::Straight, true) => graphics::draw(ctx, self.resources.body_sprite_big.as_ref().unwrap(), draw_param),
                (SpriteType::Straight, false) => graphics::draw(ctx, self.resources.body_sprite.as_ref().unwrap(), draw_param),
                (SpriteType::Left, true) => graphics::draw(ctx, self.resources.body_left_sprite_big.as_ref().unwrap(), draw_param),
                (SpriteType::Left, false) => graphics::draw(ctx, self.resources.body_left_sprite.as_ref().unwrap(), draw_param),
                (SpriteType::Right, true) => graphics::draw(ctx, self.resources.body_right_sprite_big.as_ref().unwrap(), draw_param),
                (SpriteType::Right, false) => graphics::draw(ctx, self.resources.body_right_sprite.as_ref().unwrap(), draw_param),
            }.unwrap();
            /*if body_part.is_big {
                let body_position = Vec2D {
                    x: body_part.position.x * CELL_SIZE as i32 + BIG_OFFSET as i32,
                    y: body_part.position.y * CELL_SIZE as i32 + BIG_OFFSET as i32,
                };
                let draw_param = graphics::DrawParam::new().dest(body_position);
                graphics::draw(ctx, self.resources.big_body_mesh.as_ref().unwrap(), draw_param).unwrap();
            } else {
                let body_position = Vec2D {
                    x: body_part.position.x * CELL_SIZE as i32 + OFFSET as i32,
                    y: body_part.position.y * CELL_SIZE as i32 + OFFSET as i32,
                };
                let draw_param = graphics::DrawParam::new().dest(body_position);
                graphics::draw(ctx, self.resources.body_mesh.as_ref().unwrap(), draw_param).unwrap();
            }*/
        }
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

struct SnakeGameConf {
    playing_area: Rectangle,
}

impl SnakeGameConf {
    fn new(width: u32, height: u32) -> SnakeGameConf {
        SnakeGameConf {
            playing_area: Rectangle::new(0, 0, width as i32 - 1, height as i32 - 1)
        }
    }

    fn compute_window_mode(&self) -> WindowMode {
        let width = self.playing_area.width() * CELL_SIZE;
        let height = self.playing_area.height() * CELL_SIZE;
        WindowMode::default()
            .dimensions(width as f32, height as f32)
    }
}

struct SnakeGameResources {
    head_sprite: Option<graphics::Image>,
    body_sprite: Option<graphics::Image>,
    body_right_sprite: Option<graphics::Image>,
    body_left_sprite: Option<graphics::Image>,
    body_sprite_big: Option<graphics::Image>,
    body_right_sprite_big: Option<graphics::Image>,
    body_left_sprite_big: Option<graphics::Image>,
    tail_sprite: Option<graphics::Image>,
    food_sprite: Option<graphics::Image>,
    body_mesh: Option<graphics::Mesh>,
    big_body_mesh: Option<graphics::Mesh>,
    food_mesh: Option<graphics::Mesh>,
}

impl SnakeGameResources {
    fn default() -> SnakeGameResources {
        SnakeGameResources {
            head_sprite: None,
            body_sprite: None,
            body_right_sprite: None,
            body_left_sprite: None,
            body_sprite_big: None,
            body_right_sprite_big: None,
            body_left_sprite_big: None,
            tail_sprite: None,
            food_sprite: None,
            body_mesh: None,
            big_body_mesh: None,
            food_mesh: None,
        }
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

enum GameState {
    Playing,
    GameOver,
}

impl GameState {
    pub fn default() -> GameState {
        GameState::Playing
    }
}