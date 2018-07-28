.PHONY: release debug clean run 

py_dir = hello
lib_name = myrust
module_name = rustlib

release:
	RUSTFLAGS="-C target-cpu=native" cargo build --release
	@cp target/release/lib$(lib_name).so $(py_dir)/$(module_name).so

debug:
	cargo build
	@cp target/debug/lib$(lib_name).so $(py_dir)/$(module_name).so

clean:
	rm $(py_dir)/*.so

run: release 
	python3 $(py_dir)/main.py

