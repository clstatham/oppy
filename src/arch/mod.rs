cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub mod x86_64;
        pub use x86_64::X86_64 as TargetArch;
    } else {
        compile_error!("Unsupported target architecture");
    }
}

pub trait Arch {
    const NAME: &'static str;
    const KERNEL_STACK_SIZE: usize;

    /// Initialize the architecture-specific components of the kernel
    ///
    /// # Safety
    ///
    /// This function is unsafe because it may perform operations that require ring 0 privileges.
    unsafe fn init();

    /// Check if interrupts are enabled on the CPU
    fn interrupts_enabled() -> bool;

    /// Disable interrupts on the CPU
    fn disable_interrupts();

    /// Enable interrupts on the CPU
    fn enable_interrupts();

    /// Halt the CPU
    fn hcf() -> !;
}

/// Run a closure with interrupts disabled
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
