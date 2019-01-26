use libc::c_int;
use weechat::{ArgsWeechat, Weechat};

extern crate weechat_sys;

#[macro_export]
macro_rules! weechat_plugin(
    ($plugin:ty, $($name:ident: $value:expr; $length:expr),+) => {
        static mut __PLUGIN: Option<$plugin> = None;
        #[no_mangle]
        pub static mut weechat_plugin_api_version: [u8; weechat_sys::WEECHAT_PLUGIN_API_VERSION_LENGTH]
            = *weechat_sys::WEECHAT_PLUGIN_API_VERSION;

        #[no_mangle]
        pub extern "C" fn weechat_plugin_init(plugin: *mut weechat_sys::t_weechat_plugin,
                                              argc: libc::c_int, argv: *mut *mut ::libc::c_char) -> libc::c_int {
            let plugin = Weechat::from_ptr(plugin);
            let args = ArgsWeechat::new(argc, argv);
            match <$plugin as $crate::WeechatPlugin>::init(plugin, args) {
                Ok(p) => {
                    unsafe {
                        __PLUGIN = Some(p)
                    }
                    return weechat_sys::WEECHAT_RC_OK;
                }
                Err(_e) => {
                    return weechat_sys::WEECHAT_RC_ERROR;
                }
            }
        }
        #[no_mangle]
        pub extern "C" fn weechat_plugin_end(_plugin: *mut weechat_sys::t_weechat_plugin) -> ::libc::c_int {
            unsafe {
                // Invokes drop() on __PLUGIN, which should be used for cleanup.
                __PLUGIN = None;
            }

            weechat_sys::WEECHAT_RC_OK
        }
        $(
            weechat_plugin!(@attribute $name, $value, $length);
        )*

    };

    (@attribute $name:ident, $value:expr, $length:expr) => {
        weechat_plugin_info!($name, $value, $length);
    };
);

pub trait WeechatPlugin: Sized {
    fn init(weechat: Weechat, args: ArgsWeechat) -> WeechatResult<Self>;
}

pub struct Error(c_int);
pub type WeechatResult<T> = Result<T, Error>;

#[macro_export]
macro_rules! weechat_plugin_name(
    ($name:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static weechat_plugin_name: [u8; $length] = *$name;
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
macro_rules! weechat_plugin_info(
    (name, $name:expr, $length:expr) => {
        weechat_plugin_name!($name, $length);
    };
    (author, $author:expr, $length:expr) => {
        weechat_plugin_author!($author, $length);
    };
    (description, $description:expr, $length:expr) => {
        weechat_plugin_description!($description, $length);
    };
    (version, $version:expr, $length:expr) => {
        weechat_plugin_version!($version, $length);
    };
    (license, $license:expr, $length:expr) => {
        weechat_plugin_license!($license, $length);
    };
);
