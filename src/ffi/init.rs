use ffi::consts;
use libc::{c_int, c_char};
use plugin::WeechatPlugin;
use std::ffi::CStr;
use std::ops::Index;

#[macro_export]
macro_rules! weechat_plugin_init(
    ($init_fn:path) => {
        #[no_mangle]
        pub extern "C" fn weechat_plugin_init(plugin: *mut $crate::ffi::structs::t_weechat_plugin,
                                              argc: ::libc::c_int, argv: *mut *mut ::libc::c_char) -> ::libc::c_int {
            return $crate::ffi::init::init(plugin, argc, argv, $init_fn);
        }
    };
);

#[macro_export]
macro_rules! weechat_plugin_end(
    ($end_fn:path) => {
        #[no_mangle]
        pub extern "C" fn weechat_plugin_end(plugin: *mut $crate::ffi::structs::t_weechat_plugin) -> ::libc::c_int {
            return $crate::ffi::init::end(plugin, $end_fn);
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

pub fn init(plugin: *mut super::structs::t_weechat_plugin,
            argc: c_int, argv: *mut *mut c_char,
            f: fn(plugin: WeechatPlugin, args: Args) -> bool) -> c_int {
    let plugin = WeechatPlugin::from_ptr(plugin);
    let args = Args {
        argc: argc as u32,
        argv: argv,
    };
    let result = f(plugin, args);

    if result { consts::WEECHAT_RC_OK } else { consts::WEECHAT_RC_ERROR }
}

pub fn end(plugin: *mut super::structs::t_weechat_plugin,
            f: fn(plugin: WeechatPlugin) -> bool) -> c_int {
    let plugin = WeechatPlugin::from_ptr(plugin);
    let result = f(plugin);

    if result { consts::WEECHAT_RC_OK } else { consts::WEECHAT_RC_ERROR }
}
