mod tilemap;
use tilemap::*;

use std::{
    thread,
    hash::Hash,
    time::Duration,
};

use sdl2::{
    rect::Rect,
    pixels::Color,
    keyboard::Scancode,
    mouse::MouseButton,
    render::WindowCanvas,
    event::Event, EventPump,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line<T> {
    pub x0: T, pub y0: T,
    pub x1: T, pub y1: T,
}

struct World {
    pub canvas: WindowCanvas,
    pub scale: usize,
    pub tilemap: Tilemap,
    #[allow(dead_code)]
    pub wallmap: Vec<Line<i32>>,
}

impl World {
    fn new(title: &str, width: usize, height: usize, scale: usize) -> (Self, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();

        // try to find an opengl compatible driver
        let mut driver_id = 0;
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                driver_id = index;
                break;
            }
        }

        let window = video.window(title, width as u32, height as u32)
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .index(driver_id as u32)
            .build()
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();
        let tilemap = Tilemap::new(width / scale, height / scale);

        (Self {
            scale,
            canvas,
            tilemap,
            wallmap: Vec::new(),
        }, event_pump)
    }

    fn update(&mut self) {
        self.tilemap.to_wallmap(self.scale, &mut self.wallmap);
    }

    fn render(&mut self) {
        let scale = self.scale as i32;

        for y in 0..self.tilemap.height {
            for x in 0..self.tilemap.width {
                match self.tilemap[[x, y]] {
                    0 => self.canvas.set_draw_color(Color::BLACK),
                    1 => self.canvas.set_draw_color(Color::WHITE),
                    _ => {},
                }

                self.canvas.fill_rect(Rect::new(
                    x as i32 * scale, y as i32 * scale,
                    scale as u32, scale as u32
                )).unwrap();
            }
        }

        self.canvas.present();
    }
}

fn main() {
    let (mut world, mut event_pump) =
        World::new("shadowcasting!", 640, 480, 10);

    let scale = 10;
    let mut m_x = 0;
    let mut m_y = 0;

    let mut lheld = false;
    let mut rheld = false;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { scancode: Some(Scancode::Q), .. } => break 'main,

                Event::MouseMotion { x, y, .. } => {
                    m_x = x;
                    m_y = y;
                }

                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    lheld = true;
                }

                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    lheld = false;
                }

                Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => {
                    rheld = true;
                }

                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => {
                    rheld = false;
                }

                _ => {}
            }
        }

        if lheld && !rheld {
            world.tilemap[[m_x as usize / scale, m_y as usize / scale]] = 1;
        }

        if rheld && !lheld {
            world.tilemap[[m_x as usize / scale, m_y as usize / scale]] = 0;
        }

        if lheld || rheld {
            world.update();
        }

        world.render();

        thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
