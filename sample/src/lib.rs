#[macro_use]
extern crate weechat;
extern crate libc;

use std::time::Instant;
use weechat::{
    ArgsWeechat, Buffer, CommandHook, CommandDescription, Config,
    ConfigSectionInfo, NickArgs, OptionDescription, Weechat, WeechatPlugin,
    WeechatResult,
};

struct SamplePlugin {
    weechat: Weechat,
    _rust_hook: CommandHook<String>,
    _rust_config: Config,
}

impl SamplePlugin {
    fn input_cb(data: &mut String, buffer: Buffer, _input: &str) {
        buffer.print(data);
        if data == "Hello" {
            data.push_str(" world.");
        }
    }

    fn close_cb(_data: &(), buffer: Buffer) {
        let w = buffer.get_weechat();
        w.print("Closing buffer")
    }

    fn rust_command_cb(data: &String, buffer: Buffer, args: ArgsWeechat) {
        buffer.print(data);
        for arg in args {
            buffer.print(&arg)
        }
    }
}

impl WeechatPlugin for SamplePlugin {
    fn init(weechat: Weechat, _args: ArgsWeechat) -> WeechatResult<Self> {
        weechat.print("Hello Rust!");

        let buffer: Buffer = weechat.buffer_new(
            "Test buffer",
            Some(SamplePlugin::input_cb),
            Some("Hello".to_owned()),
            Some(SamplePlugin::close_cb),
            None,
        );
        buffer.print("Hello test buffer");

        let n = 100;

        let now = Instant::now();

        let op_group = buffer.add_group("operators", "blue", true, None);
        let emma = buffer.add_nick(
            NickArgs {
                name: "Emma",
                color: "magenta",
                prefix: "&",
                prefix_color: "green",
                ..Default::default()
            },
            Some(&op_group),
        );

        weechat.print(&format!("Nick name getting test: {}", emma.get_name()));

        for nick_number in 0..n {
            let nick = NickArgs {
                name: &format!("nick_{}", &nick_number.to_string()),
                ..Default::default()
            };
            let _ = buffer.add_nick(nick, None);
        }

        buffer.print(&format!(
            "Elapsed time for {} nick additions: {}.{}s.",
            &n.to_string(),
            &now.elapsed().as_secs().to_string(),
            &now.elapsed().subsec_millis().to_string()
        ));

        let sample_command = CommandDescription {
            name: "rustcommand",
            ..Default::default()
        };

        let command = weechat.hook_command(
            sample_command,
            SamplePlugin::rust_command_cb,
            Some("Hello rust command".to_owned()),
        );

        let mut config =
            weechat.config_new("rust_sample", None, None::<String>);

        let section_info: ConfigSectionInfo<String> = ConfigSectionInfo {
            name: "sample_section",
            ..Default::default()
        };

        let section = config.new_section(section_info);

        let option_info = OptionDescription {
            name: "test_option",
            option_type: "string",
            ..Default::default()
        };

        section.new_option(option_info);

        Ok(SamplePlugin {
            weechat: weechat,
            _rust_hook: command,
            _rust_config: config,
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
