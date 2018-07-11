#[macro_use]
extern crate weechat;
extern crate libc;

use weechat::Weechat;
use weechat::init::Args;

struct SamplePlugin {
    weechat: Weechat
}

impl weechat::WeechatPlugin for SamplePlugin {
    fn init(weechat: Weechat, _args: Args) -> weechat::WeechatResult<Self> {
        weechat.print("Hello Rust!");
        weechat.buffer_new("Test buffer");
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
