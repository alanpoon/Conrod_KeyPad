use custom_widget::keypad::{KeyVariant, KeyPadVariant, ClosureVariant, KeyButtonTrait,
                            SvgOrString, KeyPressType, BlankEnum};
use custom_widget::text_edit::Style;
use conrod_core;
use conrod_core::{widget,color,event};
use conrod_core::event::{Text, Press, Button};
use conrod_core::input::{Key, ModifierKey};
use conrod_core::widget::envelope_editor::EnvelopePoint;
use load_svg::SvgKeypad;
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
                        events.push(conrod_core::event::Widget::Text(Text {
                                                                    string: tstring,
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                    _ => {
                        events.push(conrod_core::event::Widget::Text(Text {
                                                                    string: s.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                }
            }
            &KeyVariant::Num(ref s1, ref s2) => {
                match keypadvariant {
                    &mut KeyPadVariant::Num(1) => {
                        events.push(conrod_core::event::Widget::Text(Text {
                                                                    string: s1.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                    &mut KeyPadVariant::Num(2) => {
                        events.push(conrod_core::event::Widget::Text(Text {
                                                                    string: s2.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                    _ => {}
                }
            }
            &KeyVariant::Spacebar(_, ref _l) => {
                events.push(conrod_core::event::Widget::Text(Text {
                                                            string: _l.clone(),
                                                            modifiers: ModifierKey::empty(),
                                                        }));
            }
            &KeyVariant::StringHold(ref s1, ref s2) => {
                match keypresstype {
                    KeyPressType::Press => {
                        events.push(conrod_core::event::Widget::Text(Text {
                                                                    string: s1.clone(),
                                                                    modifiers: ModifierKey::empty(),
                                                                }));
                    }
                    KeyPressType::Hold => {
                        events.push(conrod_core::event::Widget::Text(Text {
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
pub fn populate()
                -> (Vec<KeyButton>, Vec<KeyButton>, KeyButton) {
    let svg_keypad = SvgKeypad::new();     
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
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(SvgOrString::Svg(svg_keypad.arrow_full_up.clone(),color::BLACK,color::GREEN)),Box::new(|_,kpv|{match kpv{
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
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(SvgOrString::Svg(svg_keypad.delete.clone(),color::BLACK,color::BLACK)),Box::new(|events,_|{
        events.push(conrod_core::event::Widget::Press(Press{button:Button::Keyboard(Key::Backspace),modifiers:ModifierKey::empty()}));
        }))), //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(SvgOrString::StringOnly([String::from("?123"),String::from("?123")])),Box::new(|_,kpv|{*kpv = KeyPadVariant::Num(1);}))),
    KeyButton(KeyVariant::Spacebar(svg_keypad.spacebar.clone(),String::from(" "))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(SvgOrString::Svg(svg_keypad.enter.clone(),color::BLACK,color::BLACK)),Box::new(|events,_|{
          events.push(conrod_core::event::Widget::Press(Press{button:Button::Keyboard(Key::Return),modifiers:ModifierKey::empty()}));
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
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(SvgOrString::StringOnly([String::from("1/3"),String::from("2/3")])),Box::new(|_,kpv|{
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
    KeyButton(KeyVariant::Blank(0.0,BlankEnum::Svg(svg_keypad.arrow_full_up.clone()))), //22
    KeyButton(KeyVariant::Num(String::from("?"),String::from("¿"))),
    KeyButton(KeyVariant::Num(String::from("!"),String::from("¡"))),
    KeyButton(KeyVariant::Num(String::from("\""),String::from("\\"))),
    KeyButton(KeyVariant::Num(String::from("\'"),String::from("<<"))),
    KeyButton(KeyVariant::Num(String::from(":"),String::from(">>"))),
    KeyButton(KeyVariant::Num(String::from(";"),String::from("®"))),
    KeyButton(KeyVariant::Num(String::from(","),String::from("©"))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(SvgOrString::Svg(svg_keypad.delete.clone(),color::BLACK,color::BLACK)),Box::new(|events,_|{
        events.push(conrod_core::event::Widget::Press(Press{button:Button::Keyboard(Key::Backspace),modifiers:ModifierKey::empty()}));
         }))), //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(SvgOrString::StringOnly([String::from("abc"),String::from("abc")])),Box::new(|_,kpv|{*kpv=KeyPadVariant::Num(1);}))),
    KeyButton(KeyVariant::Spacebar(svg_keypad.spacebar.clone(),String::from(" "))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(SvgOrString::Svg(svg_keypad.enter.clone(),color::BLACK,color::BLACK)),Box::new(|events,_|{
           events.push(conrod_core::event::Widget::Press(Press{button:Button::Keyboard(Key::Return),modifiers:ModifierKey::empty()}));
        }))) //new line
        
        ];
    let closetabbutton =
        KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(SvgOrString::Svg(svg_keypad.double_arrow_down.clone(),color::BLACK,color::BLACK)),
                                      Box::new(|_, kpv| { *kpv = KeyPadVariant::None; })));
    (letter_vec, number_vec, closetabbutton)
}
