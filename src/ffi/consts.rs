use libc::c_int;

pub const WEECHAT_PLUGIN_API_VERSION_LENGTH: usize = 12;
pub const WEECHAT_PLUGIN_API_VERSION: [u8; WEECHAT_PLUGIN_API_VERSION_LENGTH] = *b"20150704-02\0";

/* return codes for plugin functions */
pub const WEECHAT_RC_OK: c_int = 0;
pub const WEECHAT_RC_OK_EAT: c_int = 1;
pub const WEECHAT_RC_ERROR: c_int = -1;

/* return codes for config read functions/callbacks */
pub const WEECHAT_CONFIG_READ_OK: c_int = 0;
pub const WEECHAT_CONFIG_READ_MEMORY_ERROR: c_int = -1;
pub const WEECHAT_CONFIG_READ_FILE_NOT_FOUND: c_int = -2;

/* return codes for config write functions/callbacks */
pub const WEECHAT_CONFIG_WRITE_OK: c_int = 0;
pub const WEECHAT_CONFIG_WRITE_ERROR: c_int = -1;
pub const WEECHAT_CONFIG_WRITE_MEMORY_ERROR: c_int = -2;

/* null value for option */
pub const WEECHAT_CONFIG_OPTION_NULL: *const u8 = b"null\0" as *const u8;

/* return codes for config option set */
pub const WEECHAT_CONFIG_OPTION_SET_OK_CHANGED: c_int = 2;
pub const WEECHAT_CONFIG_OPTION_SET_OK_SAME_VALUE: c_int = 1;
pub const WEECHAT_CONFIG_OPTION_SET_ERROR: c_int = 0;
pub const WEECHAT_CONFIG_OPTION_SET_OPTION_NOT_FOUND: c_int = -1;

/* return codes for config option unset */
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_NO_RESET: c_int = 0;
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_RESET: c_int = 1;
pub const WEECHAT_CONFIG_OPTION_UNSET_OK_REMOVED: c_int = 2;
pub const WEECHAT_CONFIG_OPTION_UNSET_ERROR: c_int = -1;

/* list management (order of elements) */
pub const WEECHAT_LIST_POS_SORT: *const u8 = b"sort\0" as *const u8;
pub const WEECHAT_LIST_POS_BEGINNING: *const u8 = b"beginning\0" as *const u8;
pub const WEECHAT_LIST_POS_END: *const u8 = b"end\0" as *const u8;

/* type for keys and values in hashtable */
pub const WEECHAT_HASHTABLE_INTEGER: *const u8 = b"integer\0" as *const u8;
pub const WEECHAT_HASHTABLE_STRING: *const u8 = b"string\0" as *const u8;
pub const WEECHAT_HASHTABLE_POINTER: *const u8 = b"pointer\0" as *const u8;
pub const WEECHAT_HASHTABLE_BUFFER: *const u8 = b"buffer\0" as *const u8;
pub const WEECHAT_HASHTABLE_TIME: *const u8 = b"time\0" as *const u8;

/* types for hdata */
pub const WEECHAT_HDATA_OTHER: c_int = 0;
pub const WEECHAT_HDATA_CHAR: c_int = 1;
pub const WEECHAT_HDATA_INTEGER: c_int = 2;
pub const WEECHAT_HDATA_LONG: c_int = 3;
pub const WEECHAT_HDATA_STRING: c_int = 4;
pub const WEECHAT_HDATA_POINTER: c_int = 5;
pub const WEECHAT_HDATA_TIME: c_int = 6;
pub const WEECHAT_HDATA_HASHTABLE: c_int = 7;
pub const WEECHAT_HDATA_SHARED_STRING: c_int = 8;

/* flags for hdata lists */
pub const WEECHAT_HDATA_LIST_CHECK_POINTERS: c_int = 1;

/* buffer hotlist */
pub const WEECHAT_HOTLIST_LOW: *const u8 = b"0\0" as *const u8;
pub const WEECHAT_HOTLIST_MESSAGE: *const u8 = b"1\0" as *const u8;
pub const WEECHAT_HOTLIST_PRIVATE: *const u8 = b"2\0" as *const u8;
pub const WEECHAT_HOTLIST_HIGHLIGHT: *const u8 = b"3\0" as *const u8;
pub const WEECHAT_HOOK_PROCESS_RUNNING: c_int = -1;
pub const WEECHAT_HOOK_PROCESS_ERROR: c_int = -2;

/* connect status for connection hooked */
pub const WEECHAT_HOOK_CONNECT_OK: c_int = 0;
pub const WEECHAT_HOOK_CONNECT_ADDRESS_NOT_FOUND: c_int = 1;
pub const WEECHAT_HOOK_CONNECT_IP_ADDRESS_NOT_FOUND: c_int = 2;
pub const WEECHAT_HOOK_CONNECT_CONNECTION_REFUSED: c_int = 3;
pub const WEECHAT_HOOK_CONNECT_PROXY_ERROR: c_int = 4;
pub const WEECHAT_HOOK_CONNECT_LOCAL_HOSTNAME_ERROR: c_int = 5;
pub const WEECHAT_HOOK_CONNECT_GNUTLS_INIT_ERROR: c_int = 6;
pub const WEECHAT_HOOK_CONNECT_GNUTLS_HANDSHAKE_ERROR: c_int = 7;
pub const WEECHAT_HOOK_CONNECT_MEMORY_ERROR: c_int = 8;
pub const WEECHAT_HOOK_CONNECT_TIMEOUT: c_int = 9;
pub const WEECHAT_HOOK_CONNECT_SOCKET_ERROR: c_int = 10;

/* action for gnutls callback: verify or set certificate */
pub const WEECHAT_HOOK_CONNECT_GNUTLS_CB_VERIFY_CERT: c_int = 0;
pub const WEECHAT_HOOK_CONNECT_GNUTLS_CB_SET_CERT: c_int = 1;

/* type of data for signal hooked */
pub const WEECHAT_HOOK_SIGNAL_STRING: *const u8 = b"string\0" as *const u8;
pub const WEECHAT_HOOK_SIGNAL_INT: *const u8 = b"int\0" as *const u8;
pub const WEECHAT_HOOK_SIGNAL_POINTER: *const u8 = b"pointer\0" as *const u8;
