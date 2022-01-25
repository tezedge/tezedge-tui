pub mod extended_table;
use std::io::Stdout;

pub use extended_table::*;
use num::{FromPrimitive, ToPrimitive};
use tui::{backend::CrosstermBackend, style::Color, Frame};

use crate::automaton::State;

pub trait Renderable {
    fn draw_screen(state: &State, f: &mut Frame<CrosstermBackend<Stdout>>);
}

pub fn get_color<T: FromPrimitive + PartialOrd>(value: T) -> Color {
    if value < FromPrimitive::from_u64(20000000).unwrap() {
        Color::Reset
    } else if value < FromPrimitive::from_u64(50000000).unwrap() {
        Color::Rgb(255, 165, 0) // orange
    } else {
        Color::Red
    }
}

pub fn convert_time_to_unit_string<T>(time: T) -> String
where
    T: ToPrimitive + PartialOrd + std::ops::Div<Output = T> + std::fmt::Display,
{
    let time = if let Some(time) = time.to_f64() {
        time
    } else {
        return String::from("NaN")
    };

    let millisecond_factor: f64 = 1000.0.to_f64().unwrap();
    let microsecond_factor: f64 = 1000000.0.to_f64().unwrap();
    let nanosecond_factor: f64 = 1000000000.0.to_f64().unwrap();

    if time >= nanosecond_factor {
        format!("{:.2}s", time / nanosecond_factor)
    } else if time >= microsecond_factor {
        format!("{:.2}ms", time / microsecond_factor)
    } else if time >= millisecond_factor {
        format!("{:.2}μs", time / millisecond_factor)
    } else {
        format!("{}ns", time)
    }
}
