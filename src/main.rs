use tetra::graphics::{self, Color, Texture};
// use tetra::input::{self, Key};
// use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

fn main() -> tetra::Result {
	ContextBuilder::new("Pong", 640, 480)
		.quit_on_escape(true)
		.build()?
		.run(GameState::new)
}

struct GameState {
	paddle_texture: Texture
}

impl State for GameState {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgb(0.1, 0.5, 0.9));

		Ok(())
	}
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let paddle_texture = Texture::new(ctx, "./src/assets/paddleBlue.png")?;
		Ok(GameState { paddle_texture })
	}
}
