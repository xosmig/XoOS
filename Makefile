CC ?= gcc
LD ?= ld

TARGET := x86_64-unknown-none-gnu

#RUSTFLAGS := -C target-feature=-mmx,-3dnow,-3dnowa,-avx,-avx2 -C no-vectorize-slp -C no-vectorize-loops
RUSTFLAGS :=$(RUSTFLAGS) $(patsubst %,--cfg %,$(CFG))

CARGO_FLAGS := --target=$(TARGET) --no-default-features -v
LD_FLAGS := --nmagic -nostdlib -z max-page-size=0x1000 --gc-sections
QEMU_FLAGS := -d int -no-reboot -serial stdio -s 2> qemu.log

CARGO := RUSTFLAGS='$(RUSTFLAGS)' xargo

ASM_SRC := $(wildcard src/asm/*.S)
ASM_OBJ := $(ASM_SRC:.S=.o)

RUST_OBJ := target/$(TARGET)/debug/libxo_os.a
RUST_OBJ_RELEASE := target/$(TARGET)/release/libxo_os.a

OBJ := $(ASM_OBJ) $(RUST_OBJ)
OBJ_RELEASE := $(ASM_OBJ) $(RUST_OBJ_RELEASE)

RES_DIR := bin/debug
RES := $(RES_DIR)/kernel

RES_DIR_RELEASE := bin/release
RES_RELEASE := $(RES_DIR_RELEASE)/kernel

.PHONY: clean default build build_rust run gdb qemu release build_rust_release

# ======= useful targets: =======

default: build

release: $(RES_RELEASE)

build: $(RES)

tests:
	CFG='os_test' make run

run: $(RES) qemu

clean:
	$(CARGO) clean
	rm -rf bin src/asm/*.o src/asm/*.d *.tmp *.log

gdb:
	gdb bin/debug/kernel -ex 'set architecture i386:x86-64' -ex 'target remote localhost:1234'

qemu:
	qemu-system-x86_64 $(QEMU_FLAGS) -kernel bin/debug/kernel

# ======= compilation: =======

build_rust_release:
	$(CARGO) build $(CARGO_FLAGS) --release

build_rust:
	$(CARGO) build $(CARGO_FLAGS)

$(RES_RELEASE): build_rust_release $(OBJ_RELEASE) kernel.ld
	mkdir -p "$(RES_DIR_RELEASE)" 2> /dev/null
	$(LD) $(LD_FLAGS) -T kernel.ld -o $@ $(OBJ_RELEASE)

$(RES): build_rust $(OBJ) kernel.ld
	mkdir -p "$(RES_DIR)" 2> /dev/null
	$(LD) $(LD_FLAGS) -T kernel.ld -o $@ $(OBJ)

$(ASM_OBJ): %.o: %.S src/asm/interrupts.h
	$(CC) -D__ASM_FILE__ -I./src/asm -g -MMD -c $< -o $@
