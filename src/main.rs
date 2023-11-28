use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PADDLE_SPEED: f32 = 8.0;

fn main() -> tetra::Result {
	ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
		.quit_on_escape(true)
		.build()?
		.run(GameState::new)
}

struct GameState {
	paddle_texture: Texture,
	paddle_position: Vec2<f32>,
}

impl State for GameState {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgb(0.1, 0.5, 0.9));

		self.paddle_texture.draw(ctx, self.paddle_position);

		Ok(())
	}

	fn update(&mut self, ctx: &mut Context) -> tetra::Result {
		// Move up
		if input::is_key_down(ctx, Key::W) {
			self.paddle_position.y -= PADDLE_SPEED;
		}

		// Move down
		if input::is_key_down(ctx, Key::S) {
			self.paddle_position.y += PADDLE_SPEED;
		}

		Ok(())
	}
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let paddle_texture = Texture::new(ctx, "./src/assets/paddleBlue.png")?;
		let paddle_position =
        	Vec2::new(16.0, (WINDOW_HEIGHT - paddle_texture.height() as f32) / 2.0);

		Ok(GameState {
			paddle_texture,
			paddle_position,
		})
	}
}
