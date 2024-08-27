use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::rusty_boi::RustyBoi;

const UP_ADDRESS: u16 = 0x87FE;
const DOWN_ADDRESS: u16 = 0x87FD;
const LEFT_ADDRESS: u16 = 0x87FC;
const RIGHT_ADDRESS: u16 = 0x87FB;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 400;

pub fn run_vm(mut rusty_boi: RustyBoi) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Test", 600, 400)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        match key.to_string().as_str() {
                            "W" => rusty_boi.write_memory_byte(UP_ADDRESS, 1),
                            "A" => rusty_boi.write_memory_byte(LEFT_ADDRESS, 1),
                            "S" => rusty_boi.write_memory_byte(DOWN_ADDRESS, 1),
                            "D" => rusty_boi.write_memory_byte(RIGHT_ADDRESS, 1),
                            _ => (),
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        match key.to_string().as_str() {
                            "W" => rusty_boi.write_memory_byte(UP_ADDRESS, 0),
                            "A" => rusty_boi.write_memory_byte(LEFT_ADDRESS, 0),
                            "S" => rusty_boi.write_memory_byte(DOWN_ADDRESS, 0),
                            "D" => rusty_boi.write_memory_byte(RIGHT_ADDRESS, 0),
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        rusty_boi.run();

        println!("{:?}", rusty_boi.read_memory_word(0x8000));

        render(&mut canvas, &rusty_boi);
    }

    Ok(())
}

fn render(canvas: &mut Canvas<Window>, rusty_boi: &RustyBoi) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let screen_center = Point::new(
        canvas.window().size().0 as i32 / 2,
        canvas.window().size().1 as i32 / 2,
    );

    let game_rect = Rect::from_center(screen_center, SCREEN_WIDTH, SCREEN_HEIGHT);

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.set_viewport(game_rect);

    //canvas.draw_rect(game_rect).unwrap();
    let mut mem_index: usize = 0x87FF;
    for x in (0..SCREEN_WIDTH).step_by(5) {
        for y in (0..SCREEN_HEIGHT).step_by(5) {
            canvas.set_draw_color(Color::RGB(
                rusty_boi.memory[mem_index],
                rusty_boi.memory[mem_index + 1],
                rusty_boi.memory[mem_index + 2],
            ));
            canvas
                .fill_rect(Rect::new(x as i32, y as i32, 5, 5))
                .unwrap();
            mem_index += 3;
        }
    }

    canvas.present();
}
