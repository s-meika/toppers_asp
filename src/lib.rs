#![no_std]
#[allow(unused_variables)]
#[allow(dead_code)]

pub mod kernel;
pub mod library;
pub mod syssvc;

#[macro_export]
macro_rules! toppers_syssvc_syslog{
    ($prio : expr, $fmt : expr, $($arg : expr),*) => {
        let ini_ary = {
            let mut ary : [u32; 6] = [0; 6];            
            ary[0] = concat!($fmt, '\0').as_bytes().as_ptr() as u32;
            let mut _index = 1;
            
            $(
                {
                    ary[_index] = $arg;
                    _index = _index + 1;
                }
            )*
            ary
        } ; 

        let mut _syslog = Syslog {
            logtype : LOG_TYPE_COMMENT,
            logtim : 0,
            loginfo : ini_ary
        };

        unsafe{
            let _ = syslog_wri_log($prio, &_syslog);
        }
    };
}

#[macro_export]
macro_rules! toppers_assert_impl {
    ($expr : expr, $msg : expr) => {
        if $expr == false  {
            toppers_syssvc_syslog!(LOG_EMERG, $msg,);
         }
    };
}

#[macro_export]
macro_rules! toppers_assert {
    ($expr : expr) => {
        toppers_assert_impl!($expr, concat!(file!(), line!(), stringify!($expr)));
    };
}
