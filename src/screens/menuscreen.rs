#![allow(unused_imports)]

use apply::Apply;
use conrod::{
    Background,
    Color,
    Colorable,
    Frameable,
    Labelable,
    Positionable,
    Sizeable,
    Theme,
    Toggle,
    Ui,
    Widget
};
use conrod::color::blue;
use find_folder;
use game::{EventHandler, Update, GameInput, RcWindow, Render};
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::{Event, Input, RenderArgs, UpdateArgs};
use screens::{AfterUpdateInfo, GameScreen, ScreenKey, OVERWORLD_SCREEN_KEY};

pub struct MenuScreen {
    background_color: Color,
    ui: Ui<GlyphCache<'static>>,
    next_screen: Option<ScreenKey>
}

impl GameScreen for MenuScreen {
    fn new(_: RcWindow) -> Self where Self: Sized {
        let ui = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .expect("Could not find assets folder")
            .join("fonts/NotoSans/NotoSans-Regular.ttf")
            .apply(|font_path| GlyphCache::new(&font_path))
            .expect("Could not find font file within assets folder")
            .apply(|glyph_cache| Ui::new(glyph_cache, Theme::default()));

        MenuScreen {
            background_color: blue(),
            ui: ui,
            next_screen: None
        }
    }

    fn get_after_update_info(&self) -> Option<AfterUpdateInfo> {
        AfterUpdateInfo::new(self.next_screen, None)
    }
}

impl EventHandler for MenuScreen {
    fn handle_event(&mut self, e: &Event) { self.ui.handle_event(e); }
}

impl GameInput for MenuScreen {
    fn input(&mut self, args: &Input) {
        use piston::input::Button;
        use piston::input::keyboard::Key;
        if let &Input::Press(Button::Keyboard(Key::Space)) = args {
            self.next_screen = Some(OVERWORLD_SCREEN_KEY);
        }
    }
}

impl Update for MenuScreen {
    fn update(&mut self, _: &UpdateArgs, _: RcWindow) { }
}

impl Render for MenuScreen {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, gl| {
            Background::new().color(self.background_color).set(&mut self.ui);

            // draw widgets here...

            self.ui.draw(c, gl);
        });
    }
}