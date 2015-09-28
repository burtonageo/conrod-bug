use game::{EventHandler, GameInput, RcWindow, Render, Update};
use opengl_graphics::GlGraphics;
use piston::input::{Event, Input, RenderArgs, UpdateArgs};
use std::any::Any;

pub type ArgVec = Vec<Box<Any>>;
pub type ScreenKey = usize;

pub trait GameScreen: Update + GameInput + Render + EventHandler {
    fn new(window: RcWindow) -> Self where Self: Sized;

    fn with_args(_window: RcWindow, _args: ArgVec) -> Option<Self> where Self: Sized {
        None
    }

    fn get_after_update_info(&self) -> Option<AfterUpdateInfo> {
        None
    }
}

pub struct AfterUpdateInfo {
    next_screen: ScreenKey,
    args: Option<ArgVec>
}

impl AfterUpdateInfo {
    fn new(optkey: Option<ScreenKey>, args: Option<ArgVec>) -> Option<Self> {
        match optkey {
            Some(scr) => Some(AfterUpdateInfo {
                                  next_screen: scr,
                                  args: args
                              }),
            None => None
        }
    }
}

pub struct GameScreens {
    inner: GameScreensInner,
}

impl GameScreens {
    pub fn new(win: RcWindow) -> GameScreens {
        GameScreens {
            inner: GameScreensInner::MainMenu(MenuScreen::new(win)),
        }
    }

    pub fn get_current_screen(&self) -> &GameScreen {
        self.inner.get_current_branch()
    }

    pub fn get_current_screen_mut(&mut self) -> &mut GameScreen {
        self.inner.get_current_branch_mut()
    }

    pub fn set_screen(&mut self, index: ScreenKey, window: RcWindow) {
        self.inner.set_branch(index, window);
    }

    pub fn set_screen_with_args(&mut self, index: ScreenKey, window: RcWindow, args: ArgVec) {
        self.inner.set_branch_with_args(index, window, args);
    }
}

impl EventHandler for GameScreens {
    fn handle_event(&mut self, e: &Event) {
        self.get_current_screen_mut().handle_event(e);
    }
}

impl GameInput for GameScreens {
    fn input(&mut self, iput: &Input) {
        self.get_current_screen_mut().input(iput);
    }
}

impl Update for GameScreens {
    fn update(&mut self, args: &UpdateArgs, window: RcWindow) {
        self.get_current_screen_mut().update(args, window.clone());

        if let Some(AfterUpdateInfo { next_screen, args }) = self.get_current_screen().get_after_update_info() {
            match args {
                Some(a) => self.set_screen_with_args(next_screen, window.clone(), a),
                None => self.set_screen(next_screen, window.clone())
            }
        }
    }
}

impl Render for GameScreens {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        self.get_current_screen_mut().render(args, gl);
    }
}

macro_rules! enum_map(
    ($nm:ident : $idx_ty:ty => $key_ty:ty {
        $(($key_nm:ident = $idx:expr) => $brnch:ident($mod_nm:ident :: $inner_ty:ident)),*
    }) => (
        $(pub const $key_nm: $idx_ty = $idx;)*
        enum_map!( $nm : $idx_ty => $key_ty {
            $($idx => $brnch($mod_nm :: $inner_ty)),*
        });
    );

    ($nm:ident : $idx_ty:ty => $key_ty:ty {
        $($idx:expr => $brnch:ident($mod_nm:ident :: $inner_ty:ident)),*
    }) => (
        $(mod $mod_nm;)*
        $(use self::$mod_nm::$inner_ty;)*

        enum $nm {
            $($brnch($inner_ty)),*
        }

        impl $nm {
            fn get_current_branch(&self) -> &$key_ty {
                match self {
                    $(&$nm::$brnch(ref inner) => &*inner as &$key_ty),*
                }
            }

            fn get_current_branch_mut(&mut self) -> &mut $key_ty {
                match self {
                    $(&mut $nm::$brnch(ref mut inner) => &mut *inner as &mut $key_ty),*
                }
            }

            fn set_branch_with_args(&mut self, index: $idx_ty, window: RcWindow, args: ArgVec) {
                match index {
                    $($idx => *self = $nm::$brnch($inner_ty::with_args(window, args).unwrap())),*
                    , _ => { }
                }
            }

            fn set_branch(&mut self, index: $idx_ty, window: RcWindow) {
                match index {
                    $($idx => *self = $nm::$brnch($inner_ty::new(window))),*
                    , _ => { }
                }
            }
        }
    );
);

enum_map!( GameScreensInner: ScreenKey => GameScreen {
    (MENU_SCREEN_KEY = 0)      => MainMenu(menuscreen::MenuScreen),
    (OVERWORLD_SCREEN_KEY = 1) => Overworld(overworld::OverworldScreen)
});