pub mod extended_table;
use std::io::Stdout;

pub use extended_table::*;
use num::{FromPrimitive, ToPrimitive};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    Frame,
};

use crate::automaton::State;

pub mod custom_border_separator;
pub use custom_border_separator::*;

pub trait Renderable {
    fn draw_screen(state: &State, f: &mut Frame<CrosstermBackend<Stdout>>);
}

pub fn get_time_style<T: FromPrimitive + PartialOrd>(value: T) -> Style {
    let style = Style::default();
    if value < FromPrimitive::from_u64(20000000).unwrap() {
        style.fg(Color::White).add_modifier(Modifier::DIM)
    } else if value < FromPrimitive::from_u64(50000000).unwrap() {
        style
            .fg(Color::Rgb(255, 165, 0))
            .add_modifier(Modifier::DIM) // orange
    } else {
        style.fg(Color::LightRed).add_modifier(Modifier::DIM)
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

// TODO: combine those two, and edit occurences
pub fn convert_time_to_unit_string_option<T>(time: Option<T>) -> String
where
    T: ToPrimitive + PartialOrd + std::ops::Div<Output = T> + std::fmt::Display,
{
    if let Some(time) = time {
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
    } else {
        String::from(" - ")
    }
}

#[derive(Debug, Clone)]
pub struct StyledTime<T> {
    value: T,
    pub style: Style,
    pub string_representation: String,
}

impl<T> StyledTime<T>
where
    T: ToPrimitive
        + PartialOrd
        + std::ops::Div<Output = T>
        + std::fmt::Display
        + FromPrimitive
        + PartialOrd
        + Copy
        + Default,
{
    pub fn new(value: Option<T>) -> Self {
        if let Some(value) = value {
            Self {
                value,
                style: Self::get_time_style(value),
                string_representation: Self::convert_time_to_unit_string(value),
            }
        } else {
            Self {
                value: Default::default(),
                style: Style::default(),
                string_representation: String::from(" - "),
            }
        }
    }

    fn convert_time_to_unit_string(time: T) -> String {
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

    fn get_time_style(value: T) -> Style {
        let style = Style::default();
        if value < FromPrimitive::from_u64(20000000).unwrap() {
            style.fg(Color::White).add_modifier(Modifier::DIM)
        } else if value < FromPrimitive::from_u64(50000000).unwrap() {
            style
                .fg(Color::Rgb(255, 165, 0))
                .add_modifier(Modifier::DIM) // orange
        } else {
            style.fg(Color::LightRed).add_modifier(Modifier::DIM)
        }
    }

    pub fn get_value(&self) -> T {
        self.value
    }

    pub fn get_style(&self) -> Style {
        self.style
    }

    pub fn get_string_representation(&self) -> String {
        self.string_representation.clone()
    }
}
