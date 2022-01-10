pub mod mempool_model;
pub use mempool_model::*;

pub mod state;
pub use state::*;

pub mod syncing_model;
pub use syncing_model::*;

pub mod operation_statistics_model;
pub use operation_statistics_model::*;

pub trait SortableByFocus {
    fn sort_by_focus(&mut self, focus_index: usize);
}

pub fn convert_time_to_unit_string(time: u64) -> String {
    let time = time as f64;
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
