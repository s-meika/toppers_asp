//
// sil_[r|wr][b|h|w]_memはRustで実装すべきと判断して提供していない
//
//

pub struct Sil{

}

extern "C"{
    fn sil_dly_nse(dlytim : u32);
}


impl Sil{

    /// 微小時間待ち
    /// * `dlytime` - 待ち時間(ns)
    pub fn sil_dly_nse(dlytim : u32){
        unsafe{
            sil_dly_nse(dlytim)
        }
    }
}
