use x86_64::{
    registers::segmentation::{Segment, CS, DS, ES, FS, GS, SS},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

use crate::arch::Arch;

use super::X86_64;

pub const KERNEL_CS_IDX: u16 = 1;
pub const KERNEL_DS_IDX: u16 = 2;
pub const TSS_IDX: u16 = 3;
pub const USER_DS_IDX: u16 = 5;
pub const USER_CS_IDX: u16 = 6;

static mut EXCEPTION_STACK: [u8; X86_64::KERNEL_STACK_SIZE] = [0; X86_64::KERNEL_STACK_SIZE];

lazy_static::lazy_static! {
    static ref BOOT_GDT: BootGdtWithSelectors = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code_selector = gdt.append(Descriptor::kernel_code_segment());
        let kernel_data_selector = gdt.append(Descriptor::kernel_data_segment());
        BootGdtWithSelectors {
            gdt,
            selectors: [
                kernel_code_selector,
                kernel_data_selector,
            ]
        }
    };
}

/// Boot GDT and its associated segment selectors
struct BootGdtWithSelectors {
    gdt: GlobalDescriptorTable,
    selectors: [SegmentSelector; 2],
}

/// Initialize the boot GDT
///
/// # Safety
///
/// This function is unsafe because it changes the segment registers which requires ring 0 privileges.
pub unsafe fn init_boot_gdt() {
    // load boot GDT
    BOOT_GDT.gdt.load();

    // set segment registers to entries in the boot GDT
    unsafe {
        CS::set_reg(BOOT_GDT.selectors[0]); // code segment register
        DS::set_reg(BOOT_GDT.selectors[1]); // data segment register

        // set ES, FS, GS, SS to the data segment as well
        ES::set_reg(BOOT_GDT.selectors[1]);
        FS::set_reg(BOOT_GDT.selectors[1]);
        GS::set_reg(BOOT_GDT.selectors[1]);
        SS::set_reg(BOOT_GDT.selectors[1]);
    }
}
