.extern LD_STACK_PTR
.extern LD_TTBR0_BASE
.extern LD_TTBR1_BASE
.extern exception_vector_table

.section ".text.start"

.globl _start
_start:
// preserve current address
    adr     x20, .
// first 8 bits of mpidr indicate core id
    mrs     x1, MPIDR_EL1
    and     x1, x1, 0x0f
    cbz     x1, 2f

// loop non-zero CPU
1:  wfe
    b       1b

// continue with zero CPU
2:  mrs     x0, CurrentEL
    and     x0, x0, #0x0C           // 0x0C = b1100 - mask for EL
    cmp     x0, #0x0C               // EL3
    bne     3f

// quit EL3 to EL1
    ldr     x2, =SCR_EL3_VALUE
    msr     SCR_EL3, x2

    ldr     x2, =SPSR_EL3_VALUE
    msr     SPSR_EL3, x2
    adr     x2, 3f

// exception link register @ EL3 -> elr_el3
    msr     elr_el3, x2
    eret

// consider EL1
3:  mrs     x0, sctlr_el1
    bic     x0, x0, 0x04            // clear cache bit C:2
    msr     sctlr_el1, x0

// Prepare values in registers
    ldr     x19, =_start
    sub     x19, x19, x20           // delta to calculate real address

    ldr     x21, =LD_TTBR0_BASE
    sub     x21, x21, x19           // LD_TTBR0_BASE real address

    ldr     x22, =LD_TTBR1_BASE
    sub     x22, x22, x19           // LD_TTBR1_BASE real address

    ldr     x23, =LD_TTL2_BASE
    sub     x23, x23, x19           // LD_TTL2_BASE real address

    ldr     x24, =LD_TTL3_BASE
    sub     x24, x24, x19           // LD_TTL3_BASE real address

// Erase BSS, stack and translation table memory
    ldr     x0, =LD_BSS_BASE
    ldr     x1, =LD_KERNEL_END
    sub     x1, x1, x0
    sub     x0, x0, x19
    bl      memzero

// Prepare initial 1GB identity mapping
    lsr     x0, x20, #30            // divide kernel start address by 1G
    lsl     x1, x0, #30             // multiply by 1G, and keep table index in x0
    ldr     x10, =IDENTITY_MAP_ATTR
    orr     x1, x1, x10             // add flags
    str     x1, [x21, x0, lsl #3]   // x0 -> table index

// Set higher half translation table register
    orr     x0, x21, #1             // set CNP flag
    msr     ttbr0_el1, x0

// Map peripherals memory to kernel space
// TODO: move it later to SoC driver, peripherals should not be used until
// SoC is determined and initiated, event UART output
    ldr     x0, =PERIPHERALS_BASE
    ldr     x1, =PERIPHERALS_ATTR
    orr     x0, x0, x1
    str     x0, [x22]

// Map kernel: L1
    ldr     x0, =_start
    lsr     x0, x0, #30
    and     x0, x0, #0x7f
    ldr     x10, =KERNEL_RODATA_ATTR
    orr     x1, x23, x10            // x23 = real LD_TTL2_BASE
    str     x1, [x22, x0, lsl #3]   // x22 = real LD_TTBR1_BASE

// Set lower half translation table register
    orr     x0, x22, #1
    msr     ttbr1_el1, x0

// Map kernel: L2
    orr     x0, x24, x10            // x24 = real LD_TTL3_BASE
    str     x0, [x23]               // x23 = real LD_TTL2_BASE

// Map kernel: L3
    mov     x0, x20                 // real kernel start address
    ldr     x1, =LD_RODATA_BASE
    sub     x1, x1, x19             // mapping end address
    mov     x3, x24                 // x24 = real LD_TTL3_BASE
    ldr     x10, =KERNEL_CODE_ATTR  // descriptor attributes
    ldr     x2, =6f                 // cycle exit point
    sub     x2, x2, x19
4:  cmp     x0, x1                  // Section could be empty, so check first
    blt     5f
    br      x2
5:  orr     x4, x0, x10
    str     x4, [x3], #8
    add     x0, x0, #0x1000
    b       4b

// Continue with RODATA
6:  ldr     x10, =KERNEL_RODATA_ATTR
    ldr     x1, =LD_DATA_BASE
    sub     x1, x1, x19
    ldr     x2, =7f
    sub     x2, x2, x19
    b       4b

// Continue with data, bss, stack, translation tables
7:  ldr     x10, =KERNEL_DATA_ATTR
    ldr     x1, =LD_KERNEL_END
    sub     x1, x1, x19
    ldr     x2, =8f
    sub     x2, x2, x19
    b       4b

// Initialize MMU
8:  ldr     x0, =TCR_EL1_VALUE
    msr     tcr_el1, x0
    ldr     x0, =MAIR_EL1_VALUE
    msr     mair_el1, x0

    dsb     ish                      // make changes visible
    isb
    mrs     x0, sctlr_el1
    orr     x0, x0, #0x01            //  The M (MMU Enable) bit
    msr     sctlr_el1, x0

// Initialize exceptions
    ldr     x0, =exception_vector_table
    msr     vbar_el1, x0
    isb

// enable floating point instructions
    mrs    x1, cpacr_el1
    mov    x0, #(3 << 20)
    orr    x0, x1, x0
    msr    cpacr_el1, x0

// Start Kernel
    ldr     x2, =LD_STACK_PTR
    mov     sp, x2
    mov     x0, x20
    adr     x1, LD_KERNEL_END
    sub     x1, x1, x0

    ldr     x11, =kernel_main
    blr     x11

.globl system_off
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0

.globl memzero
memzero:
    str xzr, [x0], 8
    subs x1, x1, 8
    b.gt memzero
    ret

.equ PERIPHERALS_ATTR, 0x60000000000605 // -------------------------------------

// UXN   | b1      << 54 | Unprivileged eXecute Never
// PXN   | b1      << 53 | Privileged eXecute Never
// AF    | b1      << 10 | Access Flag
// SH    | b10     << 8  | Outer shareable
// AP    | b00     << 6  | R/W, EL0 access denied
// NS    | b0      << 5  | Security bit (EL3 and Secure EL1 only)
// INDX  | b001    << 2  | Attribute index in MAIR_ELn
// ENTRY | b01     << 0  | Block entry

.equ IDENTITY_MAP_ATTR, 0x40000000000701 // ------------------------------------

// UXN   | b1      << 54 | Unprivileged eXecute Never
// PXN   | b0      << 53 | Privileged eXecute Never
// AF    | b1      << 10 | Access Flag
// SH    | b11     << 8  | Inner shareable
// AP    | b00     << 6  | R/W, EL0 access denied
// NS    | b0      << 5  | Security bit (EL3 and Secure EL1 only)
// INDX  | b000    << 2  | Attribute index in MAIR_ELn
// ENTRY | b01     << 0  | Block entry

.equ KERNEL_CODE_ATTR, 0x40000000000783 // -------------------------------------

// UXN   | b1      << 54 | Unprivileged eXecute Never
// PXN   | b0      << 53 | Privileged eXecute Never
// AF    | b1      << 10 | Access Flag
// SH    | b11     << 8  | Inner shareable
// AP    | b10     << 6  | Read-only, EL1 access
// NS    | b0      << 5  | Security bit (EL3 and Secure EL1 only)
// INDX  | b000    << 2  | Attribute index in MAIR_ELn
// ENTRY | b11     << 0  | Table descriptor entry

.equ KERNEL_RODATA_ATTR, 0x60000000000783 // -----------------------------------

// UXN   | b1      << 54 | Unprivileged eXecute Never
// PXN   | b1      << 53 | Privileged eXecute Never
// AF    | b1      << 10 | Access Flag
// SH    | b11     << 8  | Inner shareable
// AP    | b10     << 6  | Read-only, EL1 access
// NS    | b0      << 5  | Security bit (EL3 and Secure EL1 only)
// INDX  | b000    << 2  | Attribute index in MAIR_ELn
// ENTRY | b11     << 0  | Table descriptor entry

.equ KERNEL_DATA_ATTR, 0x60000000000703 // -------------------------------------

// UXN   | b1      << 54 | Unprivileged eXecute Never
// PXN   | b1      << 53 | Privileged eXecute Never
// AF    | b1      << 10 | Access Flag
// SH    | b11     << 8  | Inner shareable
// AP    | b00     << 6  | R/W, EL1 access
// NS    | b0      << 5  | Security bit (EL3 and Secure EL1 only)
// INDX  | b000    << 2  | Attribute index in MAIR_ELn
// ENTRY | b11     << 0  | Table descriptor entry

.equ MAIR_EL1_VALUE, 0x000004FF // ---------------------------------------------

// IDX 0 | b11111111 << 0 | Normal memory
// IDX 1 | b00000100 << 8 | Device-nGnRE memory (non-cacheble)

.equ TCR_EL1_VALUE, 0x1B51B351B // ---------------------------------------------

// IPS   | b001    << 32 | 36bits address space - 64GB
// TG1   | b10     << 30 | 4KB granule size for TTBR1_EL1
// SH1   | b11     << 28 | Inner shareable
// ORGN1 | b01     << 26 | Normal, Outer Wr.Back Rd.alloc Wr.alloc Cacheble
// IRGN1 | b01     << 24 | Normal, Inner Wr.Back Rd.alloc Wr.alloc Cacheble
// EPD   | b0      << 23 | Perform translation table walk using TTBR1_EL1
// A1    | b0      << 22 | TTBR1_EL1.ASID defined the ASID
// T1SZ  | b011011 << 16 | Memory region 2^(64-28) -> 0xffffffexxxxxxxxx
// TG0   | b00     << 14 | 4KB granule size
// SH0   | b11     << 12 | Inner Sharebale
// ORGN0 | b01     << 10 | Normal, Outer Wr.Back Rd.alloc Wr.alloc Cacheble
// IRGN0 | b01     << 8  | Normal, Inner Wr.Back Rd.alloc Wr.alloc Cacheble
// EPD0  | b0      << 7  | Perform translation table walk using TTBR0_EL1
// 0     | b0      << 6  | Zero field (reserve)
// T0SZ  | b011011 << 0  | Memory region 2^(64-28)

// -----------------------------------------------------------------------------

.equ SCR_EL3_VALUE, 0x05B1

.equ SPSR_EL3_VALUE, 0x03C9

.equ PSCI_SYSTEM_OFF, 0x84000008

.equ PERIPHERALS_BASE, 0x0000000000000000
