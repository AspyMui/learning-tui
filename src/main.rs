// HELLO THEO!!!

fn main() -> Result<(), std::io::Error> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = tui::widgets::Block::default()
            .title("Block")
            .borders(tui::widgets::Borders::ALL);
        f.render_widget(block, size);
    })?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
