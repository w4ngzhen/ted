use crate::vim_lite_editor::vim_state::{Mode, Transition, VimState};
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::prelude::*;
use tui_textarea::TextArea;

pub struct VimLiteEditor<'a> {
    textarea: TextArea<'a>,
    vim_state: VimState,
}

impl<'a> VimLiteEditor<'a> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_block(Mode::Normal.block());
        textarea.set_cursor_style(Mode::Normal.cursor_style());
        let vim_state = VimState::new(Mode::Normal);
        Self { textarea, vim_state }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }

    pub fn handle_event(&mut self, event: Event) {
        let curr_state = self.vim_state.clone();
        self.vim_state = match curr_state
            .transition(event.into(), &mut self.textarea)
        {
            Transition::Mode(next_mode) if curr_state.mode() != next_mode => {
                self.textarea.set_cursor_style(next_mode.cursor_style());
                VimState::new(next_mode)
            }
            Transition::Nop | Transition::Mode(_) => curr_state,
            Transition::Pending(input) => curr_state.with_pending(input),
            Transition::Quit => curr_state,
        }
    }
}

impl<'a> Widget for &mut VimLiteEditor<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.textarea.render(area, buf);
    }
}
