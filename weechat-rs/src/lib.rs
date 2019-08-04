pub mod buffer;
pub mod completion;
pub mod config;
pub mod config_options;
pub mod hooks;
pub mod infolist;
pub mod plugin;
pub mod weechat;

pub use weechat_macro::weechat_plugin;

pub use plugin::{WeechatPlugin, WeechatResult};
pub use weechat::{ArgsWeechat, Weechat};

pub use buffer::{Buffer, Nick, NickArgs};

pub use config::{Config, ConfigSection, ConfigSectionInfo};
pub use config_options::{
    BooleanOption, ColorOption, ConfigOption, IntegerOption, StringOption,
};

pub use hooks::{CommandDescription, CommandHook, FdHook, FdHookMode};

pub use infolist::Infolist;

use std::ffi::CString;

/// Status values for weechat callbacks
pub enum ReturnCode {
    Ok = weechat_sys::WEECHAT_RC_OK as isize,
    OkEat = weechat_sys::WEECHAT_RC_OK_EAT as isize,
    Error = weechat_sys::WEECHAT_RC_ERROR as isize,
}

pub(crate) struct LossyCString;

impl LossyCString {
    pub(crate) fn new<T: AsRef<str>>(t: T) -> CString {
        match CString::new(t.as_ref()) {
            Ok(cstr) => cstr,
            Err(_) => CString::new(t.as_ref().replace('\0', ""))
                .expect("string has no nulls"),
        }
    }
}
