use crate::prelude::*;
use crate::unix::redox;

pub type regoff_t = c_long;

s! {
    pub struct stack_t {
        pub ss_sp: *mut c_void,
        pub ss_flags: c_int,
        pub ss_size: size_t,
    }

    pub struct pthread_attr_t {
        __size: [u8; _PTHREAD_ATTR_SIZE],
    }

    pub struct sigset_t {
        __val: [c_ulong; 16],
    }

    pub struct shmid_ds {
        pub shm_perm: crate::ipc_perm,
        pub shm_segsz: size_t,
        pub shm_atime: crate::time_t,
        pub shm_dtime: crate::time_t,
        pub shm_ctime: crate::time_t,
        pub shm_cpid: crate::pid_t,
        pub shm_lpid: crate::pid_t,
        pub shm_nattch: c_ulong,
        __pad1: Padding<c_ulong>,
        __pad2: Padding<c_ulong>,
    }

    pub struct msqid_ds {
        pub msg_perm: crate::ipc_perm,
        pub msg_stime: crate::time_t,
        pub msg_rtime: crate::time_t,
        pub msg_ctime: crate::time_t,
        pub __msg_cbytes: c_ulong,
        pub msg_qnum: crate::msgqnum_t,
        pub msg_qbytes: crate::msglen_t,
        pub msg_lspid: crate::pid_t,
        pub msg_lrpid: crate::pid_t,
        __pad1: Padding<c_ulong>,
        __pad2: Padding<c_ulong>,
    }

    pub struct sem_t {
        __val: [c_int; 8],
    }
}

pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 12;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 24;

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(any(target_arch = "x86_64"))] {
        mod x86_64;
        pub use self::x86_64::*;
    } else if #[cfg(any(target_arch = "riscv64"))] {
        mod riscv64;
        pub use self::riscv64::*;
    } else {
        // Unknown target_arch
    }
}
