target:
	- cargo install cargo-pgo

prod:
	- RUSTFLAGS="-C target-cpu=native" cargo pgo build -- --release
	
