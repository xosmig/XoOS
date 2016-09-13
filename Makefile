
CC ?= gcc
LD ?= ld

TARGET := x86_64-unknown-linux-gnu

CARGO_FLAGS := --target=$(TARGET)
LD_FLAGS := --nmagic --gc-sections -nostdlib -z max-page-size=0x1000

ASM_SRC := $(wildcard src/asm/*.S)
ASM_OBJ := $(ASM_SRC:.S=.o)

RUST_OBJ := target/$(TARGET)/debug/libxo_os.a

OBJ := $(ASM_OBJ) $(RUST_OBJ)

RES_DIR := bin/debug
RES := $(RES_DIR)/kernel

.PHONY: clean default build rust_build run gdb qemu

# ======= useful targets: =======

default: build

build: $(RES)

run: $(RES)
	echo "FIXME: Not implemented yet"

clean:
	cargo clean
	rm -rf bin src/asm/*.o src/asm/*.d

gdb:
	gdb bin/debug/kernel -ex 'set architecture i386:x86-64' -ex 'target remote localhost:1234'

qemu:
	qemu-system-x86_64 -kernel bin/debug/kernel -serial stdio -s &

# ======= compilation: =======

rust_build:
	cargo build $(CARGO_FLAGS)

$(RES): rust_build $(OBJ) kernel.ld
	mkdir -p "$(RES_DIR)" 2> /dev/null
	$(LD) $(LD_FLAGS) -T kernel.ld -o $@ $(OBJ)

$(ASM_OBJ): %.o: %.S
	$(CC) -D__ASM_FILE__ -I./src/asm -g -MMD -c $< -o $@
