use ggez;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;
use ggez::{Context, GameResult};

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
    key_down: KeyCode
}

impl Player {
    fn new(id: PlayerID) -> GameResult<Player> {
        let mut key_up;
        let mut key_down;
        if id == PlayerID::Player1 {
            key_up = KeyCode::W;
            key_down = KeyCode::S;
        } else {
            key_up = KeyCode::Up;
            key_down = KeyCode::Down;
        }
        let p = Player {player_id: id, pos_x: 0.0, pos_y: 380.0, key_up, key_down};
        Ok(p)
    }
}

struct MainState {
    player1: Player,
    player2: Player
}

impl MainState {
    fn new(player1: Player, player2: Player) -> GameResult<MainState> {
        let s = MainState {
            player1: player1,
            player2: player2
        };
        Ok(s)
    }
}

// event::EventHandler is a trait which defines what functionality the type (MainState) must provide (here it is update and draw)
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.player1.pos_x += 4.5;
            }
            self.player1.pos_x += 0.5;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.player1.pos_x -= 4.5;
            }
            self.player1.pos_x -= 0.5;
        } 
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 380.0),
            100.0,
            2.0,
            graphics::WHITE
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(self.player1.pos_x, self.player1.pos_y),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("rong", "young_guns");
    let (ctx, event_loop) = &mut cb.build()?;
    let p1 = Player::new(PlayerID::Player1)?;
    let p2 = Player::new(PlayerID::Player2)?;
    let state = &mut MainState::new(p1, p2);
    event::run(ctx, event_loop, state)
}