extern crate libc;
extern crate weechat_sys;

pub mod plugin;
pub mod weechat;
pub mod buffer;
pub mod hooks;

pub use weechat::Weechat;
pub use hooks::{Hook, CommandInfo};
pub use buffer::{Buffer, Nick, NickArgs};
pub use plugin::WeechatPlugin;
pub use plugin::WeechatResult;
pub use plugin::WeechatPluginArgs;
