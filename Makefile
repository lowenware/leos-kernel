BOARD ?= qemu

ARCH ?= aarch64-unknown-none-softfloat
CPU ?= cortex-a57
LINKER ?= src/aarch64/aarch64.ld

ifeq ($(BOARD),qemu)
	SOC = virt
else ifeq ($(BOARD), pinebookpro)
	SOC = rk3399
else ifeq ($(BOARD), raspi3)
	SOC = bcm2711
else ifeq ($(BOARD), raspi4)
	SOC = bcm2711
	CPU = cortex-a72
else
    $(error unsupported board $(BOARD))
endif

RUSTFLAGS = \
  -C link-arg=-T$(LINKER) \
  -C target-cpu=$(CPU) \
  --cfg board=\"$(BOARD)\" \
  --cfg soc=\"$(SOC)\" \

OBJCOPY = aarch64-none-elf-objcopy
OBJDUMP = aarch64-none-elf-objdump
GDB = aarch64-none-elf-gdb
QEMU_CMD = qemu-system-aarch64 \
	-machine virt,virtualization=on \
	-m 1024M \
	-serial stdio \
	-display none \
	-cpu cortex-a72 \

#	-netdev user,id=n1 -device virtio-net-device,netdev=n1 \
#	-device virtio-gpu-device \
#	-vnc :0 \

KERNEL_BUILD = target/$(ARCH)/release/leos-kernel
KERNEL_IMAGE = $(KERNEL_BUILD).bin

debug ?= 0

all: $(KERNEL_IMAGE)

.PHONY: $(KERNEL_BUILD)
$(KERNEL_BUILD):
	RUSTFLAGS="$(RUSTFLAGS)" cargo rustc --target $(ARCH) --release

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

.PHONY: lint
lint:
	RUSTFLAGS="$(RUSTFLAGS)" cargo clippy

.PHONY: test
test:
	RUSTFLAGS="$(RUSTFLAGS)" cargo test --target $(ARCH) --release

.PHONY: dtc
dtc:
	$(QEMU_CMD) -machine dumpdtb=qemu.dtb
	dtc -I dtb -O dts qemu.dtb

.PHONY: disasm
disasm:
	$(OBJDUMP) --disassemble-all $(KERNEL_BUILD) | less

