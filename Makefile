dev:
	cargo build
	target/debug/udp-pretend-tcp --remote-host test

release:
	cross build --target x86_64-unknown-linux-gnu --release


