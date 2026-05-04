//! Redox OS libc.
//!
//! * Headers: <https://gitlab.redox-os.org/redox-os/relibc>

pub(crate) mod unistd;
pub(crate) mod pthread;
pub(crate) mod sys {
    pub(crate) mod socket;
}
