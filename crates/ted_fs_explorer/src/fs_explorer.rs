use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Scrollbar, ScrollbarOrientation};
use std::io;
use std::path::PathBuf;
use tui_tree_widget::{Tree, TreeItem, TreeState};

pub struct FsExplorer {
    state: TreeState<String>,
    root_item: Vec<TreeItem<'static, String>>,
}
impl FsExplorer {
    pub fn new(root_path: PathBuf) -> io::Result<FsExplorer> {
        fn build_items(
            path: &PathBuf,
            curr_level: u8,
            max_level: u8,
        ) -> io::Result<TreeItem<'static, String>> {
            let file_name =
                path.file_name().unwrap().to_owned().into_string().unwrap();
            let path_str = path.to_str().expect("");
            if path.is_dir() && curr_level <= max_level {
                let mut children = vec![];
                let dir = path.read_dir()?;
                for entry in dir {
                    let entry = entry?;
                    let path_in_dir = entry.path();
                    let child_item =
                        build_items(&path_in_dir, curr_level + 1, max_level)?;
                    children.push(child_item);
                }
                TreeItem::new(path_str.to_string(), file_name, children)
            } else {
                let ext = path.extension().unwrap().to_str().unwrap();
                Ok(TreeItem::new_leaf(path_str.to_string(), file_name))
            }
        }

        let root_item = if root_path.is_dir() {
            build_items(&root_path, 1, 2)?
        } else {
            let parent = root_path.parent().unwrap();
            build_items(&parent.into(), 1, 2)?
        };
        Ok(Self { state: TreeState::default(), root_item: vec![root_item] })
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let widget = Tree::new(&self.root_item)
            .expect("all item identifiers are unique")
            .block(Block::bordered().title("project"))
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ))
            .highlight_style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_stateful_widget(widget, area, &mut self.state);
    }

    pub fn handle_event(&mut self, event: Event) {
        let update = match event {
            Event::Key(key) if !matches!(key.kind, KeyEventKind::Press) => {
                false
            }
            Event::Key(key) => match key.code {
                KeyCode::Char('\n' | ' ') => self.state.toggle_selected(),
                KeyCode::Left => self.state.key_left(),
                KeyCode::Right => self.state.key_right(),
                KeyCode::Down => self.state.key_down(),
                KeyCode::Up => self.state.key_up(),
                KeyCode::Esc => self.state.select(Vec::new()),
                KeyCode::Home => self.state.select_first(),
                KeyCode::End => self.state.select_last(),
                KeyCode::PageDown => self.state.scroll_down(3),
                KeyCode::PageUp => self.state.scroll_up(3),
                _ => false,
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => self.state.scroll_down(1),
                MouseEventKind::ScrollUp => self.state.scroll_up(1),
                MouseEventKind::Down(_button) => {
                    self.state.click_at(Position::new(mouse.column, mouse.row))
                }
                _ => false,
            },
            Event::Resize(_, _) => true,
            _ => false,
        };
    }
}
