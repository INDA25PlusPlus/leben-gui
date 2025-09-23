use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::KeyInput;
use rsoderh_chess::{FinishedGame, Game};
use crate::resources::{Resources};
use crate::drawing;
use crate::drawing::colors::*;

enum GameState {
    OngoingGame(Game),
    FinishedGame(FinishedGame),
}

pub struct GameContainer {
    resources: Resources,
    game_state: GameState,
}

impl GameContainer {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<GameContainer> {
        Ok(GameContainer {
            resources: Resources::new(ctx)?,
            game_state: GameState::OngoingGame(Game::new_standard()),
        })
    }
}

impl event::EventHandler for GameContainer {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, BACKGROUND_COLOR);

        let board = match &self.game_state {
            GameState::OngoingGame(game) => game.board(),
            GameState::FinishedGame(finished_game) => finished_game.board(),
        };

        drawing::draw_board(ctx, &mut canvas, &self.resources.images, board)?;
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut ggez::Context, input: KeyInput, _repeated: bool) -> ggez::GameResult {
        // overrides the default behavior of exiting the program when pressing ESC
        Ok(())
    }
}
