use core::{fmt, mem::align_of};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(pub usize);

impl PhysAddr {
    pub fn new(addr: usize) -> Self {
        debug_assert_eq!(addr & 0xfff, 0, "Physical address is not page aligned");
        PhysAddr(addr)
    }

    /// # Safety
    ///
    /// This function is unsafe because the caller must ensure that the address is valid (page-aligned).
    pub const unsafe fn new_const(addr: usize) -> Self {
        PhysAddr(addr)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0 as u64
    }

    pub const fn align_up(&self, align: usize) -> Self {
        PhysAddr((self.0 + align - 1) & !(align - 1))
    }

    pub const fn align_down(&self, align: usize) -> Self {
        PhysAddr(self.0 & !(align - 1))
    }

    pub const fn is_aligned(&self, align: usize) -> bool {
        self.0 % align == 0
    }
}

impl fmt::Display for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PhysAddr(0x{:x})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(pub usize);

impl VirtAddr {
    pub const fn new(addr: usize) -> Self {
        VirtAddr(addr)
    }

    pub const fn as_usize(&self) -> usize {
        self.0
    }

    pub const fn as_u64(&self) -> u64 {
        self.0 as u64
    }

    pub const fn align_up(&self, align: usize) -> Self {
        VirtAddr((self.0 + align - 1) & !(align - 1))
    }

    pub const fn align_down(&self, align: usize) -> Self {
        VirtAddr(self.0 & !(align - 1))
    }

    pub const fn is_aligned(&self, align: usize) -> bool {
        self.0 % align == 0
    }

    pub const fn read_ok<T: Sized>(&self) -> bool {
        self.0 != 0 && self.0 % align_of::<T>() == 0
    }

    pub fn as_ptr<T>(&self) -> *const T {
        debug_assert!(self.read_ok::<T>(), "Invalid pointer alignment");
        self.0 as *const T
    }

    pub fn as_mut_ptr<T>(&self) -> *mut T {
        debug_assert!(self.read_ok::<T>(), "Invalid pointer alignment");
        self.0 as *mut T
    }

    pub const fn offset(&self, offset: usize) -> Self {
        VirtAddr(self.0 + offset)
    }

    /// # Safety
    ///
    /// This function is unsafe because the caller must ensure that the address is a valid pointer to a `T`.
    pub unsafe fn read<T: Sized>(&self) -> T {
        debug_assert!(self.read_ok::<T>(), "Invalid pointer alignment");
        unsafe { core::ptr::read(self.as_ptr()) }
    }

    /// # Safety
    ///
    /// This function is unsafe because the caller must ensure that the address is a valid pointer to a `T`.
    pub unsafe fn write<T: Sized>(&self, value: T) {
        debug_assert!(self.read_ok::<T>(), "Invalid pointer alignment");
        unsafe { core::ptr::write(self.as_mut_ptr(), value) }
    }
}

impl fmt::Display for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VirtAddr(0x{:x})", self.0)
    }
}
