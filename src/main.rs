use minifb::*;
use std::time::Instant;
use std::cmp::{max, min};
const WHITE : u32 = 16777215;
const BLACK : u32 = 0;

//window sizes
const W_WIDTH : usize = 1000;
const W_HEIGHT : usize = 600;
//paddle size
const P_WIDTH : usize = 10;
const P_HEIGHT : usize = 80;
//Ball radius (tho it's a square')
const B_SIZE : usize = 5;


fn main() {
    
    let mut paddle_pos = (P_HEIGHT, W_HEIGHT/2);
    
    let mut ball_pos: (f32, f32) = ((W_WIDTH/2) as f32, (W_HEIGHT/2) as f32);
    let mut ball_speed = (-0.001, 0.001, Instant::now());

    let mut window = Window::new("Pong!", W_WIDTH, W_HEIGHT, WindowOptions::default())
        .expect("minifb was unable to create a new window.");

    loop {
        //cleans the window
        let mut buffer: Vec<u32> = vec![BLACK; W_WIDTH * W_HEIGHT];

        draw_paddle(&mut buffer, &paddle_pos);
        draw_ball(&mut buffer, &ball_pos);

        window.update_with_buffer(&buffer, W_WIDTH, W_HEIGHT)
            .expect("minifb was unable to update the window.");
        
        //turn the y of the paddle into the y of the mouse
        window.get_mouse_pos(MouseMode::Pass).map(|mouse| paddle_pos.1 = mouse.1 as usize);

        //force bounds to prevent the paddle touching the outside of the screen
        paddle_pos.1 = min(max(paddle_pos.1, P_HEIGHT), W_HEIGHT-P_HEIGHT);
        
        update_ball(&mut ball_pos, &mut ball_speed);

    }
}

fn update_ball (pos: &mut (f32, f32), speed: &mut (f32, f32, std::time::Instant)) {
    let time = speed.2.elapsed().as_millis();   //time since last update in milliseconds
    pos.0 = pos.0 + (speed.0 * time as f32);
    pos.1 = pos.1 + (speed.1 * time as f32);
    
    if pos.0 < P_HEIGHT as f32 {
        pos.0 = P_HEIGHT as f32;
        speed.0 = -speed.0;
    } else if pos.0 > (W_WIDTH-P_HEIGHT) as f32 {
        pos.0 = (W_WIDTH-P_HEIGHT) as f32;
        speed.0 = -speed.0
    }
     if pos.1 < P_HEIGHT as f32 {
        pos.1 = P_HEIGHT as f32;
        speed.1 = -speed.1;
    } else if pos.1 > (W_HEIGHT-P_HEIGHT) as f32 {
        pos.1 = (W_HEIGHT-P_HEIGHT) as f32;
        speed.1 = -speed.1
    }   
}

fn draw_paddle (buffer: &mut Vec<u32>, pos: &(usize, usize)) {
    let xbound = P_WIDTH/2;
    let ybound = P_HEIGHT/2;

    for i in 1..P_HEIGHT {
        for j in 1..P_WIDTH {
            buffer[W_WIDTH*(i + pos.1 as usize - ybound) + j + pos.0 as usize - xbound] = WHITE;
        }
    }
}

fn draw_ball (buffer: &mut Vec<u32>, pos: &(f32, f32)) {
    let diameter = B_SIZE * 2;

    for i in 1..diameter {
        for j in 1..diameter {
            buffer[W_WIDTH*(i + pos.1 as usize - B_SIZE) + j + pos.0 as usize - B_SIZE] = WHITE;
        }
    }
}
