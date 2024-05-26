pub mod x86_64;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub use x86_64::X86_64 as TargetArch;
    } else {
        compile_error!("Unsupported target architecture");
    }
}

pub trait Arch {
    const NAME: &'static str;
    const KERNEL_STACK_SIZE: usize;

    fn init();
    fn interrupts_enabled() -> bool;
    fn disable_interrupts();
    fn enable_interrupts();
    fn hcf() -> !;
}

#[inline]
pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let interrupts_enabled = TargetArch::interrupts_enabled();
    if interrupts_enabled {
        TargetArch::disable_interrupts();
    }

    let result = f();

    if interrupts_enabled {
        TargetArch::enable_interrupts();
    }

    result
}
