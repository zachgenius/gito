use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    History,
    Status,
    Branches,
    Stashes,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::History => 0,
            MenuItem::Status => 1,
            MenuItem::Branches => 2,
            MenuItem::Stashes => 3,
        }
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let tab_titles = vec!["History", "Status", "Branches", "Stashes"];
        let mut active_tab_item = MenuItem::History;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;
    Ok(())
}
