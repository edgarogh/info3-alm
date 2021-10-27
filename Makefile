# Choose your Qemu, set to your own path
QEMU=qemu-system-arm

# Choose your toolchain for ARM 
TOOLCHAIN=arm-none-eabi


# Looking for GDB...
GDB:=$(TOOLCHAIN)-gdb

ifeq (, $(shell which $(GDB)))
GDB:=gdb-multiarch
endif

ifeq (, $(shell which $(GDB)))
$(error "neither arm-none-eabi-gdb nor gdb-multiarch have been found")
endif


# Choose your keyboard: azerty(fr) or qwerty(en-us)
# KEYBOARD=-k fr
KEYBOARD=-k en-us

####################################################################
# OPTIONS THAT YOU CAN CHANGE ARE ALL ABOVE THIS LINE.
# DO NOT CHANGE ANYTHING BELOW THIS LINE
# UNLESS YOU KNOW WHAT YOU ARE DOING
####################################################################

all: kernel.bin binary_tree.s binary_tree_no-O1.s

clean: 
	rm -f *.o vendored/*.o kernel.elf kernel.bin

kernel.bin: start.o maze_gen.o io.o print_string.o vendored/uidiv.o Makefile ldscript
	$(TOOLCHAIN)-ld -T ldscript start.o maze_gen.o io.o print_string.o vendored/uidiv.o -o kernel.elf
	$(TOOLCHAIN)-objcopy -O binary kernel.elf kernel.bin

start.o: start.s Makefile
	$(TOOLCHAIN)-as -mcpu=arm926ej-s -gstabs+ $< -o $@

io.o: io.s
	$(TOOLCHAIN)-as -mcpu=arm926ej-s -gstabs+ $< -o $@

print_string.o: print_string.s
	$(TOOLCHAIN)-as -mcpu=arm926ej-s -gstabs+ $< -o $@

maze_gen.o: maze_gen.c
	$(TOOLCHAIN)-gcc -Wall -mcpu=arm926ej-s -gstabs+ -c $^ -o $@

vendored/%.o: vendored/%.s
	$(TOOLCHAIN)-as -mcpu=arm926ej-s -gstabs+ $< -o $@

qemu: kernel.bin
	$(QEMU) -M versatilepb -nographic -m 64M -gdb tcp::1234 -S $(KEYBOARD) -kernel kernel.bin 

gdb: kernel.bin
	$(GDB) kernel.elf
