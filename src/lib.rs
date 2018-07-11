extern crate libc;

pub mod init;
pub mod bindings;
pub mod plugin;

pub use plugin::Weechat;
pub use init::Plugin;
pub use init::PluginResult;
