use custom_widget::keypad::{KeyVariant, ClosureVariant, KeyButtonTrait, ImageOrString};

use conrod;
use conrod::widget;
use conrod::widget::primitive::image::Image;
#[cfg(feature="hotload")]
use dyapplication as application;
use application::SpriteInfo;
pub struct KeyButton(KeyVariant);
impl KeyButtonTrait for KeyButton {
    fn dimension(&self,
                     static_style: application::KeyButtonStyle)
                     -> conrod::position::Dimensions {
        match &self.0 {
            &KeyVariant::StringOnly(_) => static_style.normal.0,
            &KeyVariant::Num(_,_)=>static_style.num.0,
            &KeyVariant::StringHold(_, _) => static_style.normal.0,
            &KeyVariant::Closure(ref a, _) => {
                match a {
                    &ClosureVariant::Enter => static_style.enter,
                    &ClosureVariant::EdgeRow3(_) => static_style.edge_row3.0,
                    &ClosureVariant::EdgeRow4(_) => static_style.edge_row4.0,
                    &ClosureVariant::Spacebar => static_style.spacebar,
                }
            }
        }
    }
}
pub fn populate(image_id: conrod::image::Id,
                                   spriteinfo: SpriteInfo)
                                   -> (Vec<KeyButton>, Vec<KeyButton>) {
    //(letter_vec,number_vec)
    let images: [Image; 4] =
        [widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(0.0)),
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(1.0)),
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(2.0)),
         widget::Image::new(image_id).source_rectangle(spriteinfo.src_rect(3.0))];
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
    KeyButton(KeyVariant::StringOnly(String::from("a"))),
    KeyButton(KeyVariant::StringOnly(String::from("s"))),
    KeyButton(KeyVariant::StringOnly(String::from("d"))),
    KeyButton(KeyVariant::StringOnly(String::from("f"))),
    KeyButton(KeyVariant::StringOnly(String::from("g"))),
    KeyButton(KeyVariant::StringOnly(String::from("h"))),
    KeyButton(KeyVariant::StringOnly(String::from("j"))),
    KeyButton(KeyVariant::StringOnly(String::from("k"))),
    KeyButton(KeyVariant::StringOnly(String::from("l"))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image(images[0])),Box::new(||{}))),
    KeyButton(KeyVariant::StringOnly(String::from("z"))),
    KeyButton(KeyVariant::StringOnly(String::from("x"))),
    KeyButton(KeyVariant::StringOnly(String::from("c"))),
    KeyButton(KeyVariant::StringOnly(String::from("v"))),
    KeyButton(KeyVariant::StringOnly(String::from("b"))),
    KeyButton(KeyVariant::StringOnly(String::from("n"))),
    KeyButton(KeyVariant::StringOnly(String::from("m"))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image(images[1])),Box::new(||{}))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::StringOnly(String::from("?123"))),Box::new(||{}))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image(images[2])),Box::new(||{}))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image(images[3])),Box::new(||{})))];

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
    KeyButton(KeyVariant::Num(String::from("@"),String::from("$"))),
    KeyButton(KeyVariant::Num(String::from("#"),String::from("€"))),
    KeyButton(KeyVariant::Num(String::from("%"),String::from("£"))),
    KeyButton(KeyVariant::Num(String::from("&"),String::from("¥"))),
    KeyButton(KeyVariant::Num(String::from("*"),String::from("¢"))),
    KeyButton(KeyVariant::Num(String::from("/"),String::from("₩"))),
    KeyButton(KeyVariant::Num(String::from("-"),String::from("§"))),
    KeyButton(KeyVariant::Num(String::from("+"),String::from("^"))),
    KeyButton(KeyVariant::Num(String::from("("),String::from("`"))),
    KeyButton(KeyVariant::Num(String::from(")"),String::from("∘"))),
    KeyButton(KeyVariant::Num(String::from("1/3"),String::from("2/3"))),
    KeyButton(KeyVariant::Num(String::from("?"),String::from("¿"))),
    KeyButton(KeyVariant::Num(String::from("!"),String::from("¡"))),
    KeyButton(KeyVariant::Num(String::from("\""),String::from("\\"))),
    KeyButton(KeyVariant::Num(String::from("\'"),String::from("<<"))),
    KeyButton(KeyVariant::Num(String::from(":"),String::from(">>"))),
    KeyButton(KeyVariant::Num(String::from(";"),String::from("®"))),
    KeyButton(KeyVariant::Num(String::from(","),String::from("©"))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow3(ImageOrString::Image(images[1])),Box::new(||{}))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::StringOnly(String::from("abc"))),Box::new(||{}))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image(images[2])),Box::new(||{}))),
    KeyButton(KeyVariant::StringOnly(String::from("."))),
    KeyButton(KeyVariant::Closure(ClosureVariant::EdgeRow4(ImageOrString::Image(images[3])),Box::new(||{})))
    ];
    (letter_vec, number_vec)
}
