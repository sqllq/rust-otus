lint:
	cargo check
	cargo fmt --check
	cargo clippy -- -Dclippy::all -Wclippy::pedantic --deny "warnings"