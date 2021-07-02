use super::super::kernel::stddef::*;

pub struct Syssvc {}

const TMAX_LONINFO: usize = 6;

pub const LOG_TYPE_COMMENT: u32 = 0x1;

pub const LOG_EMERG: u32 = 0x0;
pub const LOG_ALERT: u32 = 0x1;
pub const LOG_CRIT: u32 = 0x2;
pub const LOG_ERROR: u32 = 0x3;
pub const LOG_WARNING: u32 = 0x4;
pub const LOG_NOTICE: u32 = 0x5;
pub const LOG_INFO: u32 = 0x6;
pub const LOG_DEBUG: u32 = 0x7;

#[repr(C)]
pub struct Syslog {
    pub logtype: ToppersUInt,
    pub logtim: Systim,
    pub loginfo: [u32; TMAX_LONINFO],
}

extern "C" {
    pub fn syslog_wri_log(prio: u32, p_syslog: *const Syslog) -> Er;
    fn syslog_msk_log(logmask: u32, lowmask: u32) -> Er;
}

impl Syslog {
    pub fn syslog_msk_log(logmask: u32, lowmask: u32) -> Er {
        unsafe { syslog_msk_log(logmask, lowmask) }
    }
    pub fn syslog_log_mask(prio : u32) -> u32{
        1u32 << prio
    }
    pub fn syslog_log_upto(prio : u32) -> u32{
        (1u32 << (prio + 1)) - 1
    }
}

