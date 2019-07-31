use crate::{ArgsWeechat, Weechat};
use libc::c_int;

pub trait WeechatPlugin: Sized {
    fn init(weechat: Weechat, args: ArgsWeechat) -> WeechatResult<Self>;
}

pub struct Error(c_int);
pub type WeechatResult<T> = Result<T, Error>;
