#[macro_use]
extern crate weechat;
extern crate libc;

use weechat::WeechatPlugin;
use weechat::init::Args;

struct SamplePlugin {
    weechat_plugin: WeechatPlugin
}

impl weechat::Plugin for SamplePlugin {
    fn init(plugin: WeechatPlugin, _args: Args) -> weechat::init::PluginResult<Self> {
        plugin.print("Hello Rust!");
        plugin.buffer_new("Test buffer");
        Ok(SamplePlugin {
            weechat_plugin: plugin
        })
    }
}

impl Drop for SamplePlugin {
    fn drop(&mut self) {
        self.weechat_plugin.print("Bye rust!");
    }
}

weechat_plugin!(
    SamplePlugin,
    name: b"rust_sample\0"; 12,
    author: b"poljar\0"; 7,
    description: b"\0"; 1,
    version: b"0.1.0\0"; 6,
    license: b"MIT\0"; 4
);
