use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::Style,
    widgets::Widget,
};

pub struct CustomSeparator<'a> {
    separator: &'a str,
    corner: Corner,
    style: Style,
}

impl<'a> Default for CustomSeparator<'a> {
    fn default() -> Self {
        Self {
            separator: Default::default(),
            corner: Corner::TopLeft,
            style: Style::default(),
        }
    }
}

impl<'a> Widget for CustomSeparator<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }
        match self.corner {
            Corner::TopLeft => {
                buf.get_mut(area.left(), area.top())
                    .set_symbol(self.separator)
                    .set_style(self.style);
            }
            Corner::TopRight => {
                buf.get_mut(area.right() - 1, area.top())
                    .set_symbol(self.separator)
                    .set_style(self.style);
            }
            Corner::BottomRight => {
                buf.get_mut(area.right() - 1, area.bottom() - 1)
                    .set_symbol(self.separator)
                    .set_style(self.style);
            }
            Corner::BottomLeft => {
                buf.get_mut(area.left(), area.bottom() - 1)
                    .set_symbol(self.separator)
                    .set_style(self.style);
            }
        }
    }
}

impl<'a> CustomSeparator<'a> {
    pub fn separator(mut self, separator: &'a str) -> CustomSeparator<'a> {
        self.separator = separator;
        self
    }

    pub fn corner(mut self, corner: Corner) -> CustomSeparator<'a> {
        self.corner = corner;
        self
    }

    pub fn style(mut self, style: Style) -> CustomSeparator<'a> {
        self.style = style;
        self
    }
}
