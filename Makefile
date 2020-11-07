
build:
	rustc -o rmd main.rs
	
install:
	cp rmd /usr/local/bin/
	
clean:
	rm rmd