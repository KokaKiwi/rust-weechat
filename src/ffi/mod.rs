pub mod consts;
pub mod infos;
pub mod init;
pub mod structs;

extern "C" {
    pub static weechat_plugin: i32;
}
