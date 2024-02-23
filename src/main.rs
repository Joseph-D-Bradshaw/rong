use ggez;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::Canvas;
use ggez::graphics::{Color, PxScale, TextFragment};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use mint::Point2;

// resolution requires 4* expected pixel size, not sure why :)
const WINDOW_HEIGHT: f32 = 400.0;
const WINDOW_WIDTH: f32 = 800.0;
const PADDLE_HEIGHT: f32 = 200.0;
const PADDLE_WIDTH: f32 = 40.0;
const PADDLE_SPEED: f32 = 6.0;
const FONT: &str = "LiberationMono-Regular";

#[derive(PartialEq)]
enum PlayerID {
    Player1,
    Player2,
}

struct Player {
    /// The unique identifier for a player.
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
    radius: f32,
}

impl Ball {
    fn new() -> GameResult<Ball> {
        let b = Ball {
            pos_x: WINDOW_WIDTH / 2.0,
            pos_y: WINDOW_HEIGHT / 2.0,
            vel_x: 12.0,
            vel_y: 1.0,
            radius: 10.0,
        };
        Ok(b)
    }
}

impl Player {
    fn new(id: PlayerID) -> GameResult<Player> {
        let key_up;
        let key_down;
        let start_x: f32;
        if id == PlayerID::Player1 {
            key_up = KeyCode::W;
            key_down = KeyCode::S;
            start_x = PADDLE_WIDTH / 2.0;
        } else {
            key_up = KeyCode::Up;
            key_down = KeyCode::Down;
            start_x = WINDOW_WIDTH - PADDLE_WIDTH;
        }
        let p = Player {
            player_id: id,
            pos_x: start_x,
            pos_y: WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 4.0,
            key_up,
            key_down,
        };
        Ok(p)
    }
}

struct MainState {
    player1: Player,
    player2: Player,
    ball: Ball,
    player1_score: i32,
    player2_score: i32,
}

impl MainState {
    fn new(player1: Player, player2: Player, ball: Ball) -> GameResult<MainState> {
        let s = MainState {
            player1: player1,
            player2: player2,
            ball: ball,
            player1_score: 0,
            player2_score: 0,
        };
        Ok(s)
    }

    fn check_paddle_collisions(&self, player_id: PlayerID) -> bool {
        if player_id == PlayerID::Player1 {
            if self.ball.pos_x >= self.player1.pos_x
                && self.ball.pos_x <= self.player1.pos_x + PADDLE_WIDTH / 2.0
            {
                if self.ball.pos_y >= self.player1.pos_y
                    && self.ball.pos_y <= self.player1.pos_y + PADDLE_HEIGHT / 2.0
                {
                    return true;
                }
            }
        }
        if player_id == PlayerID::Player2 {
            if self.ball.pos_x >= self.player2.pos_x
                && self.ball.pos_x <= self.player2.pos_x + PADDLE_WIDTH
            {
                if self.ball.pos_y >= self.player2.pos_y
                    && self.ball.pos_y <= self.player2.pos_y + PADDLE_HEIGHT / 2.0
                {
                    return true;
                }
            }
        }
        return false;
    }

    fn reset_ball(&mut self) {
        self.ball.pos_x = WINDOW_WIDTH / 2.0;
        self.ball.pos_y = WINDOW_HEIGHT / 2.0;
        self.ball.vel_x *= -1.0;
    }
}

// event::EventHandler is a trait which defines what functionality the type (MainState) must provide (here it is update and draw)
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Movement

        if ctx.keyboard.is_key_pressed(self.player1.key_down)
            && self.player1.pos_y <= WINDOW_HEIGHT - PADDLE_HEIGHT / 2.0
        {
            self.player1.pos_y += PADDLE_SPEED
        } else if ctx.keyboard.is_key_pressed(self.player1.key_up) && self.player1.pos_y >= 0.0 {
            self.player1.pos_y -= PADDLE_SPEED
        }

        if ctx.keyboard.is_key_pressed(self.player2.key_down)
            && self.player2.pos_y <= WINDOW_HEIGHT - PADDLE_HEIGHT / 2.0
        {
            self.player2.pos_y += PADDLE_SPEED
        } else if ctx.keyboard.is_key_pressed(self.player2.key_up) && self.player2.pos_y >= 0.0 {
            self.player2.pos_y -= PADDLE_SPEED
        }

        // Window Ball Collision Checks
        if self.ball.pos_y <= 0.0 {
            self.ball.vel_y *= -1.0;
        }
        if self.ball.pos_y >= WINDOW_HEIGHT {
            self.ball.vel_y *= -1.0;
        }

        if self.ball.pos_x >= WINDOW_WIDTH {
            self.player1_score += 1;
            self.reset_ball();
        }

        if self.ball.pos_x <= 0.0 {
            self.player2_score += 1;
            self.reset_ball();
        }

        // Paddle Collision Checks
        if self.check_paddle_collisions(PlayerID::Player1) {
            if self.ball.vel_x < 0.0 {
                self.ball.vel_x *= -1.0;
                let offset: f32 = self.ball.pos_y - (self.player1.pos_y + PADDLE_HEIGHT / 4.0);
                let absolute_speed: f32 = self.ball.vel_x.abs() + self.ball.vel_y.abs();
                self.ball.vel_y = absolute_speed * offset / 100.0;
                if self.ball.vel_y > 5.0 {
                    self.ball.vel_y = 5.0;
                }
                if self.ball.vel_y < -5.0 {
                    self.ball.vel_y = -5.0;
                }
            }
        }
        if self.check_paddle_collisions(PlayerID::Player2) {
            if self.ball.vel_x > 0.0 {
                self.ball.vel_x *= -1.0;
                let offset: f32 = self.ball.pos_y - (self.player2.pos_y + PADDLE_HEIGHT / 4.0);
                let absolute_speed: f32 = self.ball.vel_x.abs() + self.ball.vel_y.abs();
                self.ball.vel_y = absolute_speed * offset / 100.0;
                if self.ball.vel_y > 5.0 {
                    self.ball.vel_y = 5.0;
                }
                if self.ball.vel_y < -5.0 {
                    self.ball.vel_y = -5.0;
                }
            }
        }
        self.ball.pos_x += self.ball.vel_x;
        self.ball.pos_y += self.ball.vel_y;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let paddle1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.player1.pos_x,
                self.player1.pos_y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::WHITE,
        )?;

        let paddle2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.player2.pos_x,
                self.player2.pos_y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::WHITE,
        )?;

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2 {
                x: self.ball.pos_x,
                y: self.ball.pos_y,
            },
            self.ball.radius,
            2.0,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let (mid_top, mid_bot) = (
            Point2 {
                x: WINDOW_WIDTH / 2.0,
                y: 0.0 - WINDOW_HEIGHT / 2.0,
            },
            Point2 {
                x: WINDOW_WIDTH / 2.0,
                y: WINDOW_HEIGHT * 2.0,
            },
        );
        let mid_line = graphics::Mesh::new_line(
            ctx,
            &[mid_top, mid_bot],
            3.0,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;
        let mut player1_score_text = graphics::Text::new(TextFragment {
            text: self.player1_score.to_string(),
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            scale: Some(PxScale { x: 100.0, y: 100.0 }),
            font: Some(FONT.into()),
        });
        let mut player2_score_text = graphics::Text::new(TextFragment {
            text: self.player2_score.to_string(),
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            scale: Some(PxScale { x: 100.0, y: 100.0 }),
            font: Some(FONT.into()),
        });
        if self.player1_score > 7 {
            self.ball.vel_x = 0.0;
            self.ball.vel_y = 0.0;
            player1_score_text = graphics::Text::new(TextFragment {
                text: "W".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(FONT.into()),
                scale: Some(PxScale { x: 100.0, y: 100.0 }),
            });
            player2_score_text = graphics::Text::new(TextFragment {
                text: "L".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(FONT.into()),
                scale: Some(PxScale { x: 100.0, y: 100.0 }),
            });
        }
        if self.player2_score > 7 {
            self.ball.vel_x = 0.0;
            self.ball.vel_y = 0.0;
            player1_score_text = graphics::Text::new(TextFragment {
                text: "L".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(FONT.into()),
                scale: Some(PxScale { x: 100.0, y: 100.0 }),
            });
            player2_score_text = graphics::Text::new(TextFragment {
                text: "W".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(FONT.into()),
                scale: Some(PxScale { x: 100.0, y: 100.0 }),
            });
        };
        canvas.draw(
            &paddle1,
            Point2 {
                x: self.player1.pos_x,
                y: self.player1.pos_y,
            },
        );
        canvas.draw(
            &paddle2,
            Point2 {
                x: self.player2.pos_x,
                y: self.player2.pos_y,
            },
        );
        canvas.draw(
            &ball,
            Point2 {
                x: self.ball.pos_x,
                y: self.ball.pos_y,
            },
        );
        canvas.draw(
            &mid_line,
            Point2 {
                x: WINDOW_WIDTH / 2.0,
                y: WINDOW_HEIGHT / 2.0,
            },
        );
        canvas.draw(
            &player1_score_text,
            Point2 {
                x: WINDOW_WIDTH - 150.0,
                y: 50.0,
            },
        );
        canvas.draw(
            &player2_score_text,
            Point2 {
                x: WINDOW_WIDTH + 100.0,
                y: 50.0,
            },
        );
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("rong", "young_guns")
        .window_setup(ggez::conf::WindowSetup::default().title("rong!"))
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0),
        );
    let (ctx, event_loop) = cb.build()?;

    let p1 = Player::new(PlayerID::Player1)?;
    let p2 = Player::new(PlayerID::Player2)?;
    let ball = Ball::new()?;
    let state = MainState::new(p1, p2, ball)?;
    event::run(ctx, event_loop, state)
}
