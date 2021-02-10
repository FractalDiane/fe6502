// console_colors.rs

#[macro_export]
macro_rules! con_red {
	() => {
		"\x1b[0;31m"
	};
}

#[macro_export]
macro_rules! con_yellow {
	() => {
		"\x1b[1;33m"
	};
}

#[macro_export]
macro_rules! con_green {
	() => {
		"\x1b[0;32m"
	};
}

#[macro_export]
macro_rules! con_reset {
	() => {
		"\x1b[0m"
	};
}
