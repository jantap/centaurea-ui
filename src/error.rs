use thiserror::Error;

#[derive(Debug, Error)]
pub enum BufUiError {
	#[error("terminal window is too small")]
	TerminalTooSmall,
	#[error("crossterm terminal error")]
	TerminalError(#[from] crossterm::ErrorKind),
	#[error("terminal io error")]
	IoError(#[from] std::io::Error),
}
