use super::super::kernel::stddef::*;

pub const IOCTL_NULL : u32 = 0;			/* 指定なし */
pub const IOCTL_ECHO : u32 = 0x0001;		/* 受信した文字をエコーバック */
pub const IOCTL_CRLF : u32 = 0x0010;		/* LFを送信する前にCRを付加 */
pub const IOCTL_FCSND : u32 = 0x0100;		/* 送信に対してフロー制御を行う */
pub const IOCTL_FCANY : u32 = 0x0200;		/* どのような文字でも送信再開 */
pub const IOCTL_FCRCV : u32 = 0x0400;		/* 受信に対してフロー制御を行う */

#[repr(C)]
pub struct SerialRpor {
    pub reacnt: u32, // 受信バッファ中の文字数
    pub wricnt: u32, // 送信バッファ中の文字数
}

extern "C" {
    pub fn serial_opn_por(id: Id) -> Er;
    pub fn serial_cls_por(id: Id) -> Er;
    pub fn serial_rea_dat(id: Id, buf: *mut u8, len: u32) -> ErUint;
    pub fn serial_wri_dat(id: Id, buf: *const u8, len: u32) -> ErUint;
    pub fn serial_ctl_por(id: Id, ioctl: u32) -> Er;
    pub fn serial_ref_por(id: Id, pk_rpor: *mut SerialRpor) -> Er;
}

pub struct Serial {}

impl Serial {
    pub fn serial_opn_por(id: Id) -> Er {
        unsafe { serial_opn_por(id) }
    }
    pub fn serial_cls_por(id: Id) -> Er {
        unsafe { serial_cls_por(id) }
    }
    pub fn serial_rea_dat(id: Id, buf: *mut u8, len: u32) -> ErUint {
        unsafe { serial_rea_dat(id, buf, len) }
    }
    pub fn serial_wri_dat(id: Id, buf: *mut u8, len: u32) -> ErUint {
        unsafe { serial_wri_dat(id, buf, len) }
    }
    pub fn serial_ctl_por(id: Id, ioctl: u32) -> Er {
        unsafe { serial_ctl_por(id, ioctl) }
    }
    pub fn serial_ref_por(id: Id) -> (Er, u32, u32) {
        let ercd: Er;
        let mut rpor: SerialRpor = SerialRpor {
            reacnt: 0,
            wricnt: 0,
        };

        unsafe { ercd = serial_ref_por(id, &mut rpor) }

        (ercd, rpor.reacnt, rpor.wricnt)
    }
}
