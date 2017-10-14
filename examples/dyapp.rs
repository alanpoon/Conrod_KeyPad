#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate conrod_keypad;
extern crate image;
#[cfg(target_os="android")]
extern crate rusttype;
#[cfg(target_os="android")]
extern crate android_glue;
#[cfg(not(target_os="android"))]
extern crate find_folder;

pub mod support;
use conrod::{widget, color, Colorable, Widget, Positionable, Sizeable};
use conrod::backend::glium::glium::{self, glutin, Surface};
#[cfg(feature="hotload")]
use conrod_keypad::dyapplication as application;
use conrod_keypad::custom_widget::keypad;
use conrod_keypad::english;
const LIB_PATH: &'static str = "target/debug/libtest_shared.so";
widget_ids! {
    pub struct Ids {
         master,
         keyboard,
         image,
         text_edit
    }
}
fn main() {
    let window = glutin::WindowBuilder::new();
    let context =
        glium::glutin::ContextBuilder::new()
            .with_gl(glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGlEs, (3, 0)));
    let mut events_loop = glutin::EventsLoop::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    // construct our `Ui`.
    let (screen_w, screen_h) = display.get_framebuffer_dimensions();
    let mut ui = conrod::UiBuilder::new([screen_w as f64, screen_h as f64]).build();
    ui.fonts.insert(support::assets::load_font("fonts/NotoSans/NotoSans-Regular.ttf"));
    let rust_logo = load_image(&display, "images/rust.png");
    let keypad_png = load_image(&display, "images/keypad.png");
    //  let (w, h) = (rust_logo.get_width(), rust_logo.get_height().unwrap());
    let mut image_map: conrod::image::Map<glium::texture::Texture2d> = conrod::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);
    let (w, h) = (keypad_png.get_width(), keypad_png.get_height().unwrap());
    let keypad_png = image_map.insert(keypad_png);
    let events_loop_proxy = events_loop.create_proxy();
    let mut events = Vec::new();
    let mut ids = Ids::new(ui.widget_id_generator());
    let mut app = application::Application::new(LIB_PATH);
    let mut demo_text_edit = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
            Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
            finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
            fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
            Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
            Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
            magna est, efficitur suscipit dolor eu, consectetur consectetur urna.".to_owned();
    let mut last_update = std::time::Instant::now();
    let mut last_update_sys = std::time::SystemTime::now();
    let mut c = 0;
    let mut keypadvariant = keypad::KeyPadVariant::Letter(1);
    'render: loop {
        let sixteen_ms = std::time::Duration::from_millis(100);
        let now = std::time::Instant::now();
        let duration_since_last_update = now.duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }
        application::Application::in_loop(&mut app, LIB_PATH, &mut last_update_sys);
        let (string_vec, num_vec) = english::populate(keypad_png, app.get_spriteinfo());
        events.clear();

        // Get all the new events since the last frame.
        events_loop.poll_events(|event| { events.push(event); });
        // Process the events.
        for event in events.drain(..) {

            // Break from the loop upon `Escape` or closed window.
            match event.clone() {

                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::Closed |
                            glium::glutin::WindowEvent::KeyboardInput {
                                input: glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => break 'render,
                        _ => (),
                    }
                }
                _ => (),
            };

            // Use the `winit` backend feature to convert the winit event to a conrod input.
            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };
            let f = "alan".to_owned();
            // Handle the input with the `Ui`.
            ui.handle_event(input);
            // Set the widgets.
            let ui = &mut ui.set_widgets();
            widget::Canvas::new().color(color::LIGHT_BLUE).set(ids.master, ui);
            for edit in widget::TextEdit::new(&mut demo_text_edit)
            .color(color::WHITE)
            .padded_w_of(ids.master, 20.0)
            .mid_top_of(ids.master)
            .center_justify()
            .line_spacing(2.5)
            .restrict_to_height(false) // Let the height grow infinitely and scroll.
            .set(ids.text_edit, ui) {
                demo_text_edit = edit;
            }
            let screen_dim = ui.wh_of(ids.master).unwrap();
            let h = keypad::KeyPadView::new(&mut demo_text_edit,
                                            &mut keypadvariant,
                                            &string_vec,
                                            &num_vec,
                                            app.get_keyboard_styles([screen_dim[0],
                                                                     screen_dim[1] * 0.4]));
            h.mid_bottom_of(ids.master)
                .w(screen_dim[0])
                .h(screen_dim[1] * 0.4)
                .set(ids.keyboard, ui);
            c += 1;
        }

        let primitives = ui.draw();
        renderer.fill(&display, primitives, &image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(&display, &mut target, &image_map).unwrap();
        target.finish().unwrap();
        last_update = std::time::Instant::now();
    }
}
fn load_image(display: &glium::Display, path: &str) -> glium::texture::Texture2d {
    let rgba_image = support::assets::load_image(path).to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(),
                                                                       image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}
