use limine::request::*;
use spin::Once;
use x86_64::instructions::{self, interrupts};

use super::Arch;

pub mod core_local;
pub mod gdt;

static HHDM: HhdmRequest = HhdmRequest::new();
static STACK: StackSizeRequest =
    StackSizeRequest::new().with_size(X86_64::KERNEL_STACK_SIZE as u64);
static MEM_MAP: MemoryMapRequest = MemoryMapRequest::new();

static HHDM_PHYSICAL_OFFSET: Once<usize> = Once::new();

pub fn hhdm_physical_offset() -> usize {
    *HHDM_PHYSICAL_OFFSET.call_once(|| {
        HHDM.get_response()
            .expect("Failed to get HHDM physical offset")
            .offset() as usize
    })
}

pub struct X86_64;

impl Arch for X86_64 {
    const NAME: &'static str = "x86_64";
    const KERNEL_STACK_SIZE: usize = 0x8000;

    unsafe fn init() {
        // immediately ensure interrupts are disabled because we haven't set up an IDT yet
        interrupts::disable();

        // ensure kernel stack is setup properly by bootloader
        STACK
            .get_response()
            .expect("Failed to set kernel stack size");

        // initialize HHDM physical offset static constant
        let _hhdm_physical_offset = hhdm_physical_offset();

        // initialize boot GDT
        unsafe {
            gdt::init_boot_gdt();
        }

        log::info!("Initialized boot GDT");
    }

    fn interrupts_enabled() -> bool {
        interrupts::are_enabled()
    }

    fn disable_interrupts() {
        interrupts::disable();
    }

    fn enable_interrupts() {
        interrupts::enable();
    }

    fn hcf() -> ! {
        loop {
            interrupts::disable();
            instructions::hlt();
            core::hint::spin_loop();
        }
    }
}
