use svg::node::element::path::{Command, Data,Position};
use svg::node::element::tag::{Polygon,Path,SVG};
use svg::parser::{Event,Parser};
use custom_widget::svg::{WidgetType};
use conrod_core::widget::primitive::shape::oval::Circumference;
use conrod_core::position::rect::Rect;
use std::cmp;
use nalgebra;
use num::clamp;
use bezier2::cubic;

pub struct SvgKeypad{
    pub arrow_full_up: (Vec<WidgetType>,[f64;2]),
    pub delete: (Vec<WidgetType>,[f64;2]),
    pub double_arrow_down: (Vec<WidgetType>,[f64;2]),
    pub enter: (Vec<WidgetType>,[f64;2]),
}
impl SvgKeypad{
    pub fn new()->Self{
        let arrow_full_up = include_str!("arrow_full_up.svg").to_owned();
        let delete = include_str!("a_delete.svg").to_owned();
        let double_arrow_down = include_str!("a_double-arrow-down.svg").to_owned();
        let enter = include_str!("a_space.svg").to_owned();

        SvgKeypad{
            //arrow_full_up: convert(Parser::new(arrow_full_up),"arrow_full_up.svg"),
            arrow_full_up:(vec![],[0.0,0.0]),
            //delete: convert(Parser::new(delete),"delete"),
            delete:(vec![],[0.0,0.0]),
            double_arrow_down:(vec![],[0.0,0.0]),
            //enter:vec![],
            //double_arrow_down: convert(Parser::new(double_arrow_down),"doubledown"),
            enter: convert(Parser::new(enter),"enter")
        }
    }
}

fn convert<'a>(parser:Parser<'a>,k:&'a str)->(Vec<WidgetType>,[f64;2]){
    let mut vec_widget= vec![];
    let mut svgheight:Option<f64> = None;
    let mut svgwidth:Option<f64> = None;
    for event in parser{
        match event {
            Event::Tag(SVG,_,attributes)=>{
                if let None = svgheight{
                    svgheight = Some(attributes.get("height").unwrap().split("px").collect::<Vec<&str>>()[0].parse::<f64>().unwrap());
                }
                if let None = svgwidth{
                    svgwidth = Some(attributes.get("width").unwrap().split("px").collect::<Vec<&str>>()[0].parse::<f64>().unwrap());    
                }
            },
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let stroke_boolean = if attributes.get("style").unwrap().contains("stroke"){
                    true
                }else{
                    false
                };
                let data = Data::parse(data).unwrap();
                let mut current_p:WidgetType = WidgetType::None;
                let mut last_control_point:Option<[f64;2]> = None;
                for command in data.iter() {
                    match command {
                        Command::Close=>{
                            if let WidgetType::PointPath(subject,None,stroke_boolean) = &mut current_p{
                                //https://stackoverflow.com/questions/1165647/how-to-determine-if-a-list-of-polygon-points-are-in-clockwise-order

                                let clockwise = subject.iter().fold((0f64,[0f64;2]),|(sum,prev),val|{
                                    let next = val;
                                    let mut edge = (val[0]-prev[0])*(val[1]+prev[1]);
                                    if prev ==[0f64;2]{
                                        edge = 0.0;
                                    }
                                    return (sum+edge,val.clone());
                                } );
                                if clockwise.0 > 0.0{ //clockwise
                                    subject.reverse();
                                }
                                vec_widget.push(WidgetType::PointPath(subject.clone(),None,stroke_boolean.clone()));
                                current_p = WidgetType::None;
                            } else if let WidgetType::PointPath(subject,Some(whiteindex),sb) = &mut current_p{
                                vec_widget.push(WidgetType::PointPath(subject.clone(),Some(whiteindex.clone()),sb.clone()));
                                current_p = WidgetType::None;
                            }
                        },
                        Command::Move(_position,_p) => {
                            let k = _p.clone();
                            if let WidgetType::None = current_p{
                                current_p = WidgetType::PointPath(vec![[k[0] as f64,k[1] as f64]],None,stroke_boolean.clone());
                            } else if let WidgetType::PointPath(p,_,_) = &mut current_p{
                                p.push([k[0] as f64,k[1] as f64]);
                                current_p = WidgetType::PointPath(p.clone(),Some(p.len()),stroke_boolean.clone());
                            }
                        },
                        Command::Line(_position,_p) => {
                            let k = _p.clone();
                            if let WidgetType::PointPath(t,_,_) = &mut current_p{
                                
                                let mut count =0;
                                let mut store= vec![];
                                let mut x:f32=0.0;
                                for mk in k.iter(){
                                    if count %2==0{
                                        x = mk.clone();
                                    }else{
                                        store.push([x as f64,mk.clone() as f64]);
                                    }
                                    count+=1;
                                }
                                for j in store{
                                    let last= t.get(t.len()-1).unwrap().clone();
                                    let new_point = if let Position::Absolute = _position{
                                        [j[0] as f64, j[1] as f64]
                                    }else{
                                        [j[0] as f64 +last[0],j[1] as f64 +last[1]]
                                    };
                                    t.push(new_point);
                                }
                            }
                        },
                        
                        Command::HorizontalLine(_position,_p) => {
                            let k = _p.clone();
                            
                            if let WidgetType::PointPath(t,_,_) = &mut current_p{
                                let last= t.get(t.len()-1).unwrap().clone();
                                for ki in k.iter() {
                                    let new_x = if let Position::Absolute = _position{
                                        ki.clone() as f64
                                    }else{
                                        ki.clone() as f64 +last[0]
                                    };
                                    t.push([new_x,last[1]] );
                                }
                            }
                        },
                        Command::VerticalLine(_position,_p) => {
                            let k = _p.clone();
                            
                            if let WidgetType::PointPath(t,_,_) = &mut current_p{
                                let last= t.get(t.len()-1).unwrap().clone();
                                for ki in k.iter() {
                                    let new_y = if let Position::Absolute = _position{
                                            ki.clone() as f64
                                        }else{
                                            ki.clone() as f64+last[1]
                                        };
                                    
                                    t.push([last[0],new_y] );
                                }
                            }
                        },
                        Command::QuadraticCurve(_position,_p) => {
                            let k = _p.clone();
                            if let WidgetType::PointPath(t,_,_) = &mut current_p{
                                let last= t.get(t.len()-1).unwrap().clone();
                                t.push([last[0],k[0] as f64] );
                            }
                        },
                        Command::EllipticalArc(_position,_p) => {
                            let k = _p.clone();
                            let (rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y) =(k[0],k[1],k[2],k[3],k[4],k[5],k[6]);
                            let large_arc_flag_bool = if large_arc_flag==1.0{
                                true
                            }else{
                                false
                            };
                            let sweep_flag_bool = if sweep_flag==1.0{
                                true
                            }else{
                                false
                            };
                            
                            let mut rxry = [rx as f64,ry as f64];
                            if let WidgetType::PointPath(t,_,_) = &mut current_p{
                                let last= t.get(t.len()-1).unwrap().clone();
                                let mut new_point=[0.0,0.0];
                                if let Position::Absolute = _position{
                                    new_point[0]=x as f64;
                                    new_point[1]=y as f64;
                                }else{
                                    new_point[0] = last[0]+x as f64;
                                    new_point[1] = last[1]+y as f64;
                                }
                                let (center,radiance) = endpoint_to_center_arc_params(last,new_point,&mut rxry,x_axis_rotation as f64,large_arc_flag_bool,sweep_flag_bool);
                                let section = radiance[1];
                                let rect = Rect::from_corners([center[0]-rx as f64,center[1]-ry as f64],[center[0]+rx as f64,center[1]+ry as f64]);
                                let cir = Circumference::new_section(rect,20,section).offset_radians(radiance[0]);
                                t.extend(cir);
                            }
                            
                        },
                        Command::CubicCurve(_position,_p)=>{
                            let k = _p.clone();
                            if let WidgetType::PointPath(t,_,_) = &mut current_p{
                                let last= t.get(t.len()-1).unwrap().clone();
                                let mut count =0;
                                let mut store= vec![];
                                let mut x:f64=0.0;
                                for mk in k.iter(){
                                    if count %2==0{
                                        x = if let Position::Absolute = _position{
                                            mk.clone() as f64
                                        }else{
                                            last[0] +mk.clone() as f64
                                        };
                                    }else{
                                        let o = if let Position::Absolute = _position{
                                            mk.clone() as f64
                                        }else{
                                            last[1] +mk.clone() as f64
                                        };
                                        store.push([x,o]);
                                    }
                                    count+=1;
                                }
                                last_control_point = Some(store.get(1).unwrap().clone());
                                let points = draw_cubic(store);
                                for p in points{
                                    t.push(p);
                                }   
                            }

                        },
                        Command::SmoothCubicCurve(position,_p)=>{
                            let k = _p.clone();
                            if let WidgetType::PointPath(t,_,_) = &mut current_p{
                                let last= t.get(t.len()-1).unwrap().clone();
                                let mut count =0;
                                let mut store= vec![];
                                let mut x:f64=0.0;
                                for mk in k.iter(){
                                    if count %2==0{
                                        x = if let Position::Absolute = position{
                                            mk.clone() as f64
                                        }else{
                                            last[0] +mk.clone() as f64
                                        };
                                    }else{
                                        let o = if let Position::Absolute = position{
                                            mk.clone() as f64
                                        }else{
                                            last[1] +mk.clone() as f64
                                        };
                                        store.push([x,o]);
                                    }
                                    count+=1;
                                }

                                let points = draw_smooth_cubic(last_control_point.clone(),store);
                                for p in points{
                                    t.push(p);
                                }   
                            }
                        },
                        _ => {}
                    }
                }
            },
            Event::Tag(Polygon, _, attributes) => {
                let data = attributes.get("points").unwrap();
                let points:Vec<[f64;2]> = data.split(" ").filter(|x|x!=&"").map(|x|{
                    let k:Vec<&str> = x.split(",").collect();
                    [k.get(0).unwrap().parse::<f64>().unwrap(),-k.get(1).unwrap().parse::<f64>().unwrap()]
                }).collect();
                vec_widget.push(WidgetType::Polygon(points));
            },
            _ => {}
        }
    }
    return (vec_widget,[svgheight.unwrap(),svgwidth.unwrap()]);
}
fn endpoint_to_center_arc_params(p1:[f64;2],p2:[f64;2],r_:&mut [f64;2],x_angle:f64,flag_a:bool,flag_s:bool)->([f64;2],[f64;2]){
    let mut r_x = r_[0].abs();
    let mut r_y = r_[1].abs();
    let dx2 = (p1[0] - p2[0])/2.0;
    let dy2 = (p1[1] - p2[1])/2.0;
    let x1p = x_angle.cos()*dx2 + x_angle.sin()*dy2;
    let y1p = -x_angle.sin()*dx2 + x_angle.cos()*dy2;
    let mut rxs = r_x * r_x;
    let mut rys = r_y * r_y;
    let x1ps = x1p*x1p;
    let y1ps = y1p*y1p;
    let cr = x1ps/rxs + y1ps/rys;
    if cr > 1.0 {
        //scale up r_x,r_y equally so cr == 1
        let s = cr.sqrt();
        r_x = s * r_x;
        r_y = s * r_y;
        rxs = r_x * r_x;
        rys = r_y * r_y;
    }
    let dq = rxs * y1ps + rys * x1ps;
    let pq = (rxs*rys - dq) / dq;
    let mut q = ((cmp::max(0,pq as i64)) as f64).sqrt(); //use Max to account for float precision
    if flag_a == flag_s{
        q = -q;
    }
        
    let cxp = q * r_x * y1p / r_y;
    let cyp = - q * r_y * x1p / r_x;
    let cx = x_angle.cos()*cxp - x_angle.sin()*cyp + (p1[0] + p2[0])/2.0;
    let cy = x_angle.sin()*cxp + x_angle.cos()*cyp + (p1[1] + p2[1])/2.0;
    let theta = svg_angle( 1.0,0.0, (x1p-cxp) / r_x, (y1p - cyp)/r_y );
    let mut delta = svg_angle(
        (x1p - cxp)/r_x, (y1p - cyp)/r_y,
        (-x1p - cxp)/r_x, (-y1p-cyp)/r_y);

    delta = delta % (std::f64::consts::PI * 2.0);
    
    if !flag_s{
        delta -= 2.0 * std::f64::consts::PI ;
    }

    let c = [cx,cy];
    //let angles = [theta, delta];
    let angles =[theta,delta];
    return (c,angles);
}

fn svg_angle( ux:f64,uy:f64, vx:f64, vy:f64 ) ->f64
{
    let u = nalgebra::base::Matrix1x2::new(ux, uy);
    let v = nalgebra::base::Matrix1x2::new(vx, vy);
    //(F.6.5.4)
    let dot = u.dot(&v);
    let len = u.magnitude() *v.magnitude();
    let mut ang = (clamp( dot / len,-1.0,1.0)).acos(); //floating point precision, slightly over values appear
    if  (ux*vy - uy*vx) < 0.0{
        ang = -ang;
    }
    return ang;
}
#[inline]
pub fn is_triangle_convex(a: &[f64; 2], b: &[f64; 2], c: &[f64; 2]) -> bool
{
    ((a[1] - b[1]) * (c[0] - b[0]) + (b[0] - a[0]) * (c[1] - b[1])) >= 0.0
}
fn draw_cubic(control:Vec<[f64;2]>)->Vec<[f64;2]>{
    //https://community.khronos.org/t/how-to-draw-a-bezier-curve-through-its-control-points/21804
    let num_points = 5;
    //let dt = (control.len() ) as f64 /(num_points) as f64;
    let dt = 1.0 /(num_points) as f64;
    let p1 = control.get(0).unwrap();
    let p2 = control.get(1).unwrap();
    let p3 = control.get(2).unwrap();
    let mut points =vec![]; 
    for i in 0..num_points{
        let t = i as f64 *dt;
        let c= cubic(&mut [0.0,0.0],&p1,&p2,&p3,&t).clone();
        points.push(c);
    }
    return points;
}
fn draw_smooth_cubic(last_control_pt:Option<[f64;2]>,control:Vec<[f64;2]>)->Vec<[f64;2]>{
    let num_points = 5;
    let dt = 1.0 /(num_points-1) as f64;
    let p2 = control.get(0).unwrap();
    let p1 = if let Some(k) = last_control_pt{
        k
    }else{
        p2.clone()
    };
    let p3 = control.get(1).unwrap();

    let mut points =vec![]; 
    for i in 0..num_points{
        let t = i as f64 *dt;
        let c= cubic(&mut [0.0,0.0],&p1,&p2,&p3,&t).clone();
        points.push(c);
    }
    return points;

}