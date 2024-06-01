use x86::msr::{rdmsr, IA32_GS_BASE};
use x86_64::structures::{gdt::GlobalDescriptorTable, tss::TaskStateSegment};

pub struct CoreLocalData {
    pub kernel_sp: usize,
    pub gdt: GlobalDescriptorTable,
}

#[repr(C, packed)]
pub struct CoreLocalRegion {
    pub tss: TaskStateSegment,
    pub core_local_data: &'static mut CoreLocalData,
}

impl CoreLocalRegion {
    pub fn get() -> &'static mut CoreLocalRegion {
        unsafe { &mut *(rdmsr(IA32_GS_BASE) as *mut _) }
    }

    pub fn get_tss() -> &'static mut TaskStateSegment {
        unsafe { &mut *(rdmsr(IA32_GS_BASE) as *mut _) }
    }
}
