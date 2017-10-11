use libloading::Library;
use std;
pub struct Application(pub Library);

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct KeyButtonStyle {
    pub normal: ([f64; 2],u32), //([width,height],fontsize)
    pub num:([f64;2],u32),
    pub edge_row3: ([f64; 2],u32),
    pub edge_row4: ([f64; 2],u32), //caps,backspace,num 1/3
    pub enter: [f64; 2], 
    pub spacebar: [f64; 2],
}

impl Application {
    pub fn new(libpath: &'static str) -> Application {
        Application(Library::new(libpath).unwrap_or_else(|error| panic!("{}", error)))
    }
    pub fn in_loop(&mut self, libpath: &'static str, last_modified: &mut std::time::SystemTime) {
        if let Ok(Ok(modified)) = std::fs::metadata(libpath).map(|m| m.modified()) {
            if modified > *last_modified {
                drop(self);
                *last_modified = modified;
                Application(Library::new(libpath).unwrap_or_else(|error| panic!("{}", error)));
            }
        }
    }
    pub fn get_keyboard_styles(&self) -> KeyButtonStyle {
        unsafe {
            let f = self.0.get::<fn() -> KeyButtonStyle>(b"get_keyboard_styles\0").unwrap();
            f()
        }
    }
    pub fn get_spriteinfo(&self)->SpriteInfo{
        unsafe{
            println!("j");
            let f = self.0.get::<fn() -> SpriteInfo>(b"get_spriteinfo\0").unwrap();
            println!("k");
            f()
        }
    }
}
use conrod::position;
use conrod::Rect;
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct SpriteInfo {
    pub first: (f64, f64), //left corner of first
    pub num_in_row: f64,
    pub w_h: (f64, f64),
    pub pad: (f64, f64, f64, f64),
}
impl SpriteInfo {
    pub fn src_rect(&self, index: f64) -> Rect {
        let s = self;
        let (x, y) = (index % s.num_in_row as f64, (index / (s.num_in_row)).floor());
        let r = position::rect::Rect::from_corners([s.first.0 + x * s.w_h.0 + s.pad.0,
                                                    s.first.1 - y * s.w_h.1 - s.pad.2],
                                                   [s.first.0 + (x + 1.0) * s.w_h.0 - s.pad.1,
                                                    s.first.1 - (y + 1.0) * s.w_h.1 + s.pad.3]);
        r
    }
}
