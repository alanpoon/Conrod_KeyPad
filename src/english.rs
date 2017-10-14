use custom_widget::keypad::{KeyVariant, KeyPadVariant, ClosureVariant, KeyButtonTrait,
                            ImageOrString, KeyPressType,BlankEnum};

use conrod;
use conrod::widget;
use conrod::widget::primitive::image::Image;
#[cfg(feature="hotload")]
use dyapplication as application;
use application::SpriteInfo;
pub struct KeyButton(KeyVariant);
impl KeyButtonTrait for KeyButton {
    fn dimension(&self, static_style: application::KeyButtonStyle) -> conrod::position::Dimensions {
        match &self.0 {
            &KeyVariant::StringOnly(_) => static_style.normal.0,
            &KeyVariant::Num(_, _) => static_style.num.0,
            &KeyVariant::EdgeRow3Num(_, _) => static_style.edge_row3.0,
            &KeyVariant::StringHold(_, _) => static_style.normal.0,
            &KeyVariant::Spacebar(_, _) => static_style.spacebar,
            &KeyVariant::Blank(_,_)=>static_style.normal.0,
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
               te: &mut String,
               keypresstype: KeyPressType,
               keypadvariant: &mut KeyPadVariant) {
        let mut tstring = "".to_owned();
        match self.get_variant() {
            &KeyVariant::StringOnly(ref s) => {
                match keypadvariant {
                    &mut KeyPadVariant::Letter(2) => {
                        tstring.push_str(s);
                        tstring = tstring.to_uppercase();
                        te.push_str(&tstring);
                    }
                    _ => {
                        te.push_str(s);
                    }
                }
            }
            &KeyVariant::Num(ref s1, ref s2) => {
                match keypadvariant {
                    &mut KeyPadVariant::Num(1) => {
                        te.push_str(s1);
                    }
                    &mut KeyPadVariant::Num(2) => {
                        te.push_str(s2);
                    }
                    _ => {}
                }
            }
            &KeyVariant::Spacebar(_, ref _l) => {
                te.push_str(_l);
            }
            &KeyVariant::EdgeRow3Num(_, _) => {
                match keypadvariant {
                    &mut KeyPadVariant::Num(1) => {
                        *keypadvariant = KeyPadVariant::Num(2);
                    }
                    &mut KeyPadVariant::Num(2) => {
                        *keypadvariant = KeyPadVariant::Num(1);
                    }
                    _ => {}
                }
            }
            &KeyVariant::StringHold(ref s1, ref s2) => {
                match keypresstype {
                    KeyPressType::press => {
                        te.push_str(s1);
                    }
                    KeyPressType::hold => {
                        te.push_str(s2);
                    }
                }
            }
            &KeyVariant::Blank(_,_)=>{

            }
            &KeyVariant::Closure(_, ref c) => {
                (c)(te, keypadvariant);
            }
        }
    }
}
pub fn populate(image_id: conrod::image::Id,
                spriteinfo: SpriteInfo)
                -> (Vec<KeyButton>, Vec<KeyButton>) {
    //(letter_vec,number_vec)
    let images: [Image; 5] =
        [widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(0.0)),
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(1.0)),
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(2.0)),
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(3.0)),
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(4.0))];
    let letter_vec=vec![KeyButton(KeyVariant::StringHold(String::from("q"),String::from("1"))),
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
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image(images[0])),Box::new(|_,kpv|{match kpv{
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
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image(images[2])),Box::new(|te,_|{te.pop();}))), //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::StringOnly(String::from("?123"))),Box::new(|te,kpv|{*kpv = KeyPadVariant::Num(1);}))),
    KeyButton(KeyVariant::Spacebar(images[3],String::from(" "))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image(images[4])),Box::new(|te,_|{te.push_str("/n");})))];

    let number_vec=vec![KeyButton(KeyVariant::Num(String::from("1"),String::from("~"))),
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
    KeyButton(KeyVariant::EdgeRow3Num(String::from("1/3"),String::from("2/3"))), //21
    KeyButton(KeyVariant::Blank(0.0,BlankEnum::image(images[0]))), //22
    KeyButton(KeyVariant::Num(String::from("?"),String::from("¿"))),
    KeyButton(KeyVariant::Num(String::from("!"),String::from("¡"))),
    KeyButton(KeyVariant::Num(String::from("\""),String::from("\\"))),
    KeyButton(KeyVariant::Num(String::from("\'"),String::from("<<"))),
    KeyButton(KeyVariant::Num(String::from(":"),String::from(">>"))),
    KeyButton(KeyVariant::Num(String::from(";"),String::from("®"))),
    KeyButton(KeyVariant::Num(String::from(","),String::from("©"))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image(images[2])),Box::new(|te,_|{te.pop();}))), //backspace
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::StringOnly(String::from("abc"))),Box::new(|te,kpv|{*kpv=KeyPadVariant::Num(1);}))),
    KeyButton(KeyVariant::Spacebar(images[3],String::from(" "))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image(images[4])),Box::new(|te,_|{te.push_str("\n");}))) //new line
    ];
    (letter_vec, number_vec)
}
