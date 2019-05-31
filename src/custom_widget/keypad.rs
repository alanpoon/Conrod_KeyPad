use conrod_core::{widget, Labelable, Positionable, Sizeable, Widget, color, Borderable, Colorable};
use conrod_core::color::Color;
use conrod_core::UiCell;
use conrod_core::event;
use custom_widget::{keybut,wrap_list};
use load_svg::SvgKeypad;
use custom_widget::keybut::KeyButEnum;
use custom_widget::svg::{SvgWidget,SvgInfo};
use custom_widget::text_edit::{Ids, Style};
use std::cmp::min;
#[derive(Clone,Debug,PartialEq)]
pub enum KeyPadVariant {
    Num(usize), //1:page 1, 2:page2
    Letter(usize), //1:lowercase,2:uppercase
    None,
}
pub enum KeyPressType {
    Press,
    Hold,
}
pub enum BlankEnum {
    Flat,
    Svg(SvgInfo),
}
pub enum KeyVariant {
    Blank(f64, BlankEnum), //spacing multiply by width, used as spacing, KeyButEnum needs to be normalize
    StringOnly(String),
    StringHold(String, String),
    Closure(ClosureVariant, Box<Fn(&mut Vec<event::Widget>, &mut KeyPadVariant)>),
    Num(String, String),
    Spacebar(SvgInfo, String),
}
pub enum SvgOrString {
    Svg(SvgInfo,Color,Color),
    StringOnly([String; 2]),
}
pub enum ClosureVariant {
    EdgeRow3(SvgOrString),
    EdgeRow4(SvgOrString),
}

pub trait KeyButtonTrait {
    fn dimension(&self, Style) -> [f64; 2];
    fn get_variant(&self) -> &KeyVariant;
    fn process(&self, &mut Vec<event::Widget>, &mut KeyPadVariant, KeyPressType);
}

pub fn render_keypad<T>(master_id: widget::Id,
                        ui: &mut UiCell,
                        ids: Ids,
                        events: &mut Vec<event::Widget>,
                        keypad_variant: &mut KeyPadVariant,
                        meta_tuple: &(Vec<T>, Vec<T>, T),
                        style: &Style)
    where T: KeyButtonTrait
{

    let can = ui.rect_of(master_id).unwrap();
    let w_can = can.w();
    let h_can = can.h() * 0.3;
    let label_size = (min(w_can.floor() as i32, h_can.floor() as i32) as f32 * 0.17).floor() as u32;
    let style = style.normalize([w_can, h_can], &ui);
    let (k_hash, len) = match keypad_variant {
        &mut KeyPadVariant::Num(_) => (&meta_tuple.1, meta_tuple.1.len()),
        &mut KeyPadVariant::Letter(_) => (&meta_tuple.0, meta_tuple.1.len()), //lowercase
        _ => (&meta_tuple.1, 0usize),
    };
    if len > 0 {
        widget::Canvas::new()
            .mid_bottom_of(master_id)
            .w(w_can)
            .h(h_can)
            .set(ids.keyboard_canvas, ui);
    }

    let mut item0 = wrap_list::WrapList::new(len)
        .w_h(w_can, h_can)
        .top_left_of(ids.keyboard_canvas)
        .set(ids.keyboard, ui);
    let mut k_h_iter = k_hash.iter();
    while let (Some(item), Some(k_h)) = (item0.next(ui), k_h_iter.next()) {
        let mut _lstring = "".to_owned();
        let mut _sstring = "".to_owned();
        let y = match (k_h).get_variant() {
            &KeyVariant::StringOnly(ref _l) => {
                _lstring = _l.clone();
                match keypad_variant {
                    &mut KeyPadVariant::Letter(2) => {
                        _lstring = _lstring.to_uppercase();
                        KeyButEnum::Flat(keybut::Button::new()
                                             .label_font_size(label_size)
                                             .label(&_lstring))
                    }
                    _ => {
                        KeyButEnum::Flat(keybut::Button::new()
                                             .label_font_size(label_size)
                                             .label(&_lstring))
                    }
                }
            }
            &KeyVariant::StringHold(ref _l, ref _s) => {
                _lstring = _l.clone();
                _sstring = _s.clone();
                KeyButEnum::Flat(keybut::Button::new()
                                     .label_font_size(label_size)
                                     .label_with_superscript(&_lstring, &_sstring))
            }
            &KeyVariant::Spacebar(ref _i, _) => {
                let svg_widget = SvgWidget::new(_i.clone()).color(color::BLACK);
                KeyButEnum::Svg(keybut::Button::svg(svg_widget))
            }
            &KeyVariant::Closure(ref _cvariant, _) => {
                match _cvariant {
                    &ClosureVariant::EdgeRow3(ref _i_or_s) => {
                        match _i_or_s {
                            &SvgOrString::Svg(ref _iv,color1,color2) => {
                                match keypad_variant {
                                    &mut KeyPadVariant::Letter(a) => {
                                        let svg_widget = SvgWidget::new(_iv.clone()).color(color2);
                                        KeyButEnum::Svg(keybut::Button::svg(svg_widget))
                                    }
                                    _ => {
                                        let svg_widget = SvgWidget::new(_iv.clone()).color(color1);
                                        KeyButEnum::Svg(keybut::Button::svg(svg_widget))
                                    },
                                }
                            }
                            &SvgOrString::StringOnly(ref _lv) => {
                                match keypad_variant {
                                    &mut KeyPadVariant::Num(a) => {
                                        _lstring = _lv[a - 1].clone();
                                        KeyButEnum::Flat(keybut::Button::new()
                                                             .label_font_size(label_size)
                                                             .label(&_lstring))
                                    }
                                    _ => {
                                        KeyButEnum::Flat(keybut::Button::new()
                                                             .label_font_size(label_size))
                                    }
                                }
                            }
                        }
                    }
                    &ClosureVariant::EdgeRow4(ref _i_or_s) => {
                        match _i_or_s {
                            &SvgOrString::Svg(ref _iv,color1,_) => {
                                let svg_widget = SvgWidget::new(_iv.clone()).color(color1);
                                KeyButEnum::Svg(keybut::Button::svg(svg_widget))
                            }
                            &SvgOrString::StringOnly(ref _l) => {
                                _lstring = _l[0].clone();
                                KeyButEnum::Flat(keybut::Button::new()
                                                     .label_font_size(label_size)
                                                     .label(&_lstring))
                            }
                        }
                    }
                }
            }
            &KeyVariant::Num(ref numpad1, ref numpad2) => {
                if let &mut KeyPadVariant::Num(1) = keypad_variant {
                    _lstring = numpad1.clone();
                    KeyButEnum::Flat(keybut::Button::new()
                                         .label_font_size(label_size)
                                         .label(&_lstring))
                } else {
                    _lstring = numpad2.clone();
                    KeyButEnum::Flat(keybut::Button::new()
                                         .label_font_size(label_size)
                                         .label(&_lstring))
                }
            }

            &KeyVariant::Blank(ref _w_multipler, ref blankenum) => {
                match blankenum {
                    &BlankEnum::Flat => {
                        KeyButEnum::BlankFlat(_w_multipler.clone(),
                                              keybut::Button::new()
                                                  .label_font_size(label_size)
                                                  .label(&""))
                    }
                    &BlankEnum::Svg(ref _i) => {
                        let svg_widget = SvgWidget::new(_i.clone()).color(color::BLACK);
                        KeyButEnum::BlankSvg(_w_multipler.clone(),
                                               keybut::Button::svg(svg_widget))
                    }
                }
            }
        };
        match y {
            KeyButEnum::Flat(j) => {
                let jj = j.wh(k_h.dimension(style)).border_color(color::BLACK);
                let jk = item.set(jj, k_h.dimension(style)[0], ui);
                if jk.clone().was_hold() {
                    k_h.process(events, keypad_variant, KeyPressType::Hold);

                } else if jk.was_clicked() {
                    k_h.process(events, keypad_variant, KeyPressType::Press);
                }
            }
            KeyButEnum::Svg(j) => {
                let jj = j.wh(k_h.dimension(style)).border_color(color::BLACK);
                let jk = item.set(jj, k_h.dimension(style)[0], ui);
                if jk.clone().was_hold() {
                    k_h.process(events, keypad_variant, KeyPressType::Hold);
                } else if jk.was_clicked() {
                    k_h.process(events, keypad_variant, KeyPressType::Press);

                }
            }
            KeyButEnum::BlankFlat(_w_multipler, j) => {
                item.set(j, k_h.dimension(style)[0] * _w_multipler, ui);
            }
            KeyButEnum::BlankSvg(_w_multipler, j) => {
                let jj = j.w(k_h.dimension(style)[0] * _w_multipler).h(k_h.dimension(style)[1]);
                item.set(jj, k_h.dimension(style)[0] * _w_multipler, ui);
            }
        }

    }
    if len > 0 {
        if let &KeyVariant::Closure(ref _cvariant, ref _closure) = meta_tuple.2.get_variant() {
            if let &ClosureVariant::EdgeRow3(ref _i_or_s) = _cvariant {
                if let &SvgOrString::Svg(ref _ia,color1,_color2) = _i_or_s {
                    let svg_widget = SvgWidget::new(_ia.clone()).color(color1);
                    let jj = keybut::Button::svg(svg_widget)
                        .wh(meta_tuple.2.dimension(style))
                        .up_from(ids.keyboard_canvas, 0.0)
                        .set(ids.close_tab, ui);
                    if jj.was_clicked() {
                        meta_tuple.2.process(events, keypad_variant, KeyPressType::Press);
                    }
                }
            }
        }
    }
}
