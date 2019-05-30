use conrod_core::{widget, Positionable, Widget, Color, Colorable,color, Sizeable};
/// The type upon which we'll implement the `Widget` trait.

#[derive(Clone)]
pub enum WidgetType{
    Polygon(Vec<[f64;2]>),
    PointPath(Vec<[f64;2]>,Option<usize>,bool),
    None
}
#[derive(WidgetCommon)]
pub struct SvgWidget<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub shapes: &'a Vec<WidgetType>,
    pub svgdimension: [f64;2],
    /// See the Style struct below.
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// The color of the Canvas' rectangle surface.
    #[conrod(default = "theme.background_color")]
    pub color: Option<Color>,
}

widget_ids! {
    struct Ids {
        items[]
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}

impl<'a> SvgWidget<'a> {
    /// Create a button context to be built upon.
    pub fn new(shapes:&'a (Vec<WidgetType>,[f64;2])) -> Self {
        SvgWidget {
            shapes: &shapes.0,
            svgdimension:shapes.1,
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for SvgWidget<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;
        let SvgWidget{shapes,svgdimension,..} = self;
        let dim = rect.dim();
        let scale  = [dim[0]/svgdimension[0],dim[1]/svgdimension[1]];
        let num = shapes.len();
        if state.ids.items.len() < num {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.items.resize(num, id_gen));
        }
        for (shape,&ii) in shapes.iter().zip(state.ids.items.iter())
        {
            match shape{
                WidgetType::PointPath(subject,Some(white_points_index),stroke_boolean)=>{
                    let m:Vec<[f64;2]> = subject.clone().iter().map(|x|{
                        [x[0]*scale[0],x[1]*scale[1]]
                    }).collect();
                    if !stroke_boolean{
                        widget::Polygon::centred_fill(m)
                        .middle_of(id)
                        .reflect()
                        .white_points_index(white_points_index.clone())
                        .wh_of(id)
                        .set(ii,ui);
                    }else{
                        widget::PointPath::centred(m).middle_of(id).wh_of(id).set(ii, ui);
                    }
                    
                }
                WidgetType::PointPath(k,None,stroke_boolean)=>{
                    let m:Vec<[f64;2]> = k.clone().iter().map(|x|{
                        [x[0]*scale[0],x[1]*scale[1]]
                    }).collect();
                    widget::Polygon::centred_fill(m)
                        .middle_of(id)
                        .reflect()
                        .wh_of(id)
                        .set(ii,ui);
                },
                
                WidgetType::Polygon(k)=>{
                    let m:Vec<[f64;2]> = k.clone().iter().map(|x|{
                        [x[0]*scale[0],x[1]*scale[1]]
                    }).collect();
                    
                    widget::Polygon::centred_fill(m)
                        .middle_of(id)
                        .wh_of(id)
                        .set(ii,ui);
                },
                _=>{}
            }
        }
        
    }
}
impl<'a> Colorable for SvgWidget<'a> {
    builder_method!(color { style.color = Some(Color) });
}
