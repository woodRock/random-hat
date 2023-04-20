all: build_release copy_404

build_release: 
	@echo "Building release 🔧"
	@trunk build 

copy_404:
	@echo "Copying 404.html 📋"
	@cp 404.html docs/

format: 
	@echo "Format 🧹"
	@cargo fmt --all -- --check

lint: 
	@echo "Linting 🧹"
	@cargo clippy --all -- -D warnings

pedantic:
	@echo "Linting (pedantic) 🧹"
	@cargo clippy --all -- -D warnings -D clippy::pedantic

