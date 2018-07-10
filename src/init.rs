use libc::{c_int, c_char};
use plugin::WeechatPlugin;
use bindings;
use std::ffi::CStr;
use std::ops::Index;

extern "C" {
    pub static weechat_plugin: i32;
}

#[macro_export]
macro_rules! weechat_plugin_init(
    ($init_fn:path) => {
        #[no_mangle]
        pub extern "C" fn weechat_plugin_init(plugin: *mut $crate::bindings::t_weechat_plugin,
                                              argc: ::libc::c_int, argv: *mut *mut ::libc::c_char) -> ::libc::c_int {
            return $crate::init::init(plugin, argc, argv, $init_fn);
        }
    };
);

#[macro_export]
macro_rules! weechat_plugin_end(
    ($end_fn:path) => {
        #[no_mangle]
        pub extern "C" fn weechat_plugin_end(plugin: *mut $crate::bindings::t_weechat_plugin) -> ::libc::c_int {
            return $crate::init::end(plugin, $end_fn);
        }
    };
);

pub struct Args {
    argc: u32,
    argv: *mut *mut c_char,
}

impl Args {
    pub fn len(&self) -> usize {
        self.argc as usize
    }
}

impl Index<usize> for Args {
    type Output = CStr;

    fn index<'a>(&'a self, index: usize) -> &'a CStr {
        assert!(index < self.len());

        unsafe {
            let ptr = self.argv.offset(index as isize);
            CStr::from_ptr(ptr as *const c_char)
        }
    }
}

pub fn init(plugin: *mut bindings::t_weechat_plugin,
            argc: c_int, argv: *mut *mut c_char,
            f: fn(plugin: WeechatPlugin, args: Args) -> bool) -> c_int {
    let plugin = WeechatPlugin::from_ptr(plugin);
    let args = Args {
        argc: argc as u32,
        argv: argv,
    };
    let result = f(plugin, args);

    if result { bindings::WEECHAT_RC_OK } else { bindings::WEECHAT_RC_ERROR }
}

pub fn end(plugin: *mut bindings::t_weechat_plugin,
            f: fn(plugin: WeechatPlugin) -> bool) -> c_int {
    let plugin = WeechatPlugin::from_ptr(plugin);
    let result = f(plugin);

    if result { bindings::WEECHAT_RC_OK } else { bindings::WEECHAT_RC_ERROR }
}

#[macro_export]
macro_rules! weechat_plugin_name(
    ($name:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static weechat_plugin_name: [u8; $length] = *$name;

        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_api_version: [u8; $crate::bindings::WEECHAT_PLUGIN_API_VERSION_LENGTH]
            = *$crate::bindings::WEECHAT_PLUGIN_API_VERSION;
    }
);

#[macro_export]
macro_rules! weechat_plugin_author(
    ($author:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_author: [u8; $length] = *$author;
    }
);

#[macro_export]
macro_rules! weechat_plugin_description(
    ($description:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_description: [u8; $length] = *$description;
    }
);

#[macro_export]
macro_rules! weechat_plugin_version(
    ($version:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_version: [u8; $length] = *$version;
    }
);

#[macro_export]
macro_rules! weechat_plugin_license(
    ($license:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_license: [u8; $length] = *$license;
    }
);

#[macro_export]
macro_rules! weechat_plugin_priority(
    ($priority:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static weechat_plugin_priority: ::libc::c_int = $priority;
    }
);

#[macro_export]
macro_rules! weenat_plugin_info(
    (name: $name:expr; $length:expr) => {
        weechat_plugin_name!($name, $length);
    };
    (author: $author:expr; $length:expr) => {
        weechat_plugin_author!($author, $length);
    };
    (description: $description:expr; $length:expr) => {
        weechat_plugin_description!($description, $length);
    };
    (version: $version:expr; $length:expr) => {
        weechat_plugin_version!($version, $length);
    };
    (license: $license:expr; $length:expr) => {
        weechat_plugin_license!($license, $length);
    };
    (priority: $priority:expr) => {
        weechat_plugin_priority!($priority, $length);
    };
);

#[macro_export]
macro_rules! weechat_plugin(
    ($($name:ident: $value:expr; $length:expr),+) => {
        $(
            weenat_plugin_info!($name: $value; $length);
        )+
    };
    ($($name:ident: $value:expr; $length:expr),+,) => {
        weechat_plugin!($($name: $value; $length),+);
    };
);
