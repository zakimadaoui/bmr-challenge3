 
BUILD_DIR=./target/riscv32imac-unknown-none-elf
TARGET=bmr-challenge3
riscv_objcopy=/home/zaki/Documents/riscv-binutils-gdb/binutils/objcopy


release:
	BUILD_OPT=release;\
	cargo build --release
	$(riscv_objcopy) -O binary $(BUILD_DIR)/release/$(TARGET) firmware.bin

debug:
	BUILD_OPT=debug;\
	cargo build
	$(riscv_objcopy) -O binary $(BUILD_DIR)/debug/$(TARGET) firmware.bin


flash:
	dfu-util -a 0 -s 0x08000000:leave -D firmware.bin

flashd: debug
	dfu-util -a 0 -s 0x08000000:leave -D firmware.bin

flashr: release
	dfu-util -a 0 -s 0x08000000:leave -D firmware.bin
clean:
	rm -r target
	rm firmware.bin





