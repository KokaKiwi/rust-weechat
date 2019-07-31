pub mod buffer;
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
