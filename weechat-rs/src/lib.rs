extern crate libc;
extern crate weechat_sys;

pub mod plugin;
pub mod weechat;
pub mod buffer;

pub use weechat::Weechat;
pub use buffer::Buffer;
pub use buffer::Nick;
pub use buffer::NickArgs;
pub use plugin::WeechatPlugin;
pub use plugin::WeechatResult;
pub use plugin::WeechatPluginArgs;
