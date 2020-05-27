
.section ".text.start"

// x0 - current task control block address
// x1 - next task control block address
.globl aarch64_switch_context
aarch64_switch_context:
    mov     x10, x0
    mov     x11, sp
    stp     x19, x20, [x10], #16
    stp     x21, x22, [x10], #16
    stp     x23, x24, [x10], #16
    stp     x25, x26, [x10], #16
    stp     x27, x28, [x10], #16
    stp     x29, x30, [x10], #16
    str     x11, [x10]
    mov     x10, x1
    ldp     x19, x20, [x10], #16
    ldp     x21, x22, [x10], #16
    ldp     x23, x24, [x10], #16
    ldp     x25, x26, [x10], #16
    ldp     x27, x28, [x10], #16
    ldp     x29, x30, [x10], #16
    ldr     x11, [x10]
    mov     sp, x11
    ret

.globl aarch64_start_task
aarch64_start_task:
    mov     x0, x19
    mov     x1, x20
    b       start_task // scheduler::start_task
