#[macro_use]
extern crate weechat;
extern crate libc;

use weechat::{
    Weechat,
    WeechatPlugin,
    WeechatPluginArgs,
    WeechatResult,
    Buffer,
    Nick
};
use std::time::Instant;

struct SamplePlugin {
    weechat: Weechat
}

fn input_cb(data: &Option<&str>, buffer: Buffer, input: &str) {
    match data {
        Some(x) => buffer.print(x),
        None => buffer.print(input),
    };
}

fn close_cb(buffer: Buffer) {
    let w = buffer.get_weechat();
    w.print("Closing buffer")
}

impl WeechatPlugin for SamplePlugin {
    fn init(weechat: Weechat, _args: WeechatPluginArgs) -> WeechatResult<Self> {
        weechat.print("Hello Rust!");

        static input: &'static Option<&str> = &Some("hello");
        let buffer = weechat.buffer_new::<&str>("Test buffer", Some(input_cb), input, Some(close_cb));
        buffer.print("Hello test buffer");

        let n = 100;

        let now = Instant::now();

        for nick_number in 0..n {
            let mut nick = Nick {
                name: &format!("nick_{}", &nick_number.to_string()),
                ..Default::default()
            };
            buffer.add_nick(&mut nick);
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
