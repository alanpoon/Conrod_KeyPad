use custom_widget::keypad::{KeyVariant, KeyPadVariant, ClosureVariant, KeyButtonTrait,
                            ImageOrString, KeyPressType, BlankEnum};
use custom_widget::text_edit::Cursor;
use conrod;
use conrod::widget;
use conrod::widget::primitive::image::Image;
use conrod::text::cursor::{index_before_char, Index};
use conrod::text::line::Info;
use conrod::text;
use std;
#[cfg(feature="hotload")]
use dyapplication as application;
use application::SpriteInfo;
pub struct KeyButton(KeyVariant);
impl KeyButtonTrait for KeyButton {
    fn dimension(&self, static_style: application::KeyButtonStyle) -> conrod::position::Dimensions {
        match &self.0 {
            &KeyVariant::StringOnly(_) => static_style.normal.0,
            &KeyVariant::Num(_, _) => static_style.num.0,
            &KeyVariant::StringHold(_, _) => static_style.normal.0,
            &KeyVariant::Spacebar(_, _) => static_style.spacebar,
            &KeyVariant::Blank(_, _) => static_style.normal.0,
            &KeyVariant::Closure(ref a, _) => {
                match a {
                    &ClosureVariant::EdgeRow3(_) => static_style.edge_row3.0,
                    &ClosureVariant::EdgeRow4(_) => static_style.edge_row4.0,
                }
            }
        }
    }
    fn get_variant(&self) -> &KeyVariant {
        &self.0
    }
    fn process(&self,
               te: &mut std::borrow::Cow<str>,
               keypadvariant: &mut KeyPadVariant,
               cursor: &mut Cursor,
               lineinfo: &Vec<Info>,
               keypresstype: KeyPressType) {
        let mut tstring = "".to_owned();
        match self.get_variant() {
            &KeyVariant::StringOnly(ref s) => {
                match keypadvariant {
                    &mut KeyPadVariant::Letter(2) => {
                        tstring.push_str(s);
                        tstring = tstring.to_uppercase();
                        te.to_mut().push_str(&tstring);
                    }
                    _ => {
                        te.to_mut().push_str(s);
                    }
                }
            }
            &KeyVariant::Num(ref s1, ref s2) => {
                match keypadvariant {
                    &mut KeyPadVariant::Num(1) => {
                        te.to_mut().push_str(s1);
                    }
                    &mut KeyPadVariant::Num(2) => {
                        te.to_mut().push_str(s2);
                    }
                    _ => {}
                }
            }
            &KeyVariant::Spacebar(_, ref _l) => {
                te.to_mut().push_str(_l);
            }
            &KeyVariant::StringHold(ref s1, ref s2) => {
                match keypresstype {
                    KeyPressType::press => {
                        te.to_mut().push_str(s1);
                    }
                    KeyPressType::hold => {
                        te.to_mut().push_str(s2);
                    }
                }
            }
            &KeyVariant::Blank(_, _) => {}
            &KeyVariant::Closure(_, ref c) => {
                println!("clicked");
                (c)(te, keypadvariant, cursor, lineinfo);
            }
        }
    }
}
pub fn populate(image_id: conrod::image::Id,
                spriteinfo: SpriteInfo)
                -> (Vec<KeyButton>, Vec<KeyButton>, KeyButton) {
    //(letter_vec,number_vec,image_button_for_closetab)
    let images: [Image; 6] =
        [widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(0.0)), //black up
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(1.0)), //green up
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(2.0)), //backspace
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(3.0)), //space
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(4.0)), //enter
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(5.0))]; //closetab
    let letter_vec = vec![KeyButton(KeyVariant::StringHold(String::from("q"), String::from("1"))),
    KeyButton(KeyVariant::StringHold(String::from("w"),String::from("2"))),
    KeyButton(KeyVariant::StringHold(String::from("e"),String::from("3"))),
    KeyButton(KeyVariant::StringHold(String::from("r"),String::from("4"))),
    KeyButton(KeyVariant::StringHold(String::from("t"),String::from("5"))),
    KeyButton(KeyVariant::StringHold(String::from("y"),String::from("6"))),
    KeyButton(KeyVariant::StringHold(String::from("u"),String::from("7"))),
    KeyButton(KeyVariant::StringHold(String::from("i"),String::from("8"))),
    KeyButton(KeyVariant::StringHold(String::from("o"),String::from("9"))),
    KeyButton(KeyVariant::StringHold(String::from("p"),String::from("0"))),
    KeyButton(KeyVariant::Blank(0.5,BlankEnum::flat)), //10
    KeyButton(KeyVariant::StringOnly(String::from("a"))),
    KeyButton(KeyVariant::StringOnly(String::from("s"))),
    KeyButton(KeyVariant::StringOnly(String::from("d"))),
    KeyButton(KeyVariant::StringOnly(String::from("f"))),
    KeyButton(KeyVariant::StringOnly(String::from("g"))),
    KeyButton(KeyVariant::StringOnly(String::from("h"))),
    KeyButton(KeyVariant::StringOnly(String::from("j"))),
    KeyButton(KeyVariant::StringOnly(String::from("k"))),
    KeyButton(KeyVariant::StringOnly(String::from("l"))),
    KeyButton(KeyVariant::Blank(0.5,BlankEnum::flat)), //20
    KeyButton(KeyVariant::Blank(0.0,BlankEnum::flat)), //21
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image([images[0],images[1]])),Box::new(|_,kpv,_,_|{match kpv{
        &mut KeyPadVariant::Letter(1)=>{
            *kpv= KeyPadVariant::Letter(2);
        },&mut KeyPadVariant::Letter(2)=>{
            *kpv = KeyPadVariant::Letter(1);
        },_=>{}
    }}))), //22
    KeyButton(KeyVariant::StringOnly(String::from("z"))),
    KeyButton(KeyVariant::StringOnly(String::from("x"))),
    KeyButton(KeyVariant::StringOnly(String::from("c"))),
    KeyButton(KeyVariant::StringOnly(String::from("v"))),
    KeyButton(KeyVariant::StringOnly(String::from("b"))),
    KeyButton(KeyVariant::StringOnly(String::from("n"))),
    KeyButton(KeyVariant::StringOnly(String::from("m"))),
    //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image([images[2],images[2]])),Box::new(|te,_,cursor,lineinfo|{
      cursor_start_end(cursor,lineinfo,te,None,Box::new(|cursor_idx,lineinfo|(cursor_idx,cursor_idx.previous(lineinfo.iter().cloned()).unwrap_or(cursor_idx))));
        }))), //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::StringOnly([String::from("?123"),String::from("?123")])),Box::new(|te,kpv,_,_|{*kpv = KeyPadVariant::Num(1);}))),
    KeyButton(KeyVariant::Spacebar(images[3],String::from(" "))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image([images[4],images[4]])),Box::new(|te,_,cursor,lineinfo|{
         cursor_start_end(cursor,lineinfo,te,Some("\n".to_owned()),Box::new(|cursor_idx,_|(cursor_idx,cursor_idx)));
    })))
    ];

    let number_vec = vec![KeyButton(KeyVariant::Num(String::from("1"), String::from("~"))), 
    KeyButton(KeyVariant::Num(String::from("2"),String::from("="))),
    KeyButton(KeyVariant::Num(String::from("3"),String::from("_"))),
    KeyButton(KeyVariant::Num(String::from("4"),String::from("<"))),
    KeyButton(KeyVariant::Num(String::from("5"),String::from(">"))),
    KeyButton(KeyVariant::Num(String::from("6"),String::from("{"))),
    KeyButton(KeyVariant::Num(String::from("7"),String::from("}"))),
    KeyButton(KeyVariant::Num(String::from("8"),String::from("["))),
    KeyButton(KeyVariant::Num(String::from("9"),String::from("]"))),
    KeyButton(KeyVariant::Num(String::from("0"),String::from("|"))),
    KeyButton(KeyVariant::Blank(0.0,BlankEnum::flat)), //10
    KeyButton(KeyVariant::Num(String::from("@"),String::from("$"))),
    KeyButton(KeyVariant::Num(String::from("#"),String::from("€"))),
    KeyButton(KeyVariant::Num(String::from("%"),String::from("£"))),
    KeyButton(KeyVariant::Num(String::from("&"),String::from("¥"))),
    KeyButton(KeyVariant::Num(String::from("*"),String::from("¢"))),
    KeyButton(KeyVariant::Num(String::from("/"),String::from("₩"))),
    KeyButton(KeyVariant::Num(String::from("-"),String::from("§"))),
    KeyButton(KeyVariant::Num(String::from("+"),String::from("^"))),
    KeyButton(KeyVariant::Num(String::from("("),String::from("`"))),
    KeyButton(KeyVariant::Num(String::from(")"),String::from("∘"))),//20
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::StringOnly([String::from("1/3"),String::from("2/3")])),Box::new(|te,kpv,_,_|{
        match kpv{
            &mut KeyPadVariant::Num(1)=>{
                *kpv = KeyPadVariant::Num(2);
            },
            &mut KeyPadVariant::Num(2)=>{
                *kpv = KeyPadVariant::Num(1);
            },
            _=>{}
        }
    }))), //21
    KeyButton(KeyVariant::Blank(0.0,BlankEnum::image(images[0]))), //22
    KeyButton(KeyVariant::Num(String::from("?"),String::from("¿"))),
    KeyButton(KeyVariant::Num(String::from("!"),String::from("¡"))),
    KeyButton(KeyVariant::Num(String::from("\""),String::from("\\"))),
    KeyButton(KeyVariant::Num(String::from("\'"),String::from("<<"))),
    KeyButton(KeyVariant::Num(String::from(":"),String::from(">>"))),
    KeyButton(KeyVariant::Num(String::from(";"),String::from("®"))),
    KeyButton(KeyVariant::Num(String::from(","),String::from("©"))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image([images[2],images[2]])),Box::new(|te,_,cursor,lineinfo|{
      cursor_start_end(cursor,lineinfo,te,None,Box::new(|cursor_idx,lineinfo|(cursor_idx,cursor_idx.previous(lineinfo.iter().cloned()).unwrap_or(cursor_idx))));
        }))), //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::StringOnly([String::from("abc"),String::from("abc")])),Box::new(|te,kpv,_,_|{*kpv=KeyPadVariant::Num(1);}))),
    KeyButton(KeyVariant::Spacebar(images[3],String::from(" "))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image([images[4],images[4]])),Box::new(|te,_,cursor,lineinfo|{
           cursor_start_end(cursor,lineinfo,te,Some("\n".to_owned()),Box::new(|cursor_idx,_|(cursor_idx,cursor_idx)));
        }))) //new line
        
        ];
    let closetabbutton =
        KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image([images[5],
                                                                                     images[5]])),
                                      Box::new(|te, kpv, _, _| { *kpv = KeyPadVariant::None; })));
    (letter_vec, number_vec, closetabbutton)
}

fn cursor_start_end(cursor: &mut Cursor,
                    lineinfo: &Vec<Info>,
                    te: &mut std::borrow::Cow<str>,
                    s: Option<String>,
                    cursor_closure: Box<Fn(Index, Vec<Info>) -> (Index, Index)>) {
    let (start, end) = match cursor.clone() {
        Cursor::Idx(cursor_idx) => (cursor_closure)(cursor_idx, lineinfo.clone()),
        Cursor::Selection { start, end } => (start, end),
    };
    let (start_idx, end_idx) = {
        let line_infos = lineinfo.iter().cloned();
        (text::glyph::index_after_cursor(line_infos.clone(), start),
         text::glyph::index_after_cursor(line_infos, end))
    };
    if let (Some(start_idx), Some(end_idx)) = (start_idx, end_idx) {
        let (start_idx, end_idx) = (std::cmp::min(start_idx, end_idx),
                                    std::cmp::max(start_idx, end_idx));

        let new_cursor_char_idx = if start_idx > 0 { start_idx } else { 0 };
        let new_cursor_idx = {
            let line_infos = lineinfo.iter().cloned();
            index_before_char(line_infos, new_cursor_char_idx).expect("char index was out of range")
        };
        *cursor = Cursor::Idx(new_cursor_idx);
        *te.to_mut() = if let Some(_s) = s {
            te.chars()
                .take(start_idx)
                .chain(_s.chars())
                .chain(te.chars().skip(end_idx))
                .collect()
        } else {
            te.chars()
                .take(start_idx)
                .chain(te.chars().skip(end_idx))
                .collect()
        }

    }

}
