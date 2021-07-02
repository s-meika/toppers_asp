///
/// カーネル機能の呼び出し
///
use super::stddef::*;
use core::ffi::c_void;

pub struct Asp {
}


// 処理単位の型定義は省略

// メッセージヘッダ
/* pub struct Tmsg{
    pub pk_next : *Tmsg;
} */

// T_MSG_PRIは使用しないので省略

// パケット形式の定義
#[repr(C)]
pub struct TRtsk {
    tskstat: Stat, // タスク状態
    tskpri: Pri,   // タスクの現在優先度
    tskbpri: Pri,  // タスクのベース優先度
    tskwait: Stat, // 待ち要因
    wobjid: Id,    // 待ち対象のオブジェクトのID
    lefttmo: Tmo,  // タイムアウトするまでの時間
    actcnt: u32,   // 起動要求キューイング数
    wupcnt: u32,   // 起床要求キューイング数
}

#[repr(C)]
pub struct TRtex {
    texstat: Stat,  // タスク例外処理の状
    pndptn: TexPtn, // 保留例外要因
}

#[repr(C)]
pub struct TRsem {
    wtskid: Id,  // セマフォの待ち行列の先頭のタスクのID番号
    semcnt: u32, // セマフォの現在の資源数
}

#[repr(C)]
pub struct TRflg {
    wtskid: Id,     // イベントフラグの待ち行列の先頭のタスクのID番号
    flgptn: FlgPtn, // イベントフラグの現在のビットパターン
}

#[repr(C)]
pub struct TRdtq {
    stskid: Id,   // データキューの送信待ち行列の先頭のタスクのID番号
    rtskid: Id,   // データキューの受信待ち行列の先頭のタスクのID番
    sdtqcnt: u32, // データキュー管理領域に格納されているデータの数
}

#[repr(C)]
pub struct TRpdq {
    stskid: Id,   // 優先度データキューの送信待ち行列の先頭のタスクのID番号
    rtskid: Id,   // 優先度データキューの受信待ち行列の先頭のタスクのID番
    sdtqcnt: u32, // 優先度データキュー管理領域に格納されているデータの数
}

// 未完成
#[repr(C)]
pub struct TRmbx {
    wtskid: Id, // メールボックスの待ち行列の先頭のタスクのID番号
}

#[repr(C)]
pub struct TRmpf {
    wtskid: Id,   // 固定長メモリプールの待ち行列の先頭のタスクのID番号
    fblkcnt: u32, // 固定長メモリプール領域の空きメモリ領域に割り付けることができる固定長メモリブロックの数
}

#[repr(C)]
pub struct TRcyc {
    cycstat: Stat,   // 周期ハンドラの動作状態
    lefttim: Reltim, // 次に周期ハンドラを起動する時刻までの相対時間
}

#[repr(C)]
pub struct TRalm {
    almstat: Stat,   // アラームハンドラの動作状態
    lefttim: Reltim, // アラームハンドラを起動する時刻までの相対時間
}

// APIのプロトタイプ
extern "C" {
    /* タスク管理機能 */
    fn act_tsk(tskid: Id) -> Er;
    fn iact_tsk(tskid: Id) -> Er;
    fn can_act(tskid: Id) -> ErUint;
    fn ext_tsk() -> Er;
    fn ter_tsk(tskid: Id) -> Er;
    fn chg_pri(tskid: Id, tskpri: Pri) -> Er;
    fn get_pri(tksid: Id, Pri: *mut Pri) -> Er;
    fn get_inf(p_exinf: *mut u32) -> Er;
    fn ref_tsk(tskid: Id, pk_rtsk: *mut TRtsk) -> Er;

    /* タスク付属同期機能 */
    fn slp_tsk() -> Er;
    fn tslp_tsk(tmout : Tmo) -> Er;
    fn wup_tsk(tskid : Id) -> Er;
    fn iwup_tsk(tskid : Id) -> Er;
    fn can_wup(tskid : Id) -> Er;
    fn rel_wai(tskid : Id) -> Er;
    fn irel_wai(tskid : Id) -> Er;
    fn sus_tsk(tskid : Id) -> Er;
    fn rsm_tsk(tskid : Id) -> Er;
    fn dly_tsk(tskid : Id) -> Er;

    /* タスク例外処理機能 */
    fn ras_tex(tskid : Er, rasptn : TexPtn) -> Er;
    fn iras_tex(tskid : Er, rasptn : TexPtn) -> Er;
    fn dis_tex() -> Er;
    fn ena_tex() -> Er;
    fn sns_tex() -> bool;
    fn ref_tex(tskid : Id, pk_rtex: *mut TRtex) -> Er;

    /* 時間管理機能 */
    fn get_tim(p_systim : *mut Systim) -> Er;
    fn get_utm(p_sysutm : *mut Sysutm) -> Er;

    fn sta_cyc(cycid : Id) -> Er;
    fn stp_cyc(cycid : Id) -> Er;
    fn ref_cyc(cycid : Id, pk_rcyc : *mut TRcyc) -> Er;
    
    fn sta_alm(almid : Id, reltim : Reltim) -> Er;
    fn ista_alm(almid : Id, reltim : Reltim) -> Er;
    fn stp_alm(almid : Id) -> Er;
    fn istp_alm(almid : Id) -> Er;
    fn ref_alm(almid : Id, pk_ralm : *mut TRalm) -> Er;

    /* システム状態管理機能 */
    fn rot_rdq(tskpri : Pri) -> Er;
    fn irot_rdq(tskpri : Pri) -> Er;
    fn get_tid(p_tskid : *mut Id) -> Er;
    fn iget_tid(p_tskid : *mut Id) -> Er;
    fn loc_cpu() -> Er;
    fn iloc_cpu() -> Er;
    fn unl_cpu() -> Er;
    fn iunl_cpu() -> Er;
    fn dis_dsp() -> Er;
    fn ena_dsp() -> Er;
    fn sns_ctx() -> bool;
    fn sns_loc() -> bool;
    fn sns_dsp() -> bool;
    fn sns_dpn() -> bool;
    fn sns_ker() -> bool;
    fn ext_ker() -> Er;
    
    /* CPU例外管理機能 */
    fn xsns_dpn(p_excinf : *const c_void) -> bool;
    fn xsns_xpn(p_excinf : *const c_void) -> bool;
}

impl Asp {
    pub fn act_tsk(tskid: Id) -> Er {
        unsafe { act_tsk(tskid) }
    }

    pub fn iact_tsk(tskid: Id) -> Er {
        unsafe { iact_tsk(tskid) }
    }

    pub fn can_act(tskid: Id) -> ErUint {
        unsafe { can_act(tskid) }
    }

    pub fn ext_tsk() -> Er {
        unsafe { ext_tsk() }
    }

    pub fn ter_tsk(tskid: Id) -> Er {
        unsafe { ter_tsk(tskid) }
    }

    pub fn chg_pri(tskid: Id, tskpri: Pri) -> Er {
        unsafe { chg_pri(tskid, tskpri) }
    }

    pub fn get_pri(tskid: Id) -> (Er, Pri) {
        let ercd: Er;
        let mut pri: Pri = 0;

        unsafe {
            ercd = get_pri(tskid, &mut pri);
        }

        (ercd, pri)
    }

    pub fn get_inf() -> (Er, u32) {
        let mut exinf: u32 = 0;
        let ercd: Er;

        unsafe {
            ercd = get_inf(&mut exinf);
        }

        (ercd, exinf)
    }

    pub fn ref_tsk(tskid: Id, pk_rtsk: &mut TRtsk) -> Er {
        unsafe { ref_tsk(tskid, pk_rtsk ) }
    }

    pub fn slp_tsk() -> Er{
        unsafe{ slp_tsk() }
    }

    pub fn tslp_tsk(tmout : Tmo) -> Er{
        unsafe{ tslp_tsk(tmout) }
    }

    pub fn wup_tsk(tskid : Id) -> Er{
        unsafe{ wup_tsk(tskid) }
    }

    pub fn iwup_tsk(tskid : Id) -> Er{
        unsafe{ iwup_tsk(tskid) }
    }

    pub fn can_wup(tskid : Id) -> Er{
        unsafe{ can_wup(tskid) }
    }

    pub fn rel_wai(tskid : Id) -> Er{
        unsafe{ rel_wai(tskid) }
    }

    pub fn irel_wai(tskid : Id) -> Er{
        unsafe{ rel_wai(tskid) }
    }

    pub fn sus_tsk(tskid : Id) -> Er{
        unsafe{ sus_tsk(tskid) }
    }

    pub fn rsm_tsk(tskid : Id) -> Er{
        unsafe{ rsm_tsk(tskid) }
    }

    pub fn dly_tsk(tskid : Id) -> Er{
        unsafe{ dly_tsk(tskid) }
    }

    pub fn ras_tex(tskid : Id, rasptn : TexPtn) -> Er{
        unsafe { ras_tex(tskid, rasptn) }
    }

    pub fn iras_tex(tskid : Id, rasptn : TexPtn) -> Er{
        unsafe { iras_tex(tskid, rasptn) }
    }

    pub fn dis_tex() -> Er{
        unsafe { dis_tex() }
    }

    pub fn ena_tex() -> Er{
        unsafe { ena_tex() }
    }

    pub fn sns_tex() -> bool{
        unsafe { sns_tex() }
    }

    pub fn ref_tex(tskid : Id, pk_rtex : &mut TRtex) -> Er{
        unsafe{ ref_tex(tskid, pk_rtex) }
    }

    pub fn get_tim() -> (Er, Systim){
        let ercd : Er;
        let mut systim : Systim = 0;
        unsafe{
            ercd = get_tim(&mut systim);
        }
        (ercd, systim)
    }
    pub fn get_utm() -> (Er, Sysutm){
        let ercd : Er;
        let mut sysutm : Sysutm = 0;
        unsafe{
            ercd = get_utm(&mut sysutm);
        }
        (ercd, sysutm)
    }

    pub fn sta_cyc(cycid : Id) -> Er{
        unsafe{ sta_cyc(cycid) }
    }
    pub fn stp_cyc(cycid : Id) -> Er{
        unsafe{ stp_cyc(cycid) }
    }
    pub fn ref_cyc(cycid : Id, pk_rcyc : &mut TRcyc) -> Er{
        unsafe{ ref_cyc(cycid, pk_rcyc) }
    }
    
    pub fn sta_alm(almid : Id, reltim : Reltim) -> Er{
        unsafe{ sta_alm(almid, reltim,) }
    }
    pub fn ista_alm(almid : Id, reltim : Reltim) -> Er{
        unsafe{ ista_alm(almid, reltim,) }
    }
    pub fn stp_alm(almid : Id) -> Er{
        unsafe{ stp_alm(almid) }
    }
    pub fn istp_alm(almid : Id) -> Er{
        unsafe{ istp_alm(almid) }
    }
    pub fn ref_alm(almid : Id, pk_ralm : &mut TRalm) -> Er{
        unsafe{ ref_alm(almid, pk_ralm) }
    }

    pub fn rot_rdq(tskpri : Pri) -> Er{
        unsafe{ rot_rdq(tskpri) }
    }
    pub fn irot_rdq(tskpri : Pri) -> Er{
        unsafe{ irot_rdq(tskpri) }
    }

    pub fn get_tid() -> (Er, Id){
        let mut tskid : Id = 0;
        let ercd : Er;
        unsafe{ ercd = get_tid(&mut tskid) }
        (ercd, tskid)
    }

    pub fn iget_tid() -> (Er, Id){
        let mut tskid : Id = 0;
        let ercd : Er;
        unsafe{ ercd = iget_tid(&mut tskid) }
        (ercd, tskid)
    }

    pub fn loc_cpu() -> Er{
        unsafe{ loc_cpu() }
    }

    pub fn iloc_cpu() -> Er{
        unsafe{ iloc_cpu() }
    }

    pub fn unl_cpu() -> Er{
        unsafe{ unl_cpu() }
    }
    
    pub fn iunl_cpu() -> Er{
        unsafe{ iunl_cpu() }
    }

    pub fn dis_dsp() -> Er{
        unsafe{ dis_dsp() }
    }

    pub fn ena_dsp() -> Er{
        unsafe{ ena_dsp() }
    }
    
    pub fn sns_ctx() -> bool{
        unsafe{ sns_ctx() }
    }
    
    pub fn sns_loc() -> bool{
        unsafe{ sns_loc() }
    }

    pub fn sns_dsp() -> bool{
        unsafe{ sns_dsp() }
    }

    pub fn sns_dpn() -> bool{
        unsafe{ sns_dpn() }
    }

    pub fn sns_ker() -> bool{
        unsafe{ sns_ker() }
    }

    pub fn ext_ker() -> Er{
        unsafe{ ext_ker() }
    }

    pub fn xsns_dpn(p_excinf : &c_void) -> bool{
        unsafe{ xsns_dpn(p_excinf) }
    }

    pub fn xsns_xpn(p_excinf : &c_void) -> bool{
        unsafe{ xsns_xpn(p_excinf) }
    }

}

/*
 * オブジェクト属性
 */

const TA_ACT: u32 = 0x02;

const TA_TPRI: u32 = 0x01;
const TA_MPRI: u32 = 0x02;

const TA_WMUL: u32 = 0x02;
const TA_CLR: u32 = 0x04;

const TA_STA: u32 = 0x02;

const TA_NOKERNEL: u32 = 0x02;

const TA_ENAINT: u32 = 0x01;
const TA_EGDE: u32 = 0x02;

/*
 * サービスコールの動作モード
 */

const TWF_ORW: u32 = 0x01;
const TWF_ANDW: u32 = 0x02;

/*
 * オブジェクトの状態の定義
 */
const TTS_RUN: u32 = 0x01;
const TTS_RDY: u32 = 0x02;
const TTS_WAI: u32 = 0x04;
const TTS_SUS: u32 = 0x08;
const TTS_WAS: u32 = 0x0c;
const TTS_DMT: u32 = 0x10;

const TTW_SLP: u32 = 0x0001;
const TTW_DLY: u32 = 0x0002;
const TTW_SEM: u32 = 0x0004;
const TTW_FLG: u32 = 0x0008;
const TTW_SDTQ: u32 = 0x0010;
const TTW_RDTQ: u32 = 0x0020;
const TTW_SPDQ: u32 = 0x0100;
const TTW_RPDQ: u32 = 0x0200;
const TTW_MBX: u32 = 0x0040;
const TTW_MPF: u32 = 0x2000;

const TTEX_ENA: u32 = 0x01;
const TEX_DIS: u32 = 0x02;

const TCYC_STP: u32 = 0x01;
const TCYC_STA: u32 = 0x02;

const TALM_STP: u32 = 0x01;
const TALM_STA: u32 = 0x02;

/*
 * その他の定数の定義
 */
const TSK_SELF: u32 = 0;
const TSK_NONE: u32 = 0;
const TPRI_SELF: u32 = 0;
const TPRI_INI: u32 = 0;
const TIPM_ENAALL: u32 = 0;

/*
 * 優先度の範囲
 */

const TMIN_TPRI: u32 = 1;
const TMAX_TPRI: u32 = 16;
const TMIN_DPRI: u32 = 1;
const TMAX_DPRI: u32 = 16;
const TMIN_MPRI: u32 = 1;
const TMAX_MPRI: u32 = 16;
const TMIN_ISRPRI: u32 = 1;
const TMAX_ISRPRI: u32 = 16;
