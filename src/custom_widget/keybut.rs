//! The `Button` widget and related items.

use conrod_core::{Color, color, Colorable, FontSize, Borderable, Labelable, Positionable, Sizeable,
             UiCell, Widget, text, event, input};
use conrod_core::position::{self, Align, Rect, Scalar};
use conrod_core::widget::envelope_editor::EnvelopePoint;
use conrod_core::widget;
use std::time::{Duration, Instant};
use custom_widget::svg::{SvgWidget,WidgetType,SvgInfo};
use std::cmp;
pub enum KeyButEnum<'a> {
    Flat(Button<'a, Flat>),
    Svg(Button<'a, SvgWidget>),
    BlankFlat(f64, Button<'a, Flat>), //width mutliplier
    BlankSvg(f64, Button<'a, SvgWidget>),
}

/// A pressable button widget whose reaction is triggered upon release.
#[derive(WidgetCommon)]
pub struct Button<'a, S> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    maybe_label: Option<&'a str>,
    maybe_label_with_superscript: Option<(&'a str, &'a str)>,
    /// Whether the `Button` is a `Flat` color or an `SvgWidget`.
    pub show: S,
    /// Unique styling parameters for the Button.
    pub style: Style,
    /// Whether or not user input is enabled.
    enabled: bool,
}

/// Unique styling for the Button.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the Button's pressable area.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// Width of the border surrounding the button
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
    /// The color of the Button's label.
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<Color>,
    /// The font size of the Button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<FontSize>,
    /// The ID of the font used to display the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<text::font::Id>>,
    /// The label's typographic alignment over the *x* axis.
    #[conrod(default = "text::Justify::Center")]
    pub label_justify: Option<text::Justify>,
    /// The position of the title bar's `Label` widget over the *x* axis.
    #[conrod(default = "position::Relative::Align(Align::Middle)")]
    pub label_x: Option<position::Relative>,
    /// The position of the title bar's `Label` widget over the *y* axis.
    #[conrod(default = "position::Relative::Align(Align::Middle)")]
    pub label_y: Option<position::Relative>,
}

/// The State of the Button widget that will be cached within the Ui.
pub struct FlatState {
    /// Track whether some sort of dragging is currently occurring.
    drag: Drag,
    ids: FlatIds,
}
impl FlatState {
    fn change_drag(&mut self, drag: Drag) {
        self.drag = drag;
    }
}
/// The State of the Button widget that will be cached within the Ui.
pub struct ImageState {
    /// Track whether some sort of dragging is currently occurring.
    drag: Drag,
    ids: ImageIds,
}
/// Track whether some sort of dragging is currently occurring.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Drag {
    /// The drag is currently selecting a range of text.
    Selecting(Instant),
    None,
    Terminate,
}

widget_ids! {
    /// Identifiers for a "flat" button.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct FlatIds {
        rectangle,
        label,
        superscript
    }
}

widget_ids! {
    /// Identifiers for an image button.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct ImageIds {
        image,
        label,
        rectangle
    }
}

/// The `Button` simply displays a flat color.
#[derive(Copy, Clone)]
pub struct Flat;

#[derive(Copy, Clone,Debug)]
pub enum Interaction {
    Idle,
    Hover,
    Press,
    Hold,
}

/// The `Event` type yielded by the `Button` widget.
///
/// Represents the number of times that the `Button` has been clicked with the left mouse button
/// since the last update.
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct TimesClicked(pub Interaction);


impl TimesClicked {
    /// `true` if the `Button` was clicked one or more times.
    pub fn was_clicked(self) -> bool {
        if let Interaction::Press = self.0 {
            true
        } else {
            false
        }
    }
    pub fn was_hold(self) -> bool {
        if let Interaction::Hold = self.0 {
            true
        } else {
            false
        }
    }
}

impl<'a> Button<'a, Flat> {
    /// Begin building a flat-colored `Button` widget.
    pub fn new() -> Self {
        Self::new_internal(Flat)
    }
    /// Add label and superscript
    pub fn label_with_superscript(mut self, label: &'a str, superscript: &'a str) -> Self {
        self.maybe_label_with_superscript = Some((label, superscript));
        self
    }
    /// Override the default button style
    pub fn with_style(mut self, s: Style) -> Self {
        self.style = s;
        self
    }
}

impl<'a, S> Button<'a, S> {
    /// Create a button context to be built upon.
    fn new_internal(show: S) -> Self {
        Button {
            common: widget::CommonBuilder::default(),
            show: show,
            maybe_label: None,
            maybe_label_with_superscript: None,
            style: Style::default(),
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }

    /// Align the label to the left of the `Button`'s surface.
    pub fn left_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Left);
        self
    }

    /// Align the label to the mid-left of the `Button`'s surface.
    ///
    /// This is the default label alignment.
    pub fn center_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Center);
        self
    }

    /// Align the label to the mid-left of the `Button`'s surface.
    pub fn right_justify_label(mut self) -> Self {
        self.style.label_justify = Some(text::Justify::Right);
        self
    }

    /// Specify the label's position relatively to `Button` along the *x* axis.
    pub fn label_x(mut self, x: position::Relative) -> Self {
        self.style.label_x = Some(x);
        self
    }

    /// Specify the label's position relatively to `Button` along the *y* axis.
    pub fn label_y(mut self, y: position::Relative) -> Self {
        self.style.label_y = Some(y);
        self
    }

    builder_methods!{
        pub enabled { enabled = bool }
    }
}
impl<'a> Button<'a, SvgWidget> {
    /// Begin building a button displaying the given `Image` on top.
    pub fn svg(svg: SvgWidget) -> Self {
        Self::new_internal(svg)
    }
}
impl<'a> Widget for Button<'a, SvgWidget> {
    type State = ImageState;
    type Style = Style;
    type Event = TimesClicked;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        ImageState {
            drag: Drag::None,
            ids: ImageIds::new(id_gen),
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the Button.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let Button { show, .. } = self;
        let mut drag = state.drag;
        let interaction = interaction_and_times_triggered(id, &mut drag, ui);
        let color = color_from_interaction(style.color(&ui.theme), interaction, &mut drag);
        bordered_rectangle(id, state.ids.rectangle, rect, color, style, ui);
        match interaction {
            Interaction::Hover => {
                //  println!("hovering id:{:?}", id);
            }
            _ => {}
        }
        // Instantiate the svg.
        let widget_image = show;
        let (x, y, w, h) = rect.x_y_w_h();
        let min_dim = cmp::min(w as u32,h as u32);
        let width_ofsvg = if (w/h >5.0){
            min_dim as f64 *2.0
        }else{
            min_dim as f64
        };
        let image = widget_image.x_y(x, y)
            .middle_of(id)
            .w_h(width_ofsvg * 0.8, min_dim as f64 * 0.8)
            .parent(id)
            .graphics_for(id);

        image.set(state.ids.image, ui);
        TimesClicked(interaction)
    }
}

impl<'a> Widget for Button<'a, Flat> {
    type State = FlatState;
    type Style = Style;
    type Event = TimesClicked;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        FlatState {
            drag: Drag::None,
            ids: FlatIds::new(id_gen),
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the Button.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let Button { maybe_label, maybe_label_with_superscript, .. } = self;
        let mut drag = state.drag.clone();
        let interaction = interaction_and_times_triggered(id, &mut drag, ui);
        let color = color_from_interaction(style.color(&ui.theme), interaction, &mut drag);
        state.update(|state| state.change_drag(drag));
        match interaction {
            Interaction::Hover => {
                // println!("hovering id:{:?}", id);
            }
            _ => {}
        }
        bordered_rectangle(id, state.ids.rectangle, rect, color, style, ui);

        // Label widget.
        if let Some(l) = maybe_label {
            label(id, state.ids.label, l, style, ui);
        }
        if let Some((l, s)) = maybe_label_with_superscript {
            label_with_superscript(id, state.ids.label, state.ids.superscript, l, s, style, ui);
        }
        TimesClicked(interaction)
    }
}



fn color_from_interaction(color: Color, interaction: Interaction, drag: &mut Drag) -> Color {
    match drag {
        &mut Drag::Selecting(_) => color.highlighted(),
        _ => {
            match interaction {
                Interaction::Idle => color,
                Interaction::Hover => color.highlighted(),
                Interaction::Press => color.clicked(),
                Interaction::Hold => color.highlighted(),
            }
        }
    }

}

fn interaction_and_times_triggered(button_id: widget::Id,
                                   drag: &mut Drag,
                                   ui: &UiCell)
                                   -> Interaction {
    let mut interaction = Interaction::Idle;
    for widget_event in ui.widget_input(button_id).events() {
        match widget_event {
            event::Widget::Press(press) => {
                match press.button {
                    event::Button::Mouse(input::MouseButton::Left, _) => {
                        let now = Instant::now();
                        match drag {
                            &mut Drag::Selecting(a) => {
                                if a.elapsed() >= Duration::from_secs(1) {
                                    interaction = Interaction::Hold;
                                    *drag = Drag::Terminate;
                                }
                            }
                            &mut Drag::None => {
                                *drag = Drag::Selecting(now);
                            }
                            &mut Drag::Terminate => {}
                        }
                    }
                    _ => {}
                }
            }
            event::Widget::Click(click) => {
                match (click, drag.clone()) {
                    (event::Click { button: input::MouseButton::Left, .. }, Drag::Terminate) => {
                        *drag = Drag::None;
                    }
                    _ => {
                        interaction = Interaction::Press;
                    }
                }
            }
            event::Widget::Release(release) => {
                if let event::Button::Mouse(input::MouseButton::Left, _) = release.button {
                    match drag {
                        &mut Drag::Selecting(a) => {
                            if a.elapsed() >= Duration::from_secs(1) {
                                *drag = Drag::Terminate;
                            } else {
                                *drag = Drag::None;
                            }
                            interaction = Interaction::Press;
                        }
                        _ => {}
                    }
                }
            }
            event::Widget::Drag(drag_event) if drag_event.button == input::MouseButton::Left => {
                match drag {
                    &mut Drag::Selecting(_) => {
                        let dim = ui.wh_of(button_id).unwrap();
                        if (drag_event.to.get_x().abs() > dim[0]) ||
                           (drag_event.to.get_y().abs() > dim[1]) {
                            *drag = Drag::None;
                            interaction = Interaction::Idle;
                        }
                    }
                    _ => {}
                }
            }
            /*   event::Widget::Click(click)=>match click.button{
                    interaction = Interaction::Press;
                },*/
            _ => {
                if let Drag::None = *drag {
                    interaction = Interaction::Hover;
                }
            }
        }
    }
    interaction
}

fn bordered_rectangle(button_id: widget::Id,
                      rectangle_id: widget::Id,
                      rect: Rect,
                      color: Color,
                      style: &Style,
                      ui: &mut UiCell) {
    // BorderedRectangle widget.
    let dim = rect.dim();
    let border = style.border(&ui.theme);
    let border_color = style.border_color(&ui.theme);
    widget::BorderedRectangle::new(dim)
        .middle_of(button_id)
        .graphics_for(button_id)
        .color(color)
        .border(border)
        .border_color(border_color)
        .set(rectangle_id, ui);
}

fn label(button_id: widget::Id,
         label_id: widget::Id,
         label: &str,
         style: &Style,
         ui: &mut UiCell) {
    let color = style.label_color(&ui.theme);
    let font_size = style.label_font_size(&ui.theme);
    let x = style.label_x(&ui.theme);
    let y = style.label_y(&ui.theme);
    let justify = style.label_justify(&ui.theme);
    let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
    widget::Text::new(label)
        .and_then(font_id, widget::Text::font_id)
        .x_position_relative_to(button_id, x)
        .y_position_relative_to(button_id, y)
        .justify(justify)
        .parent(button_id)
        .graphics_for(button_id)
        .color(color)
        .font_size(font_size)
        .set(label_id, ui);
}
fn label_with_superscript(button_id: widget::Id,
                          label_id: widget::Id,
                          superscript_id: widget::Id,
                          label: &str,
                          superscript: &str,
                          style: &Style,
                          ui: &mut UiCell) {
    let color = style.label_color(&ui.theme);
    let font_size = style.label_font_size(&ui.theme);
    let x = style.label_x(&ui.theme);
    let y = style.label_y(&ui.theme);
    let justify = style.label_justify(&ui.theme);
    let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
    let button_rect_w = ui.w_of(button_id).unwrap();
    widget::Text::new(label)
        .and_then(font_id, widget::Text::font_id)
        .x_position_relative_to(button_id, x)
        .y_position_relative_to(button_id, y)
        .justify(justify)
        .parent(button_id)
        .graphics_for(button_id)
        .color(color)
        .font_size(font_size - 1)
        .set(label_id, ui);
    widget::Text::new(superscript)
        .and_then(font_id, widget::Text::font_id)
        .top_right_with_margin_on(button_id, button_rect_w * 0.1)
        .color(color::BLACK)
        .font_size(font_size - 2)
        .set(superscript_id, ui);
}

impl<'a, S> Colorable for Button<'a, S> {
    builder_method!(color { style.color = Some(Color) });
}

impl<'a, S> Borderable for Button<'a, S> {
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}

impl<'a, S> Labelable<'a> for Button<'a, S> {
    builder_methods!{
        label { maybe_label = Some(&'a str) }
        label_color { style.label_color = Some(Color) }
        label_font_size { style.label_font_size = Some(FontSize) }
    }
}
