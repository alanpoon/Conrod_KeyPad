#[no_mangle]
pub  extern "C" fn get_keyboard_styles(screen_dim:[f64;2])->KeyButtonStyle{
    KeyButtonStyle{
      normal:([screen_dim[0]*0.1,screen_dim[1]*0.25],14),
      num:([screen_dim[0]*0.1,screen_dim[1]*0.25],15),
      edge_row3:([screen_dim[0]*0.15,screen_dim[1]*0.25],14),
      edge_row4:([screen_dim[0]*0.35*0.5,screen_dim[1]*0.25],14),
      spacebar:[screen_dim[0]*0.55,screen_dim[1]*0.25]
        }
}
#[no_mangle]
pub  extern "C" fn testreload()->i32{
  20
}
#[no_mangle]
pub  extern "C" fn get_spriteinfo()->SpriteInfo{
  SpriteInfo{
    first: (0.0, 400.0),
    num_in_row: 3.0,
    w_h: (200.0, 200.0),
    pad: (0.0, 0.0, 0.0, 0.0),
  }
}
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct KeyButtonStyle {
    pub normal: ([f64; 2],u32), //([width,height],fontsize)
    pub num:([f64;2],u32),
    pub edge_row3: ([f64; 2],u32),
    pub edge_row4: ([f64; 2],u32), //caps,backspace,num 1/3 
    pub spacebar: [f64; 2],
}

#[derive(Clone,Copy,PartialEq,Debug)]
pub struct SpriteInfo {
    pub first: (f64, f64), //left corner of first
    pub num_in_row: f64,
    pub w_h: (f64, f64),
    pub pad: (f64, f64, f64, f64),
}
