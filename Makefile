setup: npm-i cargo-build run-project

run-project:
	(cd ./frontend && npm run serve) & (cd ./backend && cargo run) && fg

npm-i:
	cd ./frontend && npm i

cargo-build:
	cd ./backend && cargo clean && cargo build