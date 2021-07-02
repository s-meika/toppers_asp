///
/// TOPPERS共通定義
///

// 共通データ型

// bool_t  -> Rust定義使用
// int_t   -> i32を使う
// uint_t  -> u32を使う

#[cfg(target_pointer_width = "32")]
pub type ToppersUInt = u32;
#[cfg(target_pointer_width = "32")]
pub type ToppersInt = i32;

#[cfg(target_pointer_width = "64")]
pub type ToppersUInt = u64;
#[cfg(target_pointer_width = "64")]
pub type ToppersInt = i64;

pub type Fc = ToppersInt; // fnがキーワードなのでfunction codeのfcとした
pub type Er = ToppersInt;
pub type Id = ToppersInt;
pub type Atr = ToppersUInt;
pub type Stat = ToppersUInt;
pub type Mode = ToppersUInt;
pub type Pri = ToppersInt;
pub type Size = ToppersUInt;

pub type Tmo = ToppersInt;
pub type Reltim = ToppersUInt;
pub type Systim = ToppersUInt;
pub type Sysutm = ToppersUInt;
pub type Fp = fn();

pub type ErBool = ToppersInt;
pub type ErUint = ToppersInt;
pub type ErIf = ToppersInt;


// MB_Tは動的生成に関わるのでやらない
pub type Actptn = u32;
pub struct Acvct {
    pub actptn1: Actptn,
    pub actptn2: Actptn,
    pub actptn3: Actptn,
    pub actptn4: Actptn,
}

// ビットパターンやオブジェクト番号(kernel.hより)
pub type TexPtn = u32;
pub type FlgPtn = u32;
pub type IntNo = u32;
pub type InhNo = u32;
pub type ExcNo = u32;

// メールボックスはサポートしないので、メールボックスヘッダは定義しない

// パケット形式のデータも当面使用しない

// 一般定数
const NULL: u32 = 0;
// true, falseはRust定義のものを使う
pub const E_OK: Er = 0;

pub const E_SYS: Er = -5; // システムエラー
pub const E_NOSPT: Er = -9; // 未サポート機能
pub const E_RSFN: Er = -10; // 予約機能コード
pub const E_RSATR: Er = -11; // 予約属性
pub const E_PAR: Er = -17; // パラメータエラー
pub const E_ID: Er = -18; // 不正ID番号
pub const E_CTX: Er = -25; // コンテキストエラー
pub const E_MACV: Er = -26; // メモリアクセス違反
pub const E_OACV: Er = -27; // オブジェクトアクセス違反
pub const E_ILUSE: Er = -28; // サービスコール不正使用
pub const E_NOMEM: Er = -33; // メモリ不足
pub const E_NOID: Er = -34; // ID番号不足
pub const E_NORES: Er = -35; // 資源不足
pub const E_OBJ: Er = -41; // オブジェクト状態エラー
pub const E_NOEXS: Er = -42; // オブジェクト未生成
pub const E_QOVR: Er = -43; // キューイングオーバーフロー
pub const E_RLWAI: Er = -49; // 待ち状態の強制解除
pub const E_TMOUT: Er = -50; // ポーリング失敗またはタイムアウト
pub const E_DLT: Er = -51; // 待ちオブジェクトの削除
pub const E_CLS: Er = -52; // 待ちオブジェクトの状態変化
pub const E_WBLK: Er = -57; // ノンブロッキング受付け
pub const E_BOVR: Er = -58; // バッファオーバーフロー

// オブジェクト属性
pub const TA_NULL: Atr = 0; // オブジェクト属性を指定しない

// タイムアウト指定
pub const TMO_POL: Tmo = 0; // ポーリング
pub const TMO_FEVR: Tmo = 0; // 永久待ち
pub const TMO_NBLK: Tmo = 0; // ノンブロッキング

// アクセス許可パターン
pub const TACP_KERNEL: Actptn = 0; // カーネルドメインだけにアクセスを許可
pub const TACP_SHARED: Actptn = !0; // すべてのドメインからアクセスを許可

// 型に関する情報を取り出すためのマクロはアプリケーションでは不要なので定義を省略


// ER型の文字列変換
#[macro_export]
macro_rules! ercd_to_strptr{
    ($x : expr, $str : expr, $($ercds : path),+) => {
        match mercd($x)
        {
            $(
                $ercds => $str = concat!(stringify!($ercds),'\0').as_bytes().as_ptr(),
            )+
            _ => $str = "unknown error\0".as_bytes().as_ptr(),
        }
    };
}
pub trait ToU8Ptr {
    fn to_u8ptr(self) -> *const u8;
}

impl ToU8Ptr for Er {
    fn to_u8ptr(self) -> *const u8 {
        let strptr: *const u8;

        ercd_to_strptr!(
            self, strptr, E_OK, E_SYS, E_NOSPT, E_RSFN, E_RSATR, E_PAR, E_ID, E_CTX, E_MACV,
            E_OACV, E_ILUSE, E_NOMEM, E_NOID, E_NORES, E_OBJ, E_NOEXS, E_QOVR, E_RLWAI, E_TMOUT,
            E_DLT, E_CLS, E_WBLK, E_BOVR
        );

        strptr
    }
}

// エラーコード生成、分解
pub fn ercd(mercd: Er, sercd: Er) -> Er {
    sercd << 8 | mercd & 0xff
}

pub fn mercd(mercd: Er) -> Er {
    mercd | !0xff
}

pub fn sercd(sercd: Er) -> Er {
    sercd >> 8
}

// アクセス許可パターン生成マクロ
#[macro_export]
macro_rules! toppers_eacp {
    ($domid : expr) => {
        ((1u32 << domid) - 1) as Id
    };
}

// 相対時間（RELTIM）に指定できる最大値
const TMAX_RELTIM: Reltim = u32::MAX;
