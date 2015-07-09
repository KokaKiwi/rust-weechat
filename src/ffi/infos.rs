
#[macro_export]
macro_rules! weechat_plugin_name(
    ($name:expr, $length:expr) => {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static weechat_plugin_name: [u8; $length] = *$name;

        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static mut weechat_plugin_api_version: [u8; $crate::ffi::consts::WEECHAT_PLUGIN_API_VERSION_LENGTH]
            = $crate::ffi::consts::WEECHAT_PLUGIN_API_VERSION;
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
