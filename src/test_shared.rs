#[no_mangle]
pub  extern "C" fn get_keyboard_styles()->KeyButtonStyle{
    KeyButtonStyle{
      normal:([20.0,50.0],14),
      num:([20.0,50.0],15),
      edge_row3:([60.0,50.0],14),
      edge_row4:([70.0,50.0],14),
      enter:[80.0,50.0],
      spacebar:[200.0,50.0]
        }
}
#[no_mangle]
pub  extern "C" fn get_spriteinfo()->SpriteInfo{
  SpriteInfo{
    first: (0.0, 535.0),
    num_in_row: 1.0,
    w_h: (40.0, 40.0),
    pad: (10.0, 10.0, 0.0, 0.0),
  }
}
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct KeyButtonStyle {
    pub normal: ([f64; 2],u32), //([width,height],fontsize)
    pub num:([f64;2],u32),
    pub edge_row3: ([f64; 2],u32),
    pub edge_row4: ([f64; 2],u32), //caps,backspace,num 1/3
    pub enter: [f64; 2], 
    pub spacebar: [f64; 2],
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct SpriteInfo {
    pub first: (f64, f64), //left corner of first
    pub num_in_row: f64,
    pub w_h: (f64, f64),
    pub pad: (f64, f64, f64, f64),
}
