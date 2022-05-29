all: main

main.o: main.c

target/debug/libeh_personality.a: src/lib.rs
	cargo build --release

main: main.o target/debug/libeh_personality.a
	gcc -o main main.o  -Ltarget/release -leh_personality

clean:
	cargo clean
	rm main.o main
