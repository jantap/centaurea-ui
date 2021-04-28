use std::io;

use crossterm::{cursor::Hide, execute};

use super::BufUiError;

pub fn prepare_terminal() -> Result<io::Stdout, BufUiError> {
	let mut stdout = io::stdout();
	crossterm::execute!(
		stdout,
		crossterm::terminal::EnterAlternateScreen,
		crossterm::event::EnableMouseCapture
	)?;
	crossterm::terminal::enable_raw_mode()?;
	execute!(stdout, Hide)?;

	Ok(stdout)
}

pub fn clean_terminal() -> Result<(), BufUiError> {
	let mut stdout = std::io::stdout();
	crossterm::execute!(
		stdout,
		crossterm::cursor::Show,
		crossterm::terminal::LeaveAlternateScreen,
		crossterm::event::DisableMouseCapture
	)?;
	crossterm::terminal::disable_raw_mode()?;

	Ok(())
}
