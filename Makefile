build:
	cargo build
clean:
	rm -rf target
	rm -rf Cargo.lock
backup:
	rm -rf backup
	mkdir backup
	cp -r src backup/
