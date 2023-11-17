test: test.o
	ld -macosx_version_min 11.0.0 -o test test.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64 

test.o: test.s
	as -o test.o test.s

stack: stack.o
	ld -macosx_version_min 11.0.0 -o stack stack.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64 

stack.o: stack.s
	as -o stack.o stack.s

printf: printf.o
	ld -macosx_version_min 11.0.0 -o printf printf.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64 

printf.o: printf.s
	as -o printf.o printf.s

all: test stack printf

clean:
	rm test
	rm stack
	rm printf
	rm *.o

re: clean all