use ggez;
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;
use ggez::{Context, GameResult};

// resolution requires 4* expected pixel size, not sure why :)
static WINDOW_HEIGHT: f32 = 400.0 * 2.0;
static WINDOW_WIDTH: f32 = 800.0 * 2.0;
static PADDLE_HEIGHT: f32 = 200.0;
static PADDLE_WIDTH: f32 = 40.0;
static PADDLE_SPEED: f32 = 5.0;

#[derive(PartialEq)]
enum PlayerID {
    Player1,
    Player2
}

struct Player {
    player_id: PlayerID,
    pos_x: f32,
    pos_y: f32,
    key_up: KeyCode,
    key_down: KeyCode,
}

struct Ball {
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    radius: f32
}

impl Ball {
    fn new() -> GameResult<Ball> {
        let b = Ball { pos_x: WINDOW_WIDTH / 2.0, pos_y: WINDOW_HEIGHT / 2.0, vel_x: 5.0, vel_y: 1.0, radius: 10.0 };
        Ok(b)
    }
}

impl Player {
    fn new(id: PlayerID) -> GameResult<Player> {
        let key_up;
        let key_down;
        let start_x;
        if id == PlayerID::Player1 {
            key_up = KeyCode::W;
            key_down = KeyCode::S;
            start_x = 40.0;
        } else {
            key_up = KeyCode::Up;
            key_down = KeyCode::Down;
            start_x = WINDOW_WIDTH - PADDLE_WIDTH;
        }
        let p = Player {player_id: id, pos_x: start_x, pos_y: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0 , key_up, key_down};
        Ok(p)
    }
}

struct MainState {
    player1: Player,
    player2: Player,
    ball: Ball
}

impl MainState {
    fn new(player1: Player, player2: Player, ball: Ball) -> GameResult<MainState> {
        let s = MainState {
            player1: player1,
            player2: player2,
            ball: ball
        };
        Ok(s)
    }

    fn check_paddle_collisions(&self, player_id: PlayerID) -> bool {
        if player_id == PlayerID::Player1 {
            if self.ball.pos_x >= self.player1.pos_x && self.ball.pos_x <= self.player1.pos_x + PADDLE_WIDTH / 2.0 {
                if self.ball.pos_y >= self.player1.pos_y && self.ball.pos_y <= self.player1.pos_y + PADDLE_HEIGHT {
                    return true;
                }
            }
        }
        if player_id == PlayerID::Player2 {
            if self.ball.pos_x >= self.player2.pos_x && self.ball.pos_x <= self.player2.pos_x + PADDLE_WIDTH {
                if self.ball.pos_y >= self.player2.pos_y && self.ball.pos_y <= self.player2.pos_y + PADDLE_HEIGHT  {
                    return true;
                }
            }
        }
        return false;
    }
}

// event::EventHandler is a trait which defines what functionality the type (MainState) must provide (here it is update and draw)
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Movement
        if keyboard::is_key_pressed(ctx, self.player1.key_down) {
            self.player1.pos_y += PADDLE_SPEED;
        } else if keyboard::is_key_pressed(ctx, self.player1.key_up) {
            self.player1.pos_y -= PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, self.player2.key_down) {
            self.player2.pos_y += PADDLE_SPEED;
        } else if keyboard::is_key_pressed(ctx, self.player2.key_up) {
            self.player2.pos_y -= PADDLE_SPEED;
        }

        // Collision Checks
        if self.ball.pos_y <= 0.0 {
            self.ball.vel_y *= -1.0;
        }
        if self.ball.pos_y >= WINDOW_HEIGHT {
            self.ball.vel_y *= -1.0;
        }

        if self.check_paddle_collisions(PlayerID::Player1) {
            if self.ball.vel_x < 0.0 {
                self.ball.vel_x *= -1.0;
            }
        }
        if self.check_paddle_collisions(PlayerID::Player2) {
            if self.ball.vel_x > 0.0 {
                self.ball.vel_x *= -1.0;
            }
        }
        self.ball.pos_x += self.ball.vel_x;
        self.ball.pos_y += self.ball.vel_y;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let paddle1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.player1.pos_x, self.player1.pos_y, PADDLE_WIDTH, PADDLE_HEIGHT),
            graphics::WHITE
        )?;

        let paddle2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.player2.pos_x, self.player2.pos_y, PADDLE_WIDTH, PADDLE_HEIGHT),
            graphics::WHITE
        )?;

        let ball = graphics::Mesh::new_circle(
            ctx, 
            graphics::DrawMode::fill(),
            na::Point2::new(self.ball.pos_x, self.ball.pos_y),
            self.ball.radius,
            2.0,
            graphics::WHITE
        )?;

        let (mid_top, mid_bot) = (na::Point2::new(WINDOW_WIDTH / 2.0, 0.0 - WINDOW_HEIGHT / 2.0), na::Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT * 2.0));
        let mid_line = graphics::Mesh::new_line(ctx, &[mid_top, mid_bot], 3.0, graphics::WHITE)?;

        graphics::draw(ctx, &paddle1, (na::Point2::new(self.player1.pos_x, self.player1.pos_y),))?;
        graphics::draw(ctx, &paddle2, (na::Point2::new(self.player2.pos_x, self.player2.pos_y),))?;
        graphics::draw(ctx, &ball, (na::Point2::new(self.ball.pos_x, self.ball.pos_y),))?;
        graphics::draw(ctx, &mid_line, (na::Point2::new(WINDOW_WIDTH/2.0, WINDOW_HEIGHT/2.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("rong", "young_guns")
        .window_setup(ggez::conf::WindowSetup::default().title("rong!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (ctx, event_loop) = &mut cb.build()?;
    let window_size = graphics::Rect::new(0.0, 0.0, WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0);
    graphics::set_screen_coordinates(ctx, window_size)?;

    let p1 = Player::new(PlayerID::Player1)?;
    let p2 = Player::new(PlayerID::Player2)?;
    let ball = Ball::new()?;
    let state = &mut MainState::new(p1, p2, ball)?;
    event::run(ctx, event_loop, state)
}