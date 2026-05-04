//! Header: `sys/socket.h`
//!
//! <https://github.com/kraj/musl/blob/kraj/master/include/sys/socket.h>

use crate::prelude::*;


s! {
    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_namelen: crate::socklen_t,
        pub msg_iov: *mut crate::iovec,
        pub msg_iovlen: size_t,
        pub msg_control: *mut c_void,
        pub msg_controllen: size_t,
        pub msg_flags: c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: size_t,
        pub cmsg_level: c_int,
        pub cmsg_type: c_int,
    }
}

extern "C" {
    pub fn sendmmsg(
        sockfd: c_int,
        msgvec: *mut crate::mmsghdr,
        vlen: c_uint,
        flags: c_uint,
    ) -> c_int;
    pub fn recvmmsg(
        sockfd: c_int,
        msgvec: *mut crate::mmsghdr,
        vlen: c_uint,
        flags: c_uint,
        timeout: *mut crate::timespec,
    ) -> c_int;
}

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
