all: compile
	./main.out

compile: main.o
	g++ main.o -o main.out -O3 -std=c++14
	rm main.o

main.o: main.cpp
	g++ main.cpp -c -o main.o -O3 -std=c++14