#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
extern crate cardgame_widgets;
#[cfg(feature="hotload")]
extern crate libloading;
#[cfg(feature="hotload")]
pub mod dyapplication;
#[cfg(feature="hotload")]
pub use dyapplication as application;

pub mod english;
pub mod custom_widget;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
