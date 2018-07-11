extern crate libc;

pub mod init;
pub mod bindings;
pub mod weechat;

pub use weechat::Weechat;
pub use init::Plugin;
pub use init::PluginResult;
