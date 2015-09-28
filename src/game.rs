use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventMap, Events};
use piston::input::{Event, Input, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use screens::GameScreens;
use std::cell::RefCell;
use std::rc::Rc;

pub type RcWindow = Rc<RefCell<GlutinWindow>>;

pub struct Game {
    screens: GameScreens,
    window: RcWindow,
    gl: GlGraphics
}

pub trait Update {
    fn update(&mut self, args: &UpdateArgs, window: RcWindow);
}

pub trait GameInput {
    fn input(&mut self, input: &Input);
}

pub trait Render {
    fn render(&mut self, render_args: &RenderArgs, graphics: &mut GlGraphics);
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event);
}

impl Game {
    pub fn new(opengl: OpenGL, window: GlutinWindow) -> Game {
        let window = Rc::new(RefCell::new(window));
        Game {
            screens: GameScreens::new(window.clone()),
            window: window,
            gl: GlGraphics::new(opengl)
        }
    }

    pub fn run_loop(&mut self) {
        for e in self.window.clone().events() {
            self.handle_event(&e);
            match e {
                Event::Render(r) => { self.render(&r); },
                Event::Update(u) => { self.update(&u); },
                Event::Input(i)  => { self.input(&i); },
                _ => { }
            }
        }
    }

    #[allow(dead_code)]
    fn terminate(&mut self) {
        // terminate the game, do cleanup, etc...
    }

    // Different signature because Game owns the GlGraphics
    fn render(&mut self, args: &RenderArgs) {
        self.screens.render(args, &mut self.gl);
    }

    fn input(&mut self, args: &Input) {
        self.screens.input(&args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.screens.update(args, self.window.clone());
    }
}

impl EventHandler for Game {
    fn handle_event(&mut self, e: &Event) {
        self.screens.handle_event(e);
    }
}