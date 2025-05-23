use ratatui::prelude::*;

pub struct TedLayout {
    pub header_left: Rect,
    pub header_right: Rect,

    pub aside_left: Rect,
    pub content: Rect,
    pub aside_right: Rect,

    pub footer_left: Rect,
    pub footer_right: Rect,
}

#[derive(Default)]
pub struct TedLayoutManager {
    left_aside_width: u16,
    prev_left_aside_width: u16,
    right_aside_width: u16,
    prev_right_aside_width: u16,
}

impl TedLayoutManager {
    pub fn build(&self, area: Rect) -> TedLayout {
        let [header_rect, body_rect, footer_rect] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Percentage(100),
            Constraint::Length(4),
        ])
        .areas(area);
        let [header_left, header_right] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .areas(header_rect);
        let [footer_left, footer_right] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .areas(footer_rect);
        let [aside_left, content, aside_right] = Layout::horizontal([
            Constraint::Length(self.left_aside_width),
            Constraint::Percentage(100),
            Constraint::Length(self.right_aside_width),
        ])
        .areas(body_rect);
        TedLayout {
            header_left,
            header_right,
            aside_left,
            content,
            aside_right,
            footer_left,
            footer_right,
        }
    }
    pub fn set_left_aside_width(&mut self, width: u16) {
        self.prev_left_aside_width = self.left_aside_width;
        self.left_aside_width = width.max(1);
    }
    pub fn set_right_aside_width(&mut self, width: u16) {
        self.prev_right_aside_width = self.right_aside_width;
        self.right_aside_width = width.max(1);
    }

    pub fn hide_left_aside(&mut self) {
        self.prev_left_aside_width = self.left_aside_width;
        self.left_aside_width = 0;
    }
    pub fn show_left_aside(&mut self) {
        self.left_aside_width = self.prev_left_aside_width;
    }
    pub fn hide_right_aside(&mut self) {
        self.prev_right_aside_width = self.right_aside_width;
        self.right_aside_width = 0;
    }
    pub fn show_right_aside(&mut self) {
        self.right_aside_width = self.prev_right_aside_width;
    }
}
