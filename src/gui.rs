use std::error::Error;

use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::KeyInput;
use rsoderh_chess::{Color, FinishedGame, Game, GameResult, HalfMoveRequest, MoveResult, PieceKind, Position};
use crate::network::chess_tp::GameStateType;
use crate::network::chess_tp::Message;
use crate::resources::Resources;
use drawing::colors::*;
use crate::network::GameConnection;
use crate::util::ReplaceCell;

mod drawing;
mod util;

/// Contains the current state of a chess game, whether it is ongoing or finished
#[derive(Debug)]
enum GameState {
    OngoingGame(Game),
    FinishedGame(FinishedGame),
}

/// Represents a selected square and a cache of its available moves
#[derive(Debug)]
struct SquareSelection {
    pos: Position,
    available_moves: Box<[Position]>
}

/// Represents the current state of the application
#[derive(Debug)]
pub struct GuiState {
    // game data and resources
    resources: Resources,
    game_state: ReplaceCell<GameState>,

    // network connection
    connection: Option<GameConnection>,

    // gui/visuals data
    hovered_square: Option<Position>,
    selected_square: Option<SquareSelection>,
    promotion_selection: Option<Position>,
}

impl GuiState {
    pub fn new_local(ctx: &mut ggez::Context) -> ggez::GameResult<GuiState> {
        Ok(GuiState {
            resources: Resources::new(ctx)?,
            game_state: ReplaceCell::new(GameState::OngoingGame(Game::new_standard())),
            connection: None,
            hovered_square: None,
            selected_square: None,
            promotion_selection: None,
        })
    }

    pub fn new_remote(ctx: &mut ggez::Context, connection: GameConnection) -> Result<GuiState, Box<dyn Error>> {
        Ok(GuiState {
            resources: Resources::new(ctx)?,
            game_state: ReplaceCell::new(GameState::OngoingGame(Game::new_standard())),
            connection: Some(connection),
            hovered_square: None,
            selected_square: None,
            promotion_selection: None,
        })
    }

    pub fn is_local_player_turn(&self) -> bool {
        match self.game_state.get_ref() {
            GameState::OngoingGame(game) => {
                self.connection.as_ref()
                    .is_none_or(|connection| connection.local_player() == game.turn)
            },
            GameState::FinishedGame(finished_game) => false,
        }
    }

    pub fn on_quit(&mut self, message: Option<String>) {
        if let Some(connection) = &mut self.connection {
            let _ = connection.send_message(Message::ChessQuit { payload: message.unwrap_or(String::new()) });
        }
    }

    fn reset_selection(&mut self) {
        self.selected_square = None;
        self.hovered_square = None;
        self.promotion_selection = None;
    }

    fn game_state_type(&self) -> GameStateType {
        match self.game_state.get_ref() {
            GameState::OngoingGame(game) => GameStateType::Normal,
            GameState::FinishedGame(game) => match game.result() {
                GameResult::Checkmate { winner, attacked_king } => {
                    match winner {
                        Color::White => GameStateType::WhiteWon,
                        Color::Black => GameStateType::BlackWon,
                    }
                },
            }
        }
    }

    fn try_move(&mut self, chess_move: HalfMoveRequest, is_remote_move: bool) {
        fn clone_chess_move(chess_move: &HalfMoveRequest) -> HalfMoveRequest {
            match chess_move {
                HalfMoveRequest::Standard { source, dest } => {
                    HalfMoveRequest::Standard { source: *source, dest: *dest }
                },
                HalfMoveRequest::Promotion { column, kind } => {
                    HalfMoveRequest::Promotion { column: *column, kind: *kind }
                },
            }
        }

        fn get_game_state_type(game_result: &GameResult) -> GameStateType {
            match game_result {
                GameResult::Checkmate { winner, attacked_king } => {
                    match winner {
                        Color::White => GameStateType::WhiteWon,
                        Color::Black => GameStateType::BlackWon,
                    }
                },
            }
        }

        self.game_state.replace(|game_state| match game_state {
            GameState::OngoingGame(game) => {
                let player = game.turn;
                let (new_game_state,
                    new_game_state_type,
                    new_board
                ) = match game.perform_move(clone_chess_move(&chess_move)) {
                    MoveResult::Ongoing(game, ..) => {
                        let new_board = game.board().clone();
                        (GameState::OngoingGame(game), GameStateType::Normal, new_board)
                    },
                    MoveResult::Finished(game) => {
                        let game_state_type = get_game_state_type(game.result());
                        let new_board = game.board().clone();
                        (GameState::FinishedGame(game), game_state_type, new_board)
                    },
                    MoveResult::Illegal(game, _) => {
                        return GameState::OngoingGame(game);
                    }
                };
                if !is_remote_move {
                    if let Some(connection) = &mut self.connection {
                        let _ = connection.send_message(Message::ChessMove {
                            player: Some(player),
                            chess_move,
                            new_game_state: new_game_state_type,
                            new_board,
                        });
                    }
                }
                new_game_state
            }
            GameState::FinishedGame(game) => GameState::FinishedGame(game),
        });
    }

    fn handle_message(&mut self, ctx: &mut ggez::Context, message: Message) {
        match message {
            Message::ChessMove {
                player,
                chess_move,
                new_game_state,
                new_board
            } => {
                self.try_move(chess_move, true);

                let board = match self.game_state.get_ref() {
                    GameState::OngoingGame(game) => game.board(),
                    GameState::FinishedGame(finished_game) => finished_game.board(),
                };
                let game_state = self.game_state_type();
                if new_board != *board || new_game_state != game_state {
                    if self.connection.as_ref().is_some_and(|conn| conn.strict_rule_policy()) {
                        println!("Other player made an invalid move");
                        self.on_quit(Some("Invalid move".to_owned()));
                        ctx.request_quit();
                    } else {
                        self.game_state.replace(|game_state| {
                            let turn = if let GameState::OngoingGame(game) = game_state {
                                game.turn
                            } else {
                                return game_state;
                            };
                            match new_game_state {
                                GameStateType::Normal => {
                                    GameState::OngoingGame(Game::new(new_board, turn))
                                },
                                GameStateType::WhiteWon | GameStateType::BlackWon | GameStateType::Draw => {
                                    unimplemented!("Cannot construct `FinishedGame`")
                                },
                            }
                        });
                    }
                }
            },
            Message::ChessQuit { payload } => {
                println!("Other player quit: {payload}");
                ctx.request_quit();
            },
        }
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

    fn handle_promotion_selection_click(&mut self, clicked_square: Position) {
        if !self.is_local_player_turn() {
            self.reset_selection();
            return;
        }
        let Some(game) = self.ongoing() else { return; };
        let Some(promotion_square) = self.promotion_selection else { return; };
        if let Some(piece_type) = util::promotion_selection_type(
            game.turn, promotion_square, clicked_square)
        {
            self.try_move(
                HalfMoveRequest::Promotion {
                    column: clicked_square.column,
                    kind: piece_type,
                },
                false
            );
        }
        self.reset_selection();
    }

    fn handle_board_click(&mut self, clicked_square: Option<Position>) {
        let Some(game) = self.ongoing() else { return; };
        if !self.is_local_player_turn() {
            self.reset_selection();
            return;
        }

        let selected_square = self.selected_square.as_ref().map(|selection| selection.pos);

        let clicked_square = if let Some(clicked_square) = clicked_square {
            clicked_square
        } else {
            self.reset_selection();
            return;
        };

        if matches!(self.promotion_selection, Some(_)) {
            self.handle_promotion_selection_click(clicked_square);
            return;
        }

        if Some(clicked_square) == selected_square {
            self.reset_selection();
            return;
        }

        if let Some((selected_square, available_moves)) = self.selected_square
            .as_ref().map(|selection| (selection.pos, &selection.available_moves))
        {
            let is_pawn = game.board().at_position(selected_square).as_piece()
                .is_some_and(|piece| matches!(piece.kind, PieceKind::Pawn));
            let last_rank = match game.turn {
                Color::White => 7,
                Color::Black => 0,
            };
            if is_pawn && clicked_square.row.get() == last_rank {
                // promotion move
                let is_valid_move = available_moves.contains(&clicked_square);
                if is_valid_move {
                    self.promotion_selection = Some(clicked_square);
                } else {
                    self.reset_selection();
                }
            } else {
                // normal move
                self.try_move(
                    HalfMoveRequest::Standard {
                        source: selected_square,
                        dest: clicked_square,
                    },
                    false
                );
                self.reset_selection();
            }
        } else {
            if game.board().at_position(clicked_square)
                .as_piece().is_some_and(|piece| piece.color == game.turn)
            {
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

impl event::EventHandler for GuiState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if !self.is_local_player_turn() {
            if let Some(connection) = &mut self.connection {
                if let Some(message) = connection.read_message().ok() {
                    self.handle_message(ctx, message);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, BACKGROUND_COLOR);

        let board = match self.game_state.get_ref() {
            GameState::OngoingGame(game) => game.board(),
            GameState::FinishedGame(finished_game) => finished_game.board(),
        };

        drawing::draw_board(ctx, &mut canvas, &self.resources.images, board,
                            self.selected_square.as_ref(), self.hovered_square,
                            self.ongoing().map(|game| game.turn),
                            self.promotion_selection)?;;

        let status_text = match self.game_state.get_ref() {
            GameState::OngoingGame(game) => match game.turn {
                Color::White => "White to play",
                Color::Black => "Black to play",
            }
            GameState::FinishedGame(finished_game) => match finished_game.result() {
                GameResult::Checkmate { winner, .. } => match winner {
                    Color::White => "White won by checkmate!",
                    Color::Black => "Black won by checkmate!",
                },
            } ,
        };

        drawing::draw_status_text(ctx, &mut canvas, status_text)?;

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut ggez::Context, button: event::MouseButton,
                               x: f32, y: f32) -> ggez::GameResult
    {
        if matches!(button, event::MouseButton::Left) {
            if let Some(game) = self.ongoing() {
                let clicked_square = util::global_to_board_pos(ctx, (x, y));
                self.handle_board_click(clicked_square);
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

    fn quit_event(&mut self, _ctx: &mut ggez::Context) -> Result<bool, ggez::GameError> {
        self.on_quit(Some("Player quit the game".into()));
        Ok(false)
    }
}
