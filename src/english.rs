use custom_widget::keypad::{KeyVariant, KeyPadVariant, ClosureVariant, KeyButtonTrait,
                            ImageOrString, KeyPressType, BlankEnum};
use custom_widget::text_edit::{Style, Cursor};
use conrod;
use conrod::event;
use conrod::event::{Text, Input, Press, Button};
use conrod::input::{Key, ModifierKey};
use conrod::Ui;
use conrod::widget;
use conrod::widget::primitive::image::Image;
use conrod::text::cursor::{index_before_char, Index};
use conrod::text::line::Info;
use conrod::text;
use std;
use sprite::SpriteInfo;
pub struct KeyButton(KeyVariant);
impl KeyButtonTrait for KeyButton {
    fn dimension(&self, style: Style) -> [f64; 2] {
        match &self.0 {
            &KeyVariant::StringOnly(_) => style.letter_dim.unwrap(),
            &KeyVariant::Num(_, _) => style.num_dim.unwrap(),
            &KeyVariant::StringHold(_, _) => style.letter_dim.unwrap(),
            &KeyVariant::Spacebar(_, _) => style.spacebar_dim.unwrap(),
            &KeyVariant::Blank(_, _) => style.letter_dim.unwrap(),
            &KeyVariant::Closure(ref a, _) => {
                match a {
                    &ClosureVariant::EdgeRow3(_) => style.edge_row3_dim.unwrap(),
                    &ClosureVariant::EdgeRow4(_) => style.edge_row4_dim.unwrap(),
                }
            }
        }
    }
    fn get_variant(&self) -> &KeyVariant {
        &self.0
    }
    fn process(&self,
               events: &mut Vec<event::Widget>,
               keypadvariant: &mut KeyPadVariant,
               keypresstype: KeyPressType) {
        let mut tstring = "".to_owned();
        match self.get_variant() {
            &KeyVariant::StringOnly(ref s) => {
                match keypadvariant {
                    &mut KeyPadVariant::Letter(2) => {
                        tstring.push_str(s);
                        tstring = tstring.to_uppercase();
                        events.push(conrod::event::Widget::Text(Text {
                                                                    string: tstring,
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                    _ => {
                        events.push(conrod::event::Widget::Text(Text {
                                                                    string: s.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                }
            }
            &KeyVariant::Num(ref s1, ref s2) => {
                match keypadvariant {
                    &mut KeyPadVariant::Num(1) => {
                        events.push(conrod::event::Widget::Text(Text {
                                                                    string: s1.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                    &mut KeyPadVariant::Num(2) => {
                        events.push(conrod::event::Widget::Text(Text {
                                                                    string: s2.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                    _ => {}
                }
            }
            &KeyVariant::Spacebar(_, ref _l) => {
                events.push(conrod::event::Widget::Text(Text {
                                                            string: _l.clone(),
                                                            modifiers: ModifierKey::empty(),
                                                        }));
            }
            &KeyVariant::StringHold(ref s1, ref s2) => {
                match keypresstype {
                    KeyPressType::Press => {
                        events.push(conrod::event::Widget::Text(Text {
                                                                    string: s1.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                    KeyPressType::Hold => {
                        events.push(conrod::event::Widget::Text(Text {
                                                                    string: s2.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                }
            }
            &KeyVariant::Blank(_, _) => {}
            &KeyVariant::Closure(_, ref c) => {
                (c)(events, keypadvariant);
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
    KeyButton(KeyVariant::Blank(0.5,BlankEnum::Flat)), //10
    KeyButton(KeyVariant::StringOnly(String::from("a"))),
    KeyButton(KeyVariant::StringOnly(String::from("s"))),
    KeyButton(KeyVariant::StringOnly(String::from("d"))),
    KeyButton(KeyVariant::StringOnly(String::from("f"))),
    KeyButton(KeyVariant::StringOnly(String::from("g"))),
    KeyButton(KeyVariant::StringOnly(String::from("h"))),
    KeyButton(KeyVariant::StringOnly(String::from("j"))),
    KeyButton(KeyVariant::StringOnly(String::from("k"))),
    KeyButton(KeyVariant::StringOnly(String::from("l"))),
    KeyButton(KeyVariant::Blank(0.5,BlankEnum::Flat)), //20
    KeyButton(KeyVariant::Blank(0.0,BlankEnum::Flat)), //21
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image([images[0],images[1]])),Box::new(|_,kpv|{match kpv{
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
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image([images[2],images[2]])),Box::new(|events,_|{
        events.push(conrod::event::Widget::Press(Press{button:Button::Keyboard(Key::Backspace),modifiers:ModifierKey::empty()}));
        }))), //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::StringOnly([String::from("?123"),String::from("?123")])),Box::new(|_,kpv|{*kpv = KeyPadVariant::Num(1);}))),
    KeyButton(KeyVariant::Spacebar(images[3],String::from(" "))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image([images[4],images[4]])),Box::new(|events,_|{
          events.push(conrod::event::Widget::Press(Press{button:Button::Keyboard(Key::Return),modifiers:ModifierKey::empty()}));
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
    KeyButton(KeyVariant::Blank(0.0,BlankEnum::Flat)), //10
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
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::StringOnly([String::from("1/3"),String::from("2/3")])),Box::new(|_,kpv|{
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
    KeyButton(KeyVariant::Blank(0.0,BlankEnum::Image(images[0]))), //22
    KeyButton(KeyVariant::Num(String::from("?"),String::from("¿"))),
    KeyButton(KeyVariant::Num(String::from("!"),String::from("¡"))),
    KeyButton(KeyVariant::Num(String::from("\""),String::from("\\"))),
    KeyButton(KeyVariant::Num(String::from("\'"),String::from("<<"))),
    KeyButton(KeyVariant::Num(String::from(":"),String::from(">>"))),
    KeyButton(KeyVariant::Num(String::from(";"),String::from("®"))),
    KeyButton(KeyVariant::Num(String::from(","),String::from("©"))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image([images[2],images[2]])),Box::new(|events,_|{
        events.push(conrod::event::Widget::Press(Press{button:Button::Keyboard(Key::Backspace),modifiers:ModifierKey::empty()}));
         }))), //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::StringOnly([String::from("abc"),String::from("abc")])),Box::new(|_,kpv|{*kpv=KeyPadVariant::Num(1);}))),
    KeyButton(KeyVariant::Spacebar(images[3],String::from(" "))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image([images[4],images[4]])),Box::new(|events,_|{
           events.push(conrod::event::Widget::Press(Press{button:Button::Keyboard(Key::Return),modifiers:ModifierKey::empty()}));
        }))) //new line
        
        ];
    let closetabbutton =
        KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image([images[5],
                                                                                     images[5]])),
                                      Box::new(|_, kpv| { *kpv = KeyPadVariant::None; })));
    (letter_vec, number_vec, closetabbutton)
}
