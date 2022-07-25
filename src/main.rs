#![allow(dead_code)]
use minifb::{Window, MouseMode, WindowOptions};
use std::time::Instant;
use std::cmp::{max, min};
const WHITE : u32 = 16777215;
const BLACK : u32 = 0;

struct Options {
    w_width : usize,
    w_height : usize,
    p_width : usize,
    p_height : usize,
    b_size : usize,
}

struct Paddle {
    x: f32,
    y: f32,
    vy: f32,
    time: std::time::Instant
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    time: std::time::Instant
}

fn main() {

    let options = Options {
        //window sizes
        w_width : 1000,
        w_height : 600,

        //paddle size
        p_width : 10,
        p_height : 80,

        //ball radius
        b_size : 5,
    };

    let mut paddle1 = Paddle {
        x: (options.p_width*8) as f32,
        y: (options.p_height/2) as f32,
        vy: 0.0,
        time: Instant::now()
    };

    let mut paddle2 = Paddle {
        x: (options.w_width - options.p_width*8) as f32,
        y: (options.w_height/2) as f32,
        vy: 0.0,
        time: Instant::now(),
    };

    let mut ball = Ball {
        x: (options.w_width/2) as f32,
        y: (options.w_height/2) as f32,
        vx: -0.7,
        vy: 0.4,
        time: Instant::now()
    };

    let mut window = Window::new("Pong!", options.w_width, options.w_height, WindowOptions::default())
        .expect("Minifb was unable to create a new window.");
            
    let mut buffer: Vec<u32> = vec![BLACK; options.w_width*options.w_height];

    let mut fps = (0, Instant::now());

    loop {
        if fps.1.elapsed().as_millis() > 1000 {
            println!("Fps: {}", fps.0);
            fps.0 = 0;
            fps.1 = Instant::now();
        }

        draw_paddle(&mut buffer, &paddle1, &options, WHITE);
        draw_paddle(&mut buffer, &paddle2, &options, WHITE);
        draw_ball(&mut buffer, &ball, &options, WHITE);

        window.update_with_buffer(&buffer, options.w_width, options.w_height)
            .expect("Minifb was unable to update the window.");

        //erase the paddles and ball from the buffer
        draw_paddle(&mut buffer, &paddle1, &options, BLACK);
        draw_paddle(&mut buffer, &paddle2, &options, BLACK);
        draw_ball(&mut buffer, &ball, &options, BLACK);

        //turn the y of the paddle into the y of the mouse
        window.get_mouse_pos(MouseMode::Pass).map(|mouse| paddle1.y = mouse.1 as f32);

        //clamp the position of the paddle to the window
        paddle1.y = min(max(paddle1.y as usize, options.p_height/2), options.w_height - options.p_height/2) as f32;
    
        update_ball(&mut ball, &paddle1, &paddle2, &options);
        ball_paddle(&mut ball, &paddle1, &options);
        ball_paddle(&mut ball, &paddle2, &options);
        fps.0 += 1;
    }
}

fn update_ball(ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, options: &Options) {
    let time = ball.time.elapsed().as_millis() as f32;
    ball.x += ball.vx * time;
    ball.y += ball.vy * time;
    
    ball.time = Instant::now(); //refresh time
    
    let top_bound = options.b_size as f32;
    let bot_bound = (options.w_height-options.b_size) as f32;
    let left_bound = options.b_size as f32;
    let right_bound = (options.w_width-options.b_size) as f32;

    //reflect the ball if it hits a wall
    if ball.x < left_bound {
        ball.x = left_bound;
        ball.vx = -ball.vx;
    } else if ball.x > right_bound {
        ball.x = right_bound;
        ball.vx = -ball.vx;
    }

    if ball.y < top_bound {
        ball.y = top_bound;
        ball.vy = -ball.vy;
    } else if ball.y > bot_bound {
        ball.y = bot_bound;
        ball.vy = -ball.vy;
    }
    
}

//ball paddle collision
fn ball_paddle (ball: &mut Ball, paddle: &Paddle, options: &Options) {
    let xdistance = ball.x - paddle.x;
    let ydistance = ball.y - paddle.y;
    let xbound = (options.p_width/2 + options.b_size) as f32;
    let ybound = (options.p_height/2 + options.b_size*2) as f32;

    if -xbound < xdistance && xdistance < xbound {
        if -ybound < ydistance && ydistance < ybound {
            ball.vx = -ball.vx;

            if ball.vx < 0.0 {
                ball.x = paddle.x - xbound;
            } else {
                ball.x = paddle.x + xbound;
            }
        }
    }
}

fn draw_paddle (buffer: &mut Vec<u32>, paddle: &Paddle, options: &Options, color: u32) {
    let xbound = options.p_width/2;
    let ybound = options.p_height/2;

    for i in 1..options.p_height {
        for j in 1..options.p_width {
            let x = j + paddle.x as usize - xbound;
            let y = i + paddle.y as usize - ybound;

            buffer[options.w_width * y + x] = color;
        }
    }
}

fn draw_ball (buffer: &mut Vec<u32>, ball: &Ball, options: &Options, color: u32) {
    let diameter = options.b_size * 2;

    for i in 1..diameter {
        for j in 1..diameter {
            let x = j + ball.x as usize - options.b_size;
            let y = i + ball.y as usize - options.b_size;

            buffer[options.w_width * y + x] = color;
        }
    }
}
