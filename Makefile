unsafe: unsafe.c
	gcc unsafe.c -Os -Wall -Wextra -Wpedantic -Werror -o unsafe

run: unsafe
	./unsafe
