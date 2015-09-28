use conrod::Color;
use conrod::color::{red, green};
use game::{EventHandler, Update, GameInput, RcWindow, Render};
use na;
use na::{Translate, Pnt2, Vec2};
use opengl_graphics::GlGraphics;
use piston::input::{Event, Input, RenderArgs, UpdateArgs};
use piston::window::{Size, Window};
use rebind::{Action, InputTranslator, RebindBuilder, Translated};
use screens::{AfterUpdateInfo, GameScreen, ScreenKey, MENU_SCREEN_KEY};

pub struct OverworldScreen {
    hero: Hero,
    bg_color: Color,
    next_screen: Option<ScreenKey>
}

impl GameScreen for OverworldScreen {
    fn new(window: RcWindow) -> Self where Self: Sized {
        OverworldScreen {
            bg_color: green(),
            hero: Hero::new(
                window.borrow().draw_size(),
                red(),
                Pnt2::new(0.0, 0.0),
                50.0),
            next_screen: None
        }
    }

    fn get_after_update_info(&self) -> Option<AfterUpdateInfo> {
        AfterUpdateInfo::new(self.next_screen, None)
    }
}

impl EventHandler for OverworldScreen {
    fn handle_event(&mut self, _e: &Event) {
    }
}

impl GameInput for OverworldScreen {
    fn input(&mut self, args: &Input) {
        use piston::input::Button;
        use piston::input::keyboard::Key;
        self.hero.input(args);
        if let &Input::Press(Button::Keyboard(Key::Space)) = args {
            self.next_screen = Some(MENU_SCREEN_KEY);
        }
    }
}

impl Update for OverworldScreen {
    fn update(&mut self, args: &UpdateArgs, window: RcWindow) {
        self.hero.update(args, window);
    }
}

impl Render for OverworldScreen {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::clear;
        gl.draw(args.viewport(), |_, gl| clear(self.bg_color.to_fsa(), gl));
        self.hero.render(args, gl);
    }
}

trait Entity: Render + Update + GameInput {
    fn update(&mut self, _: &UpdateArgs) { }
    fn input(&mut self, _: &Input) { }
    fn render(&mut self, _: &RenderArgs, _: &mut GlGraphics) { }
}

struct Hero {
    input_map: InputTranslator<HeroAction>,
    color: Color,
    topleft: Pnt2<f64>,
    curr_velocity: Vec2<f64>,
    max_velocity: Vec2<f64>,
    size: f64,
    rotation: f64
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum HeroAction {
    MoveUp,
    MoveLeft,
    MoveRight,
    MoveDown
}

impl Action for HeroAction {}

impl Hero {
    fn new(win_size: Size, col: Color, tl: Pnt2<f64>, sz: f64) -> Hero {
        use piston::input::keyboard::Key;
        use piston::input::Button::Keyboard;
        let builder = RebindBuilder::new(win_size)
            .with_action_mapping(Keyboard(Key::Up),    HeroAction::MoveUp)
            .with_action_mapping(Keyboard(Key::W),     HeroAction::MoveUp)
            .with_action_mapping(Keyboard(Key::Down),  HeroAction::MoveDown)
            .with_action_mapping(Keyboard(Key::S),     HeroAction::MoveDown)
            .with_action_mapping(Keyboard(Key::Left),  HeroAction::MoveLeft)
            .with_action_mapping(Keyboard(Key::A),     HeroAction::MoveLeft)
            .with_action_mapping(Keyboard(Key::Right), HeroAction::MoveRight)
            .with_action_mapping(Keyboard(Key::D),     HeroAction::MoveRight);

        Hero {
            input_map: builder.into(),
            color: col,
            topleft: tl,
            curr_velocity: na::zero(),
            max_velocity: Vec2::new(5.0, 5.0),
            size: sz,
            rotation: 0.0
        }
    }
}

impl Render for Hero {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        let square = rectangle::square(self.topleft[0], self.topleft[1], self.size);
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y)
                                       .rot_rad(self.rotation)
                                       .trans(-(self.size / 2.0), -(self.size / 2.0));

            rectangle(self.color.to_fsa(), square, transform, gl);
        });
    }
}

impl Update for Hero {
    fn update(&mut self, args: &UpdateArgs, _: RcWindow) {
        use std::ops::Neg;
        fn clamp_velocity<F>(vel: &Vec2<F>, max: &Vec2<F>) -> Vec2<F>
            where F: na::BaseFloat +
                     PartialOrd +
                     Neg {
            use na::clamp;
            Vec2::new(clamp(vel[0], max[0].neg(), max[0]),
                      clamp(vel[1], max[1].neg(), max[1]))
        }

        self.topleft = clamp_velocity(&(self.curr_velocity * args.dt), &self.max_velocity)
                           .translate(&self.topleft);
    }
}

impl GameInput for Hero {
    fn input(&mut self, iput: &Input) {
        if let Some(t) = self.input_map.translate(iput) {
            match t {
                Translated::Press(act) => {
                    const VELOCITY_INCREMENT: f64 = 500.0;
                    match act {
                        HeroAction::MoveUp => {
                            self.curr_velocity[1] -= VELOCITY_INCREMENT;
                        },
                        HeroAction::MoveDown => {
                            self.curr_velocity[1] += VELOCITY_INCREMENT;
                        },
                        HeroAction::MoveLeft => {
                            self.curr_velocity[0] -= VELOCITY_INCREMENT;
                        },
                        HeroAction::MoveRight => {
                            self.curr_velocity[0] += VELOCITY_INCREMENT;
                        }
                    }
                },
                Translated::Release(_) => { self.curr_velocity = na::zero(); },
                _ => { }
            }
        }
    }
}