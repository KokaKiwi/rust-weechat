#[macro_use]
extern crate weechat;
extern crate libc;

use weechat::{Weechat, WeechatPlugin, WeechatPluginArgs, WeechatResult, Buffer};

struct SamplePlugin {
    weechat: Weechat
}

fn input_cb(buffer: Buffer, input: &str) {
    buffer.print(input);
}

impl WeechatPlugin for SamplePlugin {
    fn init(weechat: Weechat, _args: WeechatPluginArgs) -> WeechatResult<Self> {
        weechat.print("Hello Rust!");
        let buffer = weechat.buffer_new("Test buffer", Some(input_cb));
        let _ = weechat.buffer_new("Test buffer 2", None);
        buffer.print("Hello buffer");
        Ok(SamplePlugin {
            weechat: weechat
        })
    }
}

impl Drop for SamplePlugin {
    fn drop(&mut self) {
        self.weechat.print("Bye rust!");
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
