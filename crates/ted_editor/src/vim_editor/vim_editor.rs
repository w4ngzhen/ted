use edtui::{EditorEventHandler, EditorState, EditorView, SyntaxHighlighter};
use ratatui::crossterm::event::Event;
use ratatui::prelude::*;

pub struct VimEditor {
    pub state: EditorState,
    pub event_handler: EditorEventHandler,
}

impl VimEditor {
    pub fn new() -> Self {
        Self {
            state: EditorState::default(),
            event_handler: EditorEventHandler::default(),
        }
    }

    pub fn handle_events(&mut self, event: Event) {
        self.event_handler.on_event(event, &mut self.state);
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl Widget for &mut VimEditor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let syntax_highlighter = SyntaxHighlighter::new("dracula", "rs");
        EditorView::new(&mut self.state)
            .wrap(true)
            .syntax_highlighter(Some(syntax_highlighter))
            .render(area, buf)
    }
}
