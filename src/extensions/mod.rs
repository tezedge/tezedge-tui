pub mod extended_table;
use std::io::Stdout;

pub use extended_table::*;
use num::{FromPrimitive, ToPrimitive};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    Frame,
};

use crate::automaton::State;

pub trait Renderable {
    fn draw_screen(state: &State, f: &mut Frame<CrosstermBackend<Stdout>>);
}

pub fn get_time_style<T: FromPrimitive + PartialOrd>(value: T) -> Style {
    let style = Style::default();
    if value < FromPrimitive::from_u64(20000000).unwrap() {
        style.fg(Color::White)
    } else if value < FromPrimitive::from_u64(50000000).unwrap() {
        style.fg(Color::Rgb(255, 165, 0)) // orange
    } else {
        style.fg(Color::Red)
    }
}

pub fn convert_time_to_unit_string<T>(time: T) -> String
where
    T: ToPrimitive + PartialOrd + std::ops::Div<Output = T> + std::fmt::Display,
{
    let time = if let Some(time) = time.to_f64() {
        time
    } else {
        return String::from("NaN");
    };

    const MILLISECOND_FACTOR: f64 = 1000.0;
    const MICROSECOND_FACTOR: f64 = 1000000.0;
    const NANOSECOND_FACTOR: f64 = 1000000000.0;

    if time >= NANOSECOND_FACTOR {
        format!("{:.2}s", time / NANOSECOND_FACTOR)
    } else if time >= MICROSECOND_FACTOR {
        format!("{:.2}ms", time / MICROSECOND_FACTOR)
    } else if time >= MILLISECOND_FACTOR {
        format!("{:.2}μs", time / MILLISECOND_FACTOR)
    } else {
        format!("{}ns", time)
    }
}
