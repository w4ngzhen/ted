use ratatui::prelude::*;

pub struct TedLayout {
    pub header_left: Rect,
    pub header_right: Rect,
    pub sidebar_left: Rect,
    pub sidebar_left_content: Rect,
    pub content: Rect,
    pub sidebar_right_content: Rect,
    pub sidebar_right: Rect,
    pub footer_left: Rect,
    pub footer_right: Rect,
}

#[derive(Default)]
pub struct TedLayoutBuilder {
    left_sidebar_content_width: u16,
    prev_left_sidebar_content_width: u16,
    right_sidebar_content_width: u16,
    prev_right_sidebar_content_width: u16,
}

impl TedLayoutBuilder {
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
        let [
            sidebar_left,
            sidebar_left_content,
            content,
            sidebar_right_content,
            sidebar_right,
        ] = Layout::horizontal([
            Constraint::Length(4),
            Constraint::Length(self.left_sidebar_content_width),
            Constraint::Percentage(100),
            Constraint::Length(self.right_sidebar_content_width),
            Constraint::Length(4),
        ])
        .areas(body_rect);
        TedLayout {
            header_left,
            header_right,
            sidebar_left,
            sidebar_left_content,
            content,
            sidebar_right_content,
            sidebar_right,
            footer_left,
            footer_right,
        }
    }
    pub fn set_left_sidebar_content_width(&mut self, width: u16) {
        self.prev_left_sidebar_content_width = self.left_sidebar_content_width;
        self.left_sidebar_content_width = width.max(1);
    }
    pub fn set_right_sidebar_content_width(&mut self, width: u16) {
        self.prev_right_sidebar_content_width =
            self.right_sidebar_content_width;
        self.right_sidebar_content_width = width.max(1);
    }

    pub fn hide_left_sidebar_content(&mut self) {
        self.prev_left_sidebar_content_width = self.left_sidebar_content_width;
        self.left_sidebar_content_width = 0;
    }
    pub fn show_left_sidebar_content(&mut self) {
        self.left_sidebar_content_width = self.prev_left_sidebar_content_width;
    }
    pub fn hide_right_sidebar_content(&mut self) {
        self.prev_right_sidebar_content_width =
            self.right_sidebar_content_width;
        self.right_sidebar_content_width = 0;
    }
    pub fn show_right_sidebar_content(&mut self) {
        self.right_sidebar_content_width =
            self.prev_right_sidebar_content_width;
    }
}
