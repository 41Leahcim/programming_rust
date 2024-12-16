unsafe: unsafe.cpp
	g++ unsafe.cpp -Os -Wall -Wextra -Wpedantic -Werror -o unsafe

run: unsafe
	./unsafe
