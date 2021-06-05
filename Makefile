build:
	cd ui && yarn build && cd ..
	cargo build
run:
	cd ui && yarn build && cd ..
	cargo run -p server
watch-webpack:
	cd ui && yarn watch
watch-server:
	cargo watch -x 'run -p server'