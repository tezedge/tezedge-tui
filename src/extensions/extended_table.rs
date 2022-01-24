use itertools::Itertools;
use tui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Cell, Row, TableState},
};

const SIDE_PADDINGS: u16 = 1;
const INITIAL_PADDING: u16 = 2;

#[derive(Clone, Debug, Default)]
pub struct ExtendedTable {
    pub table_state: TableState,

    /// The header strings of the table in order
    headers: Vec<String>,

    modified_headers: Vec<String>,

    /// Constrainst of the colums
    constraints: Vec<Constraint>,

    /// Total number of indexex able to be rendered
    rendered: usize,

    /// Always render content this number of content starting from index 0
    fixed_count: usize,

    /// First index to be rendered after the last fixed index
    first_rendered_index: usize,

    /// selected table column
    selected: usize,

    /// The index the table is sorted by
    sorted_by: Option<usize>,

    /// Sort order
    sort_order: SortOrder,
}

impl ExtendedTable {
    pub fn new(headers: Vec<String>, constraints: Vec<Constraint>, fixed_count: usize) -> Self {
        Self {
            headers: headers.clone(),
            modified_headers: headers,
            constraints,
            fixed_count,
            rendered: 0,
            first_rendered_index: fixed_count,
            selected: 0,
            sorted_by: None,
            sort_order: SortOrder::Unsorted,
            ..Default::default()
        }
    }

    pub fn sorted_by(&self) -> Option<usize> {
        self.sorted_by
    }

    pub fn sort_order(&self) -> &SortOrder {
        &self.sort_order
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn rendered(&self) -> usize {
        self.rendered
    }

    pub fn fixed(&self) -> usize {
        self.fixed_count
    }

    pub fn first_rendered_index(&self) -> usize {
        self.first_rendered_index
    }

    pub fn set_first_rendered_index(&mut self, first_rendered_index: usize) {
        self.first_rendered_index = first_rendered_index;
    }

    pub fn set_rendered(&mut self, rendered: usize) {
        self.rendered = rendered
    }

    pub fn set_fixed(&mut self, fixed: usize) {
        self.fixed_count = fixed
    }

    pub fn set_sort_order(&mut self, sort_order: SortOrder) {
        self.sort_order = sort_order
    }

    pub fn set_sorted_by(&mut self, sorted_by: Option<usize>) {
        self.sorted_by = sorted_by
    }

    pub fn next(&mut self) {
        let last_render_index = self.first_rendered_index + (self.rendered - self.fixed_count) - 1;
        let next_index = self.selected + 1;
        if next_index < self.headers.len() {
            self.selected = next_index
        }

        if self.selected >= last_render_index
            && self.first_rendered_index != last_render_index
            && self.rendered != self.headers.len()
        {
            self.first_rendered_index += 1;
        }
    }

    pub fn sort_content<T: SortableByFocus>(
        &mut self,
        content: &mut T,
        sort_by: usize,
        sort_order: &SortOrder,
        delta_toggle: bool,
    ) {
        self.sorted_by = Some(sort_by);
        self.sort_order = sort_order.clone();

        content.sort_by_focus(sort_by, delta_toggle);
        if let SortOrder::Descending = *sort_order {
            content.rev();
        }
    }

    pub fn previous(&mut self) {
        if self.selected != 0 && self.selected != self.headers.len() {
            self.selected -= 1;
        }

        if self.selected == self.first_rendered_index - 1
            && self.first_rendered_index != self.fixed_count
            && self.rendered != self.headers.len()
        {
            self.first_rendered_index -= 1;
        }
    }

    pub fn highlight_sorting(&mut self) {
        let mut headers = self.headers.clone();

        // add ▼/▲ to the selected sorted table
        if let Some(sorted_by) = self.sorted_by {
            if let Some(v) = headers.get_mut(sorted_by) {
                match self.sort_order {
                    SortOrder::Ascending => *v = format!("{}▲", v),
                    SortOrder::Descending => *v = format!("{}▼", v),
                    _ => {}
                }
            }
        }
        self.modified_headers = headers;
    }

    pub fn renderable_constraints(&mut self, max_size: u16) -> Vec<Constraint> {
        let mut acc: u16 = INITIAL_PADDING
            + self
                .constraints
                .iter()
                .take(self.fixed_count)
                .map(|c| {
                    if let Constraint::Min(unit) = c {
                        *unit
                    } else {
                        0
                    }
                })
                .reduce(|mut acc, unit| {
                    acc += unit;
                    acc
                })
                .unwrap_or(0);

        let mut to_render: Vec<Constraint> = self
            .constraints
            .iter()
            .take(self.fixed_count)
            .cloned()
            .collect();

        let dynamic_to_render: Vec<Constraint> = self
            .constraints
            .iter()
            .skip(self.first_rendered_index)
            .take_while_ref(|constraint| {
                if let Constraint::Min(unit) = constraint {
                    acc += unit + SIDE_PADDINGS;
                    acc <= max_size
                } else {
                    // TODO
                    false
                }
            })
            .cloned()
            .collect();

        to_render.extend(dynamic_to_render);

        self.rendered = to_render.len();
        to_render
    }

    pub fn renderable_headers(&self, selected_style: Style) -> Vec<Cell> {
        let selected = self.selected;
        let fixed_header_cells = self
            .modified_headers
            .iter()
            .enumerate()
            .take(self.fixed_count)
            .map(|(index, h)| {
                if index == selected {
                    Cell::from(h.as_str()).style(selected_style)
                } else {
                    Cell::from(h.as_str()).style(Style::default())
                }
            });

        let dynamic_header_cells = self
            .modified_headers
            .iter()
            .enumerate()
            .skip(self.first_rendered_index)
            .map(|(index, h)| {
                if index == selected {
                    Cell::from(h.as_str()).style(selected_style)
                } else {
                    Cell::from(h.as_str()).style(Style::default())
                }
            });

        fixed_header_cells.chain(dynamic_header_cells).collect()
    }

    pub fn renderable_rows<T: TuiTableData>(&self, content: &[T], delta_toggle: bool) -> Vec<Row> {
        content
            .iter()
            .map(|item| {
                let item = item.construct_tui_table_data(delta_toggle);
                let height = item
                    .iter()
                    .map(|(content, _)| content.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0)
                    + 1;
                let fixed_cells = item.iter().take(self.fixed_count).map(|(content, color)| {
                    Cell::from(content.clone()).style(Style::default().fg(*color))
                });
                let dynamic_cells =
                    item.iter()
                        .skip(self.first_rendered_index)
                        .map(|(content, color)| {
                            Cell::from(content.clone()).style(Style::default().fg(*color))
                        });
                let cells = fixed_cells.chain(dynamic_cells);
                Row::new(cells).height(height as u16)
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
    Unsorted,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Unsorted
    }
}

pub trait TuiTableData {
    fn construct_tui_table_data(&self, delta_toggle: bool) -> Vec<(String, Color)>;
}

pub trait SortableByFocus {
    fn sort_by_focus(&mut self, focus_index: usize, delta_toogle: bool);
    fn rev(&mut self);
}
