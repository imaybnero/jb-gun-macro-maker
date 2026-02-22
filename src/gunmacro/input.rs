// gunmacro/input

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
	Up, Down, Left, Right,
	Enter, Backslash,
}