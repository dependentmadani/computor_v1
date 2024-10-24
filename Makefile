all:
	@mkdir -p build && rustc -o build/computor_v1 src/main.rs
	@echo "Build done!"

clean:
	@rm -rf build
	@echo "Clean done!"

re: clean all
