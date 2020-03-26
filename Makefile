TARGET = aarch64-lowenware-leos
OBJCOPY = aarch64-none-elf-objcopy
GDB = aarch64-none-elf-gdb
QEMU_CMD = qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic

KERNEL_BUILD = target/$(TARGET)/release/leos-kernel
KERNEL_IMAGE = $(KERNEL_BUILD).bin

debug ?= 0

all: $(KERNEL_IMAGE)

.PHONY: $(KERNEL_BUILD)
$(KERNEL_BUILD):
	cargo xbuild --target aarch64/$(TARGET).json --release

$(KERNEL_IMAGE): $(KERNEL_BUILD)
	$(OBJCOPY) -O binary $(KERNEL_BUILD) $(KERNEL_IMAGE)

.PHONY: run
run: $(KERNEL_IMAGE)
ifeq ($(debug), 1)
	$(QEMU_CMD) -kernel $(KERNEL_IMAGE) -s -S
else
	$(QEMU_CMD) -kernel $(KERNEL_IMAGE)
endif
	@echo -e ""

.PHONY: debug
debug:
	$(GDB) -x release.gdb

.PHONY: clean
clean:
	cargo clean

