use libc::c_int;
use weechat::{ArgsWeechat, Weechat};

pub trait WeechatPlugin: Sized {
    fn init(weechat: Weechat, args: ArgsWeechat) -> WeechatResult<Self>;
}

pub struct Error(c_int);
pub type WeechatResult<T> = Result<T, Error>;
