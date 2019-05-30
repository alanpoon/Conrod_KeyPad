use conrod_core::{widget, Positionable, Widget, Color, Colorable,color, Sizeable, Point};
use conrod_core::widget::primitive::shape::triangles::Triangle;
/// The type upon which we'll implement the `Widget` trait.
use polygon2::triangulate;
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
        let SvgWidget{shapes,svgdimension,style,..} = self;
        let dim = rect.dim();
        let scale  = [dim[0]/svgdimension[0],dim[1]/svgdimension[1]];
        let num = shapes.len();
        let widget_color = style.color(&ui.theme);
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
                        let t = triangles(m,Some(white_points_index.clone()),stroke_boolean.clone());
                        widget::Triangles::single_color(widget_color,t)
                        .centre_points_to_bounding_rect()
                        .middle_of(id)
                        .wh_of(id)
                        .set(ii,ui);
                    }else{
                        widget::PointPath::new(m).middle_of(id).wh_of(id).set(ii, ui);
                    }
                    
                }
                WidgetType::PointPath(k,None,stroke_boolean)=>{
                    let m:Vec<[f64;2]> = k.clone().iter().map(|x|{
                        [x[0]*scale[0],x[1]*scale[1]]
                    }).collect();
                    //there is reflect
                    let t = triangles(m,None,stroke_boolean.clone());
                    widget::Triangles::single_color(widget_color,t)
                        .centre_points_to_bounding_rect()
                        .middle_of(id)
                        .wh_of(id)
                        .set(ii,ui);
                },
                
                WidgetType::Polygon(k)=>{
                    let m:Vec<[f64;2]> = k.clone().iter().map(|x|{
                        [x[0]*scale[0],x[1]*scale[1]]
                    }).collect();
                    let t = triangles(m,None,false);
                    widget::Triangles::single_color(widget_color,t)
                        .centre_points_to_bounding_rect()
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
pub fn triangles<I>(points:I,white_points_index:Option<usize>,reflect:bool)->Vec<Triangle<[f64;2]>> where I: IntoIterator<Item=Point>{
    let points_c = points.into_iter().collect::<Vec<Point>>();
    let mut points_k = vec![];
    if let None = white_points_index{
        let triangles = triangulate(&points_c);
        
        let l = if reflect{
            -1.0
        }else{
            1.0
        };
        for ta in 0..(triangles.len() as f64 /3.0) as usize{
            let p1 = [points_c[triangles[ta*3]][0],l*points_c[triangles[ta*3]][1]];
            let p2 = [points_c[triangles[ta*3+1]][0],l*points_c[triangles[ta*3+1]][1]];
            let p3 = [points_c[triangles[ta*3+2]][0],l*points_c[triangles[ta*3+2]][1]];
            points_k.push(Triangle([p1,p2,p3]));
        }
    }else if let Some(white_index) = white_points_index{
        let s = points_c.split_at(white_index-1).clone();
        let clip = s.0;
        let det =  points_c.iter().map(|t| rtriangulate::TriangulationPoint::new(t[0],t[1])).collect::<Vec<rtriangulate::TriangulationPoint<f64>>>();
        let det_clip = clip.iter().map(|t| rtriangulate::TriangulationPoint::new(t[0],t[1])).collect::<Vec<rtriangulate::TriangulationPoint<f64>>>();
        let subject_triangles = rtriangulate::triangulate(&det).unwrap();
        let clip_triangles = rtriangulate::triangulate(&det_clip).unwrap();
        for i in subject_triangles{
            let mut in_white =false;
            let rtriangulate::Triangle(p1,p2,p3) = i;
            for y in &clip_triangles{
                let rtriangulate::Triangle(y1,y2,y3) = y;
                if p1*p1+p2*p2+p3*p3 == y1*y1+y2*y2+y3*y3{
                    in_white = true;
                    break;
                }
            }
            if !in_white{
                points_k.push(Triangle([points_c.get(p1.clone()).unwrap().clone(),
                points_c.get(p2.clone()).unwrap().clone(),
                points_c.get(p3.clone()).unwrap().clone()]
                ));
            }
        }
    }
    return points_k;
}