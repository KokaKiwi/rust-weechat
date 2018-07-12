#[macro_use]
extern crate weechat;
extern crate libc;

use weechat::{Weechat, WeechatPlugin, WeechatPluginArgs, WeechatResult, Buffer};
use std::time::Instant;

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
        buffer.print("Hello test buffer");

        let n = 20000;

        let now = Instant::now();

        for nick_number in 0..n {
            buffer.add_nick(&format!("nick_{}", &nick_number.to_string()));
        }

        buffer.print(&format!(
            "Elapsed time for {} nick additions: {}.{}s.",
            &n.to_string(),
            &now.elapsed().as_secs().to_string(),
            &now.elapsed().subsec_millis().to_string())
        );
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
