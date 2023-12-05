lint:
	cargo check
	cargo fmt --check
	cargo clippy -- -Dclippy::all -Wclippy::pedantic --deny "warnings"

run:\lesson_1:
	cargo run -p lesson_id