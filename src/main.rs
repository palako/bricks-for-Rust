use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BRICKS_PADDING_X: f32 = 25.0;
const BRICKS_PADDING_Y: f32 = 25.0;

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

struct GameState {
    paddle: Entity,
    ball: Entity,
    bricks: Vec<Entity>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position:Vec2<f32>, velocity:Vec2<f32>) -> Entity {
        Entity {texture, position, velocity}
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }
    
    fn height(&self) -> f32 {
        self.texture.height() as f32
    }
    
    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn center(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}

impl State for GameState {
    
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.paddle.texture, self.paddle.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);
        for v in &self.bricks {
            graphics::draw(ctx, &v.texture, v.position);
        }
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::Left) {
            self.paddle.position.x -= PADDLE_SPEED;
        }
    
        if input::is_key_down(ctx, Key::Right) {
            self.paddle.position.x += PADDLE_SPEED;
        }
        
        self.ball.position += self.ball.velocity;

        let paddle_bounds = self.paddle.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&paddle_bounds) {
            Some(&self.paddle)
        } else {
            None
        };

        let brick_hit = self.bricks.iter().find(|brick| ball_bounds.intersects(&brick.bounds()));

        if let Some(p) = paddle_hit {
            //let offset = (p.center().x - self.ball.center().x) / p.width();
            //self.ball.velocity.x += PADDLE_SPIN * -offset;
            self.ball.position.y = self.paddle.position.y - self.ball.height();
            self.ball.velocity.y = -self.ball.velocity.y;
            
        }

        if self.ball.position.x <= 0.0 || self.ball.position.x + self.ball.width() >= WINDOW_WIDTH {
            self.ball.velocity.x = -self.ball.velocity.x;
        }
        
        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
            self.ball.velocity.y = -self.ball.velocity.y;
        }


        if let Some(b) = brick_hit {
            //Some reading that explains what this code is doing
            //https://learnopengl.com/In-Practice/2D-Game/Collisions/Collision-resolution
            
            //brick to ball direction
            let b_to_b = Vec2::new(b.center().x - self.ball.center().x, b.center().y - self.ball.center().y);
            
            //angles will hold four values with the angles of the b_to_b vector against the four axis
            let angles: Vec<f32> = vec![Vec2::new(0.0,1.0), Vec2::new(0.0,-1.0), Vec2::new(1.0,0.0), Vec2::new(-1.0, 0.0)]
                .into_iter().map(|v| b_to_b.angle_between(v).to_degrees()).collect();
            
            //index of the biggest element in the vector
            let max = angles.iter().enumerate().max_by(|(_,a),(_, b)| a.partial_cmp(b).unwrap()).map(|(index, _)| index ).unwrap();
            match max {
                0 => {
                    //bottom 
                    self.ball.velocity.y = -self.ball.velocity.y;
                    self.ball.position.y = b.position.y + b.height();
                },
                1 => {
                    //top
                    self.ball.velocity.y = -self.ball.velocity.y;
                    self.ball.position.y = b.position.y - self.ball.height();
                },
                2 => {
                    //left
                    self.ball.velocity.x = -self.ball.velocity.x;
                    self.ball.position.x = b.position.x + b.width();
                },
                3 => {
                    //right
                    self.ball.velocity.x = -self.ball.velocity.x;
                    self.ball.position.x = b.position.x - self.ball.width();
                },
                _ => {}
            }
        }
        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        
        //paddle
        let paddle_texture = Texture::new(ctx, "./resources/paddleBlu.png")?;
        let paddle_position = Vec2::new(
            (WINDOW_WIDTH - paddle_texture.width() as f32) / 2.0 ,
            WINDOW_HEIGHT - paddle_texture.height() as f32 -  16.0
        );

        //ball
        let ball_texture = Texture::new(ctx, "./resources/ballBlue.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT - 20.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(BALL_SPEED, -BALL_SPEED);

        
        let brick_texture = Texture::new(ctx, "./resources/element_blue_rectangle.png")?;
        let mut bricks:Vec<Entity> = Vec::new();
        for j in 0..3 {
            for i in 0..9 {
                let brick_texture = brick_texture.clone();
                let brick_position = Vec2::new(
                    BRICKS_PADDING_X+(i*brick_texture.width()) as f32, 
                    BRICKS_PADDING_Y+(j*brick_texture.height()) as f32);
                bricks.push(Entity::new(brick_texture, brick_position));
            }
    }

        Ok(GameState {
            paddle: Entity::new(paddle_texture, paddle_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            bricks: bricks,
        })
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Bricks", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
