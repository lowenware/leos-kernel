global_asm!(include_str!("exceptions.s"));

use crate::irq;

const EL1_SP0_SYNC : &str = "EL1_SP0_SYNC";
const EL1_SP0_IRQ : &str = "EL1_SP0_IRQ";
const EL1_SP0_FIQ : &str = "EL1_SP0_FIQ";
const EL1_SP0_ERROR : &str = "EL1_SP0_ERROR";
const EL1_SYNC : &str = "EL1_SYNC";
// const EL1_IRQ : &str = "EL1_IRQ";
const EL1_FIQ : &str = "EL1_FIQ";
const EL1_ERROR : &str = "EL1_ERROR";
const EL0_SYNC : &str = "EL0_SYNC";
const EL0_IRQ : &str = "EL0_IRQ";
const EL0_FIQ : &str = "EL0_FIQ";
const EL0_ERROR : &str = "EL0_ERROR";
const EL0_32_SYNC : &str = "EL1_32_SYNC";
const EL0_32_IRQ : &str = "EL1_32_IRQ";
const EL0_32_FIQ : &str = "EL1_32_FIQ";
const EL0_32_ERROR : &str = "EL1_32_ERROR";

#[repr(C)]
pub struct ExceptionContext {
    regs: [u64; 30],
    elr_el1: u64,
    spsr_el1: u64,
    lr: u64,
}

fn catch(ctx: &mut ExceptionContext, name: &str) {
    panic!(
        "\n  \
        {} @ 0x{:016x}\n\n  \
        x0 = 0x{:016x}  x10 = 0x{:016x}  x20 = 0x{:016x}\n  \
        x1 = 0x{:016x}  x11 = 0x{:016x}  x21 = 0x{:016x}\n  \
        x2 = 0x{:016x}  x12 = 0x{:016x}  x22 = 0x{:016x}\n  \
        x3 = 0x{:016x}  x13 = 0x{:016x}  x23 = 0x{:016x}\n  \
        x4 = 0x{:016x}  x14 = 0x{:016x}  x24 = 0x{:016x}\n  \
        x5 = 0x{:016x}  x15 = 0x{:016x}  x25 = 0x{:016x}\n  \
        x6 = 0x{:016x}  x16 = 0x{:016x}  x26 = 0x{:016x}\n  \
        x7 = 0x{:016x}  x17 = 0x{:016x}  x27 = 0x{:016x}\n  \
        x8 = 0x{:016x}  x18 = 0x{:016x}  x28 = 0x{:016x}\n  \
        x9 = 0x{:016x}  x19 = 0x{:016x}  x29 = 0x{:016x}\n\n  \
        lr = 0x{:016x}\n  \
        sp = 0x{:016x}\n\n  \
        SPSR_EL1 = 0x{:016x}\n  \
        \n",
        name, ctx.elr_el1,
        ctx.regs[0], ctx.regs[10], ctx.regs[20],
        ctx.regs[1], ctx.regs[11], ctx.regs[21],
        ctx.regs[2], ctx.regs[12], ctx.regs[22],
        ctx.regs[3], ctx.regs[13], ctx.regs[23],
        ctx.regs[4], ctx.regs[14], ctx.regs[24],
        ctx.regs[5], ctx.regs[15], ctx.regs[25],
        ctx.regs[6], ctx.regs[16], ctx.regs[26],
        ctx.regs[7], ctx.regs[17], ctx.regs[27],
        ctx.regs[8], ctx.regs[18], ctx.regs[28],
        ctx.regs[9], ctx.regs[19], ctx.regs[29],
        ctx.lr,
        ctx as *const ExceptionContext as u64,
        ctx.spsr_el1,
    );
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_sync(ctx: &mut ExceptionContext) {
    catch(ctx, EL1_SP0_SYNC);
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_irq(ctx: &mut ExceptionContext) {
    catch(ctx, EL1_SP0_IRQ);
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_fiq(ctx: &mut ExceptionContext) {
    catch(ctx, EL1_SP0_FIQ);
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_error(ctx: &mut ExceptionContext) {
    catch(ctx, EL1_SP0_ERROR);
}

#[no_mangle]
unsafe extern "C" fn el1_sync(ctx: &mut ExceptionContext) {
    catch(ctx, EL1_SYNC);
}

#[no_mangle]
unsafe extern "C" fn el1_irq(_ctx: &mut ExceptionContext) {
    irq::handler();
}

#[no_mangle]
unsafe extern "C" fn el1_fiq(ctx: &mut ExceptionContext) {
    catch(ctx, EL1_FIQ);
}

#[no_mangle]
unsafe extern "C" fn el1_error(ctx: &mut ExceptionContext) {
    catch(ctx, EL1_ERROR);
}

#[no_mangle]
unsafe extern "C" fn el0_sync(ctx: &mut ExceptionContext) {
    catch(ctx, EL0_SYNC);
}

#[no_mangle]
unsafe extern "C" fn el0_irq(ctx: &mut ExceptionContext) {
    catch(ctx, EL0_IRQ);
}

#[no_mangle]
unsafe extern "C" fn el0_fiq(ctx: &mut ExceptionContext) {
    catch(ctx, EL0_FIQ);
}

#[no_mangle]
unsafe extern "C" fn el0_error(ctx: &mut ExceptionContext) {
    catch(ctx, EL0_ERROR);
}

#[no_mangle]
unsafe extern "C" fn el0_32_sync(ctx: &mut ExceptionContext) {
    catch(ctx, EL0_32_SYNC);
}

#[no_mangle]
unsafe extern "C" fn el0_32_irq(ctx: &mut ExceptionContext) {
    catch(ctx, EL0_32_IRQ);
}

#[no_mangle]
unsafe extern "C" fn el0_32_fiq(ctx: &mut ExceptionContext) {
    catch(ctx, EL0_32_FIQ);
}

#[no_mangle]
unsafe extern "C" fn el0_32_error(ctx: &mut ExceptionContext) {
    catch(ctx, EL0_32_ERROR);
}

// TODO: daifclr abstraction
pub fn enable_all() {
    unsafe {
        llvm_asm!("msr daifclr, #2");
    }
}

// TODO: daifset abstraction
pub fn disable_all() {
    unsafe {
        llvm_asm!("msr daifset, #2");
    }
}
