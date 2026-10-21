use crate::prelude::*;

pub type nlink_t = u32;
pub type blksize_t = c_long;
pub type __u64 = c_ulonglong;
pub type __s64 = c_longlong;
pub type regoff_t = c_int;

s! {
    pub struct pthread_attr_t {
        __size: [u32; 9],
    }

    pub struct sigset_t {
        __val: [c_ulong; 32],
    }

    pub struct sem_t {
        __val: [c_int; 4],
    }
}

pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 12;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 24;

cfg_if! {
    if #[cfg(any(target_arch = "x86"))] {
        mod x86;
        pub use self::x86::*;
    } else {
        // Unknown target_arch
    }
}
