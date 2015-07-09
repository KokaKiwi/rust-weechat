#[macro_use]
extern crate weechat;
extern crate libc;

use weechat::WeechatPlugin;
use weechat::init::Args;

weechat_plugin!(
    name: b"rust_sample\0"; 12,
    author: b"KokaKiwi\0"; 9,
    description: b"\0"; 1,
    version: b"0.1.0\0"; 6,
    license: b"MIT\0"; 4,
);

fn init(plugin: WeechatPlugin, _args: Args) -> bool {
    plugin.print("Hello Rust!");

    true
}

fn end(_plugin: WeechatPlugin) -> bool {
    true
}

weechat_plugin_init!(init);
weechat_plugin_end!(end);
