global_asm!(include_str!("exceptions.s"));

use super::irq;
use core::ptr;

#[repr(C)]
pub struct ExceptionCtx {
    regs: [u64; 30],
    elr_el1: u64,
    spsr_el1: u64,
    lr: u64,
}

#[no_mangle]
fn on_exception(_ctx: &mut ExceptionCtx, i: u8) {

    const UART0: *mut u8 = 0xffff_ffe0_0900_0000 as *mut u8;
    unsafe {
        ptr::write_volatile(UART0, i);
    }
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_sync(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x31);
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_irq(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x32);
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_fiq(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x33);
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_error(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x34);
}


#[no_mangle]
unsafe extern "C" fn el1_sync(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x35);
}

#[no_mangle]
unsafe extern "C" fn el1_irq(ctx: &mut ExceptionCtx) {
    irq::handler(ctx);
}

#[no_mangle]
unsafe extern "C" fn el1_fiq(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x37);
}

#[no_mangle]
unsafe extern "C" fn el1_error(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x38);
}


#[no_mangle]
unsafe extern "C" fn el0_sync(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x39);
}

#[no_mangle]
unsafe extern "C" fn el0_irq(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x41);
}

#[no_mangle]
unsafe extern "C" fn el0_fiq(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x42);
}

#[no_mangle]
unsafe extern "C" fn el0_error(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x43);
}


#[no_mangle]
unsafe extern "C" fn el0_32_sync(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x44);
}

#[no_mangle]
unsafe extern "C" fn el0_32_irq(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x45);
}

#[no_mangle]
unsafe extern "C" fn el0_32_fiq(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x46);
}

#[no_mangle]
unsafe extern "C" fn el0_32_error(ctx: &mut ExceptionCtx) {
    on_exception(ctx, 0x47);
}
