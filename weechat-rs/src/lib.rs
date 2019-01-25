extern crate libc;
extern crate weechat_sys;

pub mod plugin;
pub mod weechat;
pub mod buffer;
pub mod hooks;
pub mod config;

pub use weechat::Weechat;
pub use weechat::ArgsWeechat;
pub use hooks::{CommandInfo, CommandHook, FdHook};
pub use buffer::{Buffer, Nick, NickArgs};
pub use config::{
    Config,
    ConfigSectionInfo,
    ConfigSection,
    OptionDescription
};
pub use plugin::WeechatPlugin;
pub use plugin::WeechatResult;
