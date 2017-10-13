use cardgame_widgets::custom_widget::wrap_list;
use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget, image, color, Borderable};
use conrod::widget::primitive::image::Image;
use conrod;
use custom_widget::keybut;
use custom_widget::keybut::KeyButEnum;
use application;
#[derive(Clone)]
pub enum KeyPadVariant {
    Num(usize), //1:page 1, 2:page2
    Letter(usize), //1:lowercase,2:uppercase
}
pub enum KeyPressType {
    press,
    hold,
}

pub enum KeyVariant {
    StringOnly(String),
    StringHold(String, String),
    Closure(ClosureVariant, Box<Fn(&mut String, &mut KeyPadVariant)>),
    Num(String, String),
    EdgeRow3Num(String, String),
    Spacebar(Image, String),
}
pub enum ImageOrString {
    Image(Image),
    StringOnly(String),
}
pub enum ClosureVariant {
    EdgeRow3(ImageOrString),
    EdgeRow4(ImageOrString),
}
pub trait KeyButtonTrait {
    fn dimension(&self, application::KeyButtonStyle) -> conrod::position::Dimensions;
    fn get_variant(&self) -> &KeyVariant;
    fn process(&self, &mut String, KeyPressType, &mut KeyPadVariant);
}

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct KeyPadView<'a, T: KeyButtonTrait + 'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub text_edit: &'a mut String,
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    enabled: bool,
    pub keypad_variant: KeyPadVariant,
    numkeyvec: &'a Vec<T>,
    stringkeyvec: &'a Vec<T>,
    static_style: application::KeyButtonStyle,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod::text::font::Id>>,
}

widget_ids! {
    pub struct Ids {
        canvas,
        keyboard,
    }
}

/// Represents the unique, cached state for our ChatView widget.
pub struct State {
    pub ids: Ids,
}

impl<'a, T> KeyPadView<'a, T>
    where T: KeyButtonTrait + 'a
{
    /// Create a button context to be built upon.
    pub fn new(te: &'a mut String,
               numkeyvec: &'a Vec<T>,
               stringkeyvec: &'a Vec<T>,
               keyboard_style: application::KeyButtonStyle)
               -> Self {
        KeyPadView {
            common: widget::CommonBuilder::default(),
            text_edit: te,
            style: Style::default(),
            enabled: true,
            keypad_variant: KeyPadVariant::Letter(1),
            numkeyvec: numkeyvec,
            stringkeyvec: stringkeyvec,
            static_style: keyboard_style,
        }
    }

    /// If true, will allow user inputs.  If false, will disallow user inputs.  Like
    /// other Conrod configs, this returns self for chainability. Allow dead code
    /// because we never call this in the example.
    #[allow(dead_code)]
    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, T> Widget for KeyPadView<'a, T>
    where T: KeyButtonTrait + 'a
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = Option<()>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(mut self, args: widget::UpdateArgs<Self>) -> Option<()> {
        let widget::UpdateArgs { id, state, mut ui, style, .. } = args;
        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let can = ui.rect_of(id).unwrap();
        let w_can = can.w();
        let h_can = can.h();

        widget::Canvas::new().middle_of(id).set(state.ids.canvas, ui);

        let (k_hash, len) = match self.keypad_variant {
            KeyPadVariant::Num(_) => (self.numkeyvec, self.numkeyvec.len()),
            KeyPadVariant::Letter(_) => (self.stringkeyvec, self.stringkeyvec.len()), //lowercase
        };

        //0
        let mut item0 = wrap_list::WrapList::new(len)
            .w(w_can)
            .h(h_can)
            .top_left_of(state.ids.canvas)
            .set(state.ids.keyboard, ui);
        let mut k_h_iter = k_hash.iter();
        while let (Some(item), Some(k_h)) = (item0.next(ui), k_h_iter.next()) {
            //   let j= widget::bordered_rectangle::BorderedRectangle::new(k_h.dimension(self.static_style));
            let mut lstring = "".to_owned();
            let mut sstring = "".to_owned();
            let y = match k_h.get_variant() {
                &KeyVariant::StringOnly(ref _l) => {
                    lstring = _l.clone();
                    KeyButEnum::flat(keybut::Button::new().label(&lstring))
                }
                &KeyVariant::StringHold(ref _l, ref _s) => {
                    lstring = _l.clone();
                    sstring = _s.clone();
                    KeyButEnum::flat(keybut::Button::new().label_with_superscript(&lstring,
                                                                                  &sstring))
                }
                &KeyVariant::Spacebar(ref _i, _) => {
                    KeyButEnum::image(keybut::Button::image(_i.clone()))
                }
                &KeyVariant::Closure(ref _cvariant, _) => {
                    match _cvariant {
                        &ClosureVariant::EdgeRow3(ref _i_or_s) => {
                            match _i_or_s {
                                &ImageOrString::Image(ref _i) => {
                                    KeyButEnum::image(keybut::Button::image(_i.clone()))
                                }
                                &ImageOrString::StringOnly(ref _l) => {
                                    lstring = _l.clone();
                                    KeyButEnum::flat(keybut::Button::new().label(&lstring))
                                }
                            }
                        }
                        &ClosureVariant::EdgeRow4(ref _i_or_s) => {
                            match _i_or_s {
                                &ImageOrString::Image(ref _i) => {
                                    KeyButEnum::image(keybut::Button::image(_i.clone()))
                                }
                                &ImageOrString::StringOnly(ref _l) => {
                                    lstring = _l.clone();
                                    KeyButEnum::flat(keybut::Button::new().label(&lstring))
                                }
                            }
                        }
                    }
                }
                &KeyVariant::Num(ref numpad1, ref numpad2) => {
                    if let KeyPadVariant::Num(1) = self.keypad_variant {
                        lstring = numpad1.clone();
                        KeyButEnum::flat(keybut::Button::new().label(&lstring))
                    } else {
                        lstring = numpad2.clone();
                        KeyButEnum::flat(keybut::Button::new().label(&lstring))
                    }
                }
                &KeyVariant::EdgeRow3Num(ref numpad1, ref numpad2) => {
                    if let KeyPadVariant::Num(1) = self.keypad_variant {
                        lstring = numpad1.clone();
                        KeyButEnum::flat(keybut::Button::new().label(&lstring))
                    } else {
                        lstring = numpad2.clone();
                        KeyButEnum::flat(keybut::Button::new().label(&lstring))
                    }
                }
            };
            match y {
                KeyButEnum::flat(j) => {
                    let jj = j.wh(k_h.dimension(self.static_style)).border_color(color::BLACK);
                    let jk = item.set(jj, k_h.dimension(self.static_style)[0], ui);
                    if jk.clone().was_clicked() {
                        if jk.was_hold() {
                            let mut keypad_variant = self.keypad_variant;
                            k_h.process(self.text_edit, KeyPressType::hold, &mut keypad_variant);
                            self.keypad_variant = keypad_variant;
                        } else {
                            let mut keypad_variant = self.keypad_variant;
                            k_h.process(self.text_edit, KeyPressType::press, &mut keypad_variant);
                            self.keypad_variant = keypad_variant;
                        }
                    }
                }
                KeyButEnum::image(j) => {
                    let jj = j.wh(k_h.dimension(self.static_style)).border_color(color::BLACK);
                    let jk = item.set(jj, k_h.dimension(self.static_style)[0], ui);
                    if jk.clone().was_clicked() {
                        if jk.was_hold() {
                            let mut keypad_variant = self.keypad_variant;
                            k_h.process(self.text_edit, KeyPressType::hold, &mut keypad_variant);
                            self.keypad_variant = keypad_variant;
                        } else {
                            let mut keypad_variant = self.keypad_variant;
                            k_h.process(self.text_edit, KeyPressType::press, &mut keypad_variant);
                            self.keypad_variant = keypad_variant;
                        }
                    }
                }
            }

        }

        Some(())
    }
}
