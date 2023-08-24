nothing:



build:
	cargo build

run:
	cargo run

test:
	cargo test # -- --nocapture

lint:
	cargo clippy

push:
	git commit -m s -a
	git push
	
