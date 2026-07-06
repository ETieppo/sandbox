use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::winit::keyboard::KeyCode;
use ggez::winit::keyboard::PhysicalKey;
use ggez::{Context, GameResult};

const VELOCITY: f32 = 20.;
const CLOSE_APPLE_THRESHOLD: f32 = 18.;
const WINDOW_HEIGHT: f32 = 650.;
const WINDOW_WIDTH: f32 = 650.;

#[derive(Clone, Debug)]
struct TilePos(f32, f32);

struct MainState {
	snake_body: Vec<TilePos>,
	head_direction: SegDirectEnum,
	apple: TilePos,
	can_move: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SegDirectEnum {
	Top,
	Bottom,
	Left,
	Right,
}

impl MainState {
	fn new() -> GameResult<MainState> {
		let s = MainState {
			head_direction: SegDirectEnum::Right,
			snake_body: Vec::from([TilePos(160., 160.), TilePos(10., 10.)]),
			apple: TilePos(400., 400.),
			can_move: true,
		};
		Ok(s)
	}
}

impl MainState {
	fn move_head(&self, mut seg: TilePos, direction: &SegDirectEnum) -> TilePos {
		match direction {
			SegDirectEnum::Top => seg.1 -= VELOCITY,
			SegDirectEnum::Right => seg.0 += VELOCITY,
			SegDirectEnum::Bottom => seg.1 += VELOCITY,
			SegDirectEnum::Left => seg.0 -= VELOCITY,
		}
		seg
	}

	fn return_if_border(&self, mut seg: TilePos) -> TilePos {
		if seg.1 <= 0. && self.head_direction == SegDirectEnum::Top {
			seg.1 = WINDOW_HEIGHT;
		}
		if seg.1 >= WINDOW_HEIGHT && self.head_direction == SegDirectEnum::Bottom {
			seg.1 = 0.;
		}
		if seg.0 <= 0. && self.head_direction == SegDirectEnum::Left {
			seg.0 = WINDOW_WIDTH;
		}
		if seg.0 >= WINDOW_WIDTH && self.head_direction == SegDirectEnum::Right {
			seg.0 = 0.;
		}
		seg
	}

	fn on_key_release(&mut self, ctx: &mut Context) {
		if ctx.keyboard.is_physical_key_pressed(&PhysicalKey::Code(KeyCode::KeyH)) && self.head_direction != SegDirectEnum::Right {
			self.head_direction = SegDirectEnum::Left;
		}

		if ctx.keyboard.is_physical_key_pressed(&PhysicalKey::Code(KeyCode::KeyJ)) && self.head_direction != SegDirectEnum::Top {
			self.head_direction = SegDirectEnum::Bottom;
		}

		if ctx.keyboard.is_physical_key_pressed(&PhysicalKey::Code(KeyCode::KeyK)) && self.head_direction != SegDirectEnum::Bottom {
			self.head_direction = SegDirectEnum::Top;
		}

		if ctx.keyboard.is_physical_key_pressed(&PhysicalKey::Code(KeyCode::KeyL)) && self.head_direction != SegDirectEnum::Left {
			self.head_direction = SegDirectEnum::Right;
		}
	}

	fn spawn_seg(&mut self, seg: TilePos) {
		self.snake_body.push(seg);
	}

	fn is_close_to_head(&self, item: &TilePos) -> bool {
		let x_avg = (self.snake_body[0].1 - item.1).abs() < CLOSE_APPLE_THRESHOLD;
		let y_avg = (self.snake_body[0].0 - item.0).abs() < CLOSE_APPLE_THRESHOLD;
		x_avg && y_avg
	}

	fn change_apple_pos(&mut self) {
		self.apple.1 = rand::random_range(0..WINDOW_HEIGHT as u64) as f32;
		self.apple.0 = rand::random_range(0..WINDOW_WIDTH as u64) as f32;
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.on_key_release(ctx);

		if ctx.time.check_update_time(8) && self.can_move {
			for i in (0..self.snake_body.len()).rev() {
				let mut seg = self.snake_body[i].clone();

				self.snake_body[i] = if i == 0 {
					seg = self.return_if_border(seg);
					if self.is_close_to_head(&self.apple) {
						self.change_apple_pos();
						let body_len = self.snake_body.len();
						self.spawn_seg(self.snake_body[body_len - 1].clone());
					}
					self.move_head(seg, &self.head_direction)
				} else {
					if self.is_close_to_head(&seg) {
						self.can_move = false;
					}
					self.snake_body[i - 1].clone()
				}
			}
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
		let head_prefb = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), Vec2::new(0.0, 0.0), 10.0, 2.0, Color::GREEN)?;
		let body_prefb = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), Vec2::new(0.0, 0.0), 10.0, 2.0, Color::WHITE)?;
		let apple_prefb = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), Vec2::new(0.0, 0.0), 10.0, 2.0, Color::RED)?;

		for (i, seg) in self.snake_body.iter().enumerate() {
			if i == 0 {
				canvas.draw(&head_prefb, Vec2::new(seg.0, seg.1))
			} else {
				canvas.draw(&body_prefb, Vec2::new(seg.0, seg.1));
			}
		}
		canvas.draw(&apple_prefb, Vec2::new(self.apple.0, self.apple.1));
		canvas.finish(ctx)?;
		Ok(())
	}
}

pub fn main() -> GameResult {
	let cb = ggez::ContextBuilder::new("snake", "ggez");
	let (ctx, event_loop) = cb.build()?;
	let state = MainState::new()?;
	event::run(ctx, event_loop, state)
}
