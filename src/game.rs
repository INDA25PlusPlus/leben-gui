use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::KeyInput;

pub struct GameContainer {}

impl GameContainer {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<GameContainer> {
        Ok(GameContainer {})
    }
}

impl event::EventHandler for GameContainer {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let background_color = graphics::Color::from_rgb(30, 30, 30);
        let mut canvas = graphics::Canvas::from_frame(
            ctx, background_color);

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut ggez::Context, input: KeyInput, _repeated: bool) -> ggez::GameResult {
        // overrides the default behavior of exiting the program when pressing ESC
        Ok(())
    }
}
