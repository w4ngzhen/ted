use ratatui::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::io;
use std::path::PathBuf;
use ted_editor::vim::vim_editor::VimEditor;
use ted_fs_explorer::fs_explorer::FsExplorer;
use ted_layout::TedLayoutManager;

fn main() -> io::Result<()> {
    let mut term = ratatui::init();

    let mut layout_manager = TedLayoutManager::default();
    layout_manager.set_left_aside_width(20);

    let mut fs_explorer = FsExplorer::new(PathBuf::from(
        "/Users/w4ngzhen/projects/rust-projects/ted/crates/ted_fs_explorer/test",
    ))?;

    let mut vim_editor = VimEditor::new();

    let mut focus_editor = true;

    loop {
        term.draw(|f| {
            let ted_layout = layout_manager.build(f.area());
            vim_editor.draw(f, ted_layout.content);
            fs_explorer.draw(f, ted_layout.aside_left);
        })?;

        let event = ratatui::crossterm::event::read()?;

        match event {
            Event::Key(key) if !matches!(key.kind, KeyEventKind::Press) => {
                ();
            }
            Event::Key(KeyEvent { code, modifiers, .. }) => {
                match (code, modifiers) {
                    (KeyCode::F(1), _) => {
                        focus_editor = !focus_editor;
                    }
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        break;
                    }
                    _ => {
                        if focus_editor {
                            vim_editor.handle_event(event);
                        } else {
                            fs_explorer.handle_event(event);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    ratatui::restore();

    Ok(())
}
