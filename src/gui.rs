use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::KeyInput;
use rsoderh_chess::{FinishedGame, Game, HalfMoveRequest, MoveResult, Position};
use crate::resources::Resources;
use drawing::colors::*;
use crate::util::ReplaceCell;

mod drawing;
mod util;

#[derive(Debug)]
enum GameState {
    OngoingGame(Game),
    FinishedGame(FinishedGame),
}

#[derive(Debug)]
struct SquareSelection {
    pos: Position,
    available_moves: Box<[Position]>
}

#[derive(Debug)]
pub struct GuiState {
    resources: Resources,
    game_state: ReplaceCell<GameState>,

    hovered_square: Option<Position>,
    selected_square: Option<SquareSelection>,
}

impl GuiState {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<GuiState> {
        Ok(GuiState {
            resources: Resources::new(ctx)?,
            game_state: ReplaceCell::new(GameState::OngoingGame(Game::new_standard())),
            hovered_square: None,
            selected_square: None,
        })
    }

    fn reset_selection(&mut self) {
        self.selected_square = None;
        self.hovered_square = None;
    }

    fn try_move(&mut self, half_move: HalfMoveRequest) {
        self.game_state.replace(|game_state| match game_state {
            GameState::OngoingGame(game) => match game.perform_move(half_move) {
                MoveResult::Ongoing(game, check_outcome) => {
                    GameState::OngoingGame(game)
                },
                MoveResult::Finished(game) => GameState::FinishedGame(game),
                MoveResult::Illegal(game, _) => GameState::OngoingGame(game),
            }
            GameState::FinishedGame(game) => GameState::FinishedGame(game),
        });
    }

    fn is_ongoing(&self) -> bool {
        matches!(self.game_state.get_ref(), GameState::OngoingGame(_))
    }

    fn ongoing(&self) -> Option<&Game> {
        if let GameState::OngoingGame(game) = self.game_state.get_ref() {
            Some(game)
        } else {
            None
        }
    }
}

impl event::EventHandler for GuiState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, BACKGROUND_COLOR);

        let board = match self.game_state.get_ref() {
            GameState::OngoingGame(game) => game.board(),
            GameState::FinishedGame(finished_game) => finished_game.board(),
        };

        drawing::draw_board(ctx, &mut canvas, &self.resources.images, board,
                            self.selected_square.as_ref(), self.hovered_square)?;
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut ggez::Context, button: event::MouseButton,
                               x: f32, y: f32) -> ggez::GameResult
    {
        if matches!(button, event::MouseButton::Left) {
            if let Some(game) = self.ongoing() {
                let clicked_square = util::global_to_board_pos(ctx, (x, y));
                let selected_square = self.selected_square.as_ref().map(|selection| selection.pos);

                if matches!(clicked_square, None) || clicked_square == selected_square {
                    self.selected_square = None;
                    self.reset_selection();
                } else {
                    let clicked_square = clicked_square.unwrap();
                    if let Some(selected_square) = selected_square {
                        self.try_move(HalfMoveRequest::Standard {
                            source: selected_square,
                            dest: clicked_square,
                        });
                        self.reset_selection();
                    } else {
                        self.selected_square = game.valid_moves(clicked_square)
                            .map(|available_moves|
                                SquareSelection {
                                    pos: clicked_square,
                                    available_moves,
                                });
                    }
                }
            }
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut ggez::Context, x: f32, y: f32,
                          _dx: f32, _dy: f32) -> ggez::GameResult
    {
        if self.is_ongoing() {
            self.hovered_square = util::global_to_board_pos(ctx, (x, y));
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, _input: KeyInput,
                      _repeated: bool) -> ggez::GameResult
    {
        // overrides the default behavior of exiting the program when pressing ESC
        Ok(())
    }
}
