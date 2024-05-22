use core::mem::ManuallyDrop;

use spin::mutex::{SpinMutex, SpinMutexGuard};
use x86::bits64::rflags::{self, RFlags};
use x86_64::instructions::interrupts;

/// A guard that saves the interrupt flag status when it is created and restores it when it is dropped.
pub struct SavedInterruptStatus {
    rflags: RFlags,
}

impl SavedInterruptStatus {
    /// Saves the current interrupt flag status.
    /// The returned guard will restore the interrupt flag to the saved status when it is dropped.
    pub fn save() -> SavedInterruptStatus {
        SavedInterruptStatus {
            rflags: rflags::read(),
        }
    }
}

impl Drop for SavedInterruptStatus {
    fn drop(&mut self) {
        rflags::set(rflags::read() | (self.rflags & rflags::RFlags::FLAGS_IF));
    }
}

/// An interrupt-safe mutex.
/// It is similar to [`SpinMutex`], but it disables interrupts when it locks the mutex, and restores the interrupt flag when the guard is dropped.
pub struct IrqMutex<T: ?Sized> {
    inner: SpinMutex<T>,
}

impl<T> IrqMutex<T> {
    /// Creates a new mutex.
    pub const fn new(data: T) -> Self {
        Self {
            inner: SpinMutex::new(data),
        }
    }
}

impl<T: ?Sized> IrqMutex<T> {
    /// Returns a reference to the data.
    /// This cannot deadlock because it does not lock the mutex, instead requiring a mutable reference to the mutex.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Attempts to lock the mutex.
    /// Returns `None` if the mutex is already locked.
    pub fn try_lock(&self) -> Option<IrqMutexGuard<'_, T>> {
        if self.inner.is_locked() {
            return None;
        }

        Some(self.lock())
    }

    /// Locks the mutex.
    /// This will disable interrupts until the guard is dropped.
    /// Panics if it would deadlock.
    pub fn lock(&self) -> IrqMutexGuard<'_, T> {
        if self.inner.is_locked() {
            panic!("deadlock");
        }

        let saved_status = SavedInterruptStatus::save();
        interrupts::disable();

        let guard = self.inner.lock();

        IrqMutexGuard {
            inner: ManuallyDrop::new(guard),
            saved_status: ManuallyDrop::new(saved_status),
        }
    }

    /// Returns whether the mutex is locked.
    pub fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }

    /// Forces the mutex to unlock.
    ///
    /// # Safety
    ///
    /// See [`SpinMutex::force_unlock`].
    pub unsafe fn force_unlock(&self) {
        self.inner.force_unlock();
    }
}

unsafe impl<T: ?Sized + Send> Send for IrqMutex<T> {}
unsafe impl<T: ?Sized + Sync> Sync for IrqMutex<T> {}

/// A guard that unlocks an [`IrqMutex`] and restores the interrupt flag when it is dropped.
pub struct IrqMutexGuard<'a, T: ?Sized> {
    inner: ManuallyDrop<SpinMutexGuard<'a, T>>,
    saved_status: ManuallyDrop<SavedInterruptStatus>,
}

impl<'a, T: ?Sized> Drop for IrqMutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner);
        }

        unsafe {
            ManuallyDrop::drop(&mut self.saved_status);
        }
    }
}

impl<'a, T: ?Sized> core::ops::Deref for IrqMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T: ?Sized> core::ops::DerefMut for IrqMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
