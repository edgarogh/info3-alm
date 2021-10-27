CFLAGS=-Wall -g -O3
ARMTOOLCHAIN=arm-none-eabi

all: maze_gen

.PHONY: clean all

clean:
	-rm -f maze_gen maze_gen-*

maze_gen: maze_gen.c
	$(CC) $(CFLAGS) $^ -o $@

maze_gen-%: maze_gen.c %.xbm
	$(CC) $(CFLAGS) $< -o $@ -include $*.xbm