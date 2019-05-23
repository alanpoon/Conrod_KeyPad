use conrod_core::position;
use conrod_core::Rect;
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct SpriteInfo {
    pub first: (f64, f64), //left corner of first
    pub num_in_row: f64,
    pub w_h: (f64, f64),
    pub pad: (f64, f64, f64, f64),
}
pub fn get_spriteinfo() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 600.0),
        num_in_row: 3.0,
        w_h: (200.0, 200.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
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
