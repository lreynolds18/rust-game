extern crate ggez;

use std::time::Duration;
// use std::thread;
use ggez::event::*;
use ggez::{Context, conf, graphics, event, GameResult};

struct Scene {
    frames: usize,
    w_height: f32,
    w_width: f32,
    x: f32,
    y: f32,
}

impl Scene {
    fn new(_ctx: &mut Context, height: f32, width: f32) -> GameResult<Scene> {
        Ok(Scene {
            frames: 0,
            w_height: height,
            w_width: width,
            x: width / 2.0 - 50.0,
            y: height / 2.0 + 50.0,
        })
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        // println!("Key pressed: {:?}, modifier {:?}, repeat: {}",
        //          keycode,
        //          keymod,
        //          repeat);

        // interestinly event handler can't handle two keys being pressed at same time
        match keycode {
            Keycode::D | Keycode::Right => self.x += 10.0,
            Keycode::A | Keycode::Left  => self.x -= 10.0,
            Keycode::W | Keycode::Up    => self.y -= 10.0,
            Keycode::S | Keycode::Down  => self.y += 10.0,
            _ => (),
        };
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let bgrect = graphics::Rect::new(0.0, 0.0, self.w_width * 2.0, self.w_height * 2.0);
        graphics::set_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, bgrect)?;

        let smrect = graphics::Rect::new(self.x, self.y, 50.0, 50.0);
        graphics::set_color(ctx, graphics::Color::new(0.098, 0.098, 0.439, 1.0))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, smrect)?;

        graphics::present(ctx);

        self.frames += 1;
        if self.frames % 100 == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}
        
fn main() {
    // not sure about these declerations
    let height = 720;
    let width = 1280;

    let mut conf = conf::Conf::new();
    conf.window_title = String::from("Rusty Block");
    conf.window_height = height;
    conf.window_width = width;
    conf.vsync = true;

    let ctx = &mut Context::load_from_conf("rusty-block", "lreynolds18", conf).expect(
        "Could not load configurations"
    );

    let scene = &mut Scene::new(ctx, height as f32, width as f32).expect(
        "Could not create scene"                                       
    );

    graphics::set_screen_coordinates(ctx, 0.0, 1200.0, 0.0, 720.0).expect(
        "Could not set logical screen cordinates before running initial state."
    );

    match event::run(ctx, scene) {
        Ok(()) => (),
        Err(e) => {
            println!("Error is {}", e);
            std::process::exit(1);
        }
    }

}
