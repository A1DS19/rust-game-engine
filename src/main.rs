use ggez::{
    event,
    glam::*,
    graphics::{self, Color, Rect},
    Context, GameResult,
};

struct MainState {
    left_paddle_x: f32,
    left_paddle_y: f32,
    left_paddle: graphics::Mesh,

    right_paddle_x: f32,
    right_paddle_y: f32,
    right_paddle: graphics::Mesh,

    ball_pos_x: f32,
    ball: graphics::Mesh,
}

const WINDOW_DIMENSIONS: (f32, f32) = (800.0, 600.0);
const PADDLE_MARGIN: f32 = 10.0;
const BALL_SIZE: f32 = 10.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;

trait MeshExt {
    fn create_rectangle(ctx: &mut Context, bounds: Rect) -> Self;
}

impl MeshExt for graphics::Mesh {
    fn create_rectangle(ctx: &mut Context, bounds: Rect) -> Self {
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bounds, Color::WHITE)
            .unwrap()
    }
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let left_paddle_y: f32 = 0.0;
        let right_paddle_y: f32 = 0.0;
        let ball_pos_x: f32 = 0.0;

        let left_paddle_x: f32 = PADDLE_MARGIN;
        let right_paddle_x: f32 = WINDOW_DIMENSIONS.0 - PADDLE_MARGIN - 20.0;

        let left_paddle = graphics::Mesh::create_rectangle(
            ctx,
            Rect::new(left_paddle_x, left_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT),
        );

        let right_paddle = graphics::Mesh::create_rectangle(
            ctx,
            Rect::new(right_paddle_x, right_paddle_y, PADDLE_WIDTH, PADDLE_HEIGHT),
        );

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(WINDOW_DIMENSIONS.0 / 2.0, WINDOW_DIMENSIONS.1 / 2.0),
            BALL_SIZE,
            2.0,
            Color::WHITE,
        )?;

        Ok(MainState {
            left_paddle_x,
            right_paddle_x,
            left_paddle_y,
            right_paddle_y,
            ball_pos_x,
            left_paddle,
            right_paddle,
            ball,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.ball_pos_x += 0.5;

        println!("ball_pos_x: {}", self.ball_pos_x);

        if self.ball_pos_x >= 380.0 {
            self.ball_pos_x -= 0.5;
        }

        if self.ball_pos_x >= WINDOW_DIMENSIONS.0 || self.ball_pos_x < 0.0 {
            self.ball_pos_x = 0.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.left_paddle, Vec2::new(0.0, self.left_paddle_y));
        canvas.draw(&self.right_paddle, Vec2::new(0.0, self.right_paddle_y));
        canvas.draw(&self.ball, Vec2::new(self.ball_pos_x, 0.0));

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        match input.keycode {
            // Handle keys for left paddle
            Some(ggez::input::keyboard::KeyCode::W) => {
                println!("W pressed, left_paddle_y: {}", self.left_paddle_y);
                self.left_paddle_y -= 10.0;
            }
            Some(ggez::input::keyboard::KeyCode::S) => {
                println!("S pressed, left_paddle_y: {}", self.left_paddle_y);
                self.left_paddle_y += 10.0;
            }
            // Handle keys for right paddle
            Some(ggez::input::keyboard::KeyCode::Up) => {
                println!("Up pressed, right_paddle_y: {}", self.right_paddle_y);
                self.right_paddle_y -= 10.0;
            }
            Some(ggez::input::keyboard::KeyCode::Down) => {
                println!("Down pressed, right_paddle_y: {}", self.right_paddle_y);
                self.right_paddle_y += 10.0;
            }
            _ => {}
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong", "jp")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong!"))
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(WINDOW_DIMENSIONS.0, WINDOW_DIMENSIONS.1),
        );

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
