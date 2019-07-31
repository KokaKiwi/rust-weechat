#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const WEECHAT_PLUGIN_API_VERSION_LENGTH: usize = 12;

/* return codes for plugin functions */
pub const WEECHAT_RC_OK: c_int = 0;
pub const WEECHAT_RC_ERROR: c_int = 0;
