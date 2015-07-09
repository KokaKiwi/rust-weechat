use libc::*;

macro_rules! extern_c_struct(
    ($name:ident) => {
        #[repr(C)]
        pub struct $name;
    }
);

extern_c_struct!(t_config_file);
extern_c_struct!(t_config_option);
extern_c_struct!(t_config_section);
extern_c_struct!(t_gui_bar);
extern_c_struct!(t_gui_bar_item);
extern_c_struct!(t_gui_buffer);
extern_c_struct!(t_gui_completion);
extern_c_struct!(t_gui_nick);
extern_c_struct!(t_gui_nick_group);
extern_c_struct!(t_gui_window);
extern_c_struct!(t_hashtable);
extern_c_struct!(t_hashtable_item);
extern_c_struct!(t_hdata);
extern_c_struct!(t_hook);
extern_c_struct!(t_infolist);
extern_c_struct!(t_infolist_item);
extern_c_struct!(t_infolist_var);
extern_c_struct!(t_upgrade_file);
extern_c_struct!(t_weelist);
extern_c_struct!(t_weelist_item);


#[repr(C)]
pub struct t_weechat_plugin {
    pub filename: *mut c_char,
    pub handle: *mut c_void,
    pub name: *mut c_char,
    pub description: *mut c_char,
    pub author: *mut c_char,
    pub version: *mut c_char,
    pub license: *mut c_char,
    pub charset: *mut c_char,
    pub priority: c_int,
    pub initialized: c_int,
    pub debug: c_int,

    pub prev_plugin: *mut t_weechat_plugin,
    pub next_plugin: *mut t_weechat_plugin,

    /*
     * plugin functions (API)
     */

    /* plugins */
    pub plugin_get_name: extern "C" fn(plugin: *mut t_weechat_plugin) -> *const c_char,

    /* strings */
    pub charset_set: extern "C" fn(plugin: *mut t_weechat_plugin,
                                   charset: *const c_char),
    pub iconv_to_internal: extern "C" fn(charset: *const c_char,
                                         string: *const c_char)
                               -> *mut c_char,
    pub iconv_from_internal: extern "C" fn(charset: *const c_char,
                                           string: *const c_char)
                                 -> *mut c_char,
    pub gettext: extern "C" fn(string: *const c_char) -> *const c_char,
    pub ngettext: extern "C" fn(single: *const c_char, plural: *const c_char,
                                count: c_int) -> *const c_char,
    pub strndup: extern "C" fn(string: *const c_char, length: c_int)
                     -> *mut c_char,
    pub string_tolower: extern "C" fn(string: *mut c_char),
    pub string_toupper: extern "C" fn(string: *mut c_char),
    pub strcasecmp: extern "C" fn(string1: *const c_char,
                                  string2: *const c_char) -> c_int,
    pub strcasecmp_range: extern "C" fn(string1: *const c_char,
                                        string2: *const c_char, range: c_int)
                              -> c_int,
    pub strncasecmp: extern "C" fn(string1: *const c_char,
                                   string2: *const c_char, max: c_int)
                         -> c_int,
    pub strncasecmp_range: extern "C" fn(string1: *const c_char,
                                         string2: *const c_char, max: c_int,
                                         range: c_int) -> c_int,
    pub strcmp_ignore_chars: extern "C" fn(string1: *const c_char,
                                           string2: *const c_char,
                                           chars_ignored: *const c_char,
                                           case_sensitive: c_int) -> c_int,
    pub strcasestr: extern "C" fn(string: *const c_char,
                                  search: *const c_char) -> *const c_char,
    pub strlen_screen: extern "C" fn(string: *const c_char) -> c_int,
    pub string_match: extern "C" fn(string: *const c_char,
                                    mask: *const c_char,
                                    case_sensitive: c_int) -> c_int,
    pub string_replace: extern "C" fn(string: *const c_char,
                                      search: *const c_char,
                                      replace: *const c_char) -> *mut c_char,
    pub string_expand_home: extern "C" fn(path: *const c_char) -> *mut c_char,
    pub string_eval_path_home: extern "C" fn(path: *const c_char,
                                             pointers: *mut t_hashtable,
                                             extra_vars: *mut t_hashtable,
                                             options: *mut t_hashtable)
                                   -> *mut c_char,
    pub string_remove_quotes: extern "C" fn(string: *const c_char,
                                            quotes: *const c_char)
                                  -> *mut c_char,
    pub string_strip: extern "C" fn(string: *const c_char, left: c_int,
                                    right: c_int, chars: *const c_char)
                          -> *mut c_char,
    pub string_convert_escaped_chars: extern "C" fn(string: *const c_char)
                                          -> *mut c_char,
    pub string_mask_to_regex: extern "C" fn(mask: *const c_char)
                                  -> *mut c_char,
    pub string_regex_flags: extern "C" fn(regex: *const c_char,
                                          default_flags: c_int,
                                          flags: *mut c_int) -> *const c_char,
    pub string_regcomp: extern "C" fn(preg: *mut c_void, regex: *const c_char,
                                      default_flags: c_int) -> c_int,
    pub string_has_highlight: extern "C" fn(string: *const c_char,
                                            highlight_words: *const c_char)
                                  -> c_int,
    pub string_has_highlight_regex: extern "C" fn(string: *const c_char,
                                                  regex: *const c_char)
                                        -> c_int,
    pub string_replace_regex: extern "C" fn(string: *const c_char,
                                            regex: *mut c_void,
                                            replace: *const c_char,
                                            reference_char: c_char,
                                            callback:
                                                Option<extern "C" fn(data: *mut c_void,
                                                                     text: *const c_char)
                                                           -> *mut c_char>,
                                            callback_data: *mut c_void)
                                  -> *mut c_char,
    pub string_split: extern "C" fn(string: *const c_char,
                                    separators: *const c_char,
                                    keep_eol: c_int, num_items_max: c_int,
                                    num_items: *mut c_int)
                          -> *mut *mut c_char,
    pub string_split_shell: extern "C" fn(string: *const c_char,
                                          num_items: *mut c_int)
                                -> *mut *mut c_char,
    pub string_free_split: extern "C" fn(split_string: *mut *mut c_char),
    pub string_build_with_split_string: extern "C" fn(split_string: *mut *const c_char,
                                                      separator: *const c_char)
                                            -> *mut c_char,
    pub string_split_command: extern "C" fn(command: *const c_char,
                                            separator: c_char)
                                  -> *mut *mut c_char,
    pub string_free_split_command: extern "C" fn(split_command:
                                                     *mut *mut c_char),
    pub string_format_size: extern "C" fn(size: c_ulonglong) -> *mut c_char,
    pub string_remove_color: extern "C" fn(string: *const c_char,
                                           replacement: *const c_char)
                                 -> *mut c_char,
    pub string_encode_base64: extern "C" fn(from: *const c_char,
                                            length: c_int, to: *mut c_char),
    pub string_decode_base64: extern "C" fn(from: *const c_char,
                                            to: *mut c_char) -> c_int,
    pub string_is_command_char: extern "C" fn(string: *const c_char) -> c_int,
    pub string_input_for_buffer: extern "C" fn(string: *const c_char)
                                     -> *const c_char,
    pub string_eval_expression: extern "C" fn(expr: *const c_char,
                                              pointers: *mut t_hashtable,
                                              extra_vars: *mut t_hashtable,
                                              options: *mut t_hashtable)
                                    -> *mut c_char,

    /* UTF-8 strings */
    pub utf8_has_8bits: extern "C" fn(string: *const c_char) -> c_int,
    pub utf8_is_valid: extern "C" fn(string: *const c_char,
                                     error: *mut *mut c_char) -> c_int,
    pub utf8_normalize: extern "C" fn(string: *mut c_char,
                                      replacement: c_char),
    pub utf8_prev_char: extern "C" fn(string_start: *const c_char,
                                      string: *const c_char) -> *const c_char,
    pub utf8_next_char: extern "C" fn(string: *const c_char) -> *const c_char,
    pub utf8_char_int: extern "C" fn(string: *const c_char) -> c_int,
    pub utf8_char_size: extern "C" fn(string: *const c_char) -> c_int,
    pub utf8_strlen: extern "C" fn(string: *const c_char) -> c_int,
    pub utf8_strnlen: extern "C" fn(string: *const c_char, bytes: c_int)
                          -> c_int,
    pub utf8_strlen_screen: extern "C" fn(string: *const c_char) -> c_int,
    pub utf8_charcmp: extern "C" fn(string1: *const c_char,
                                    string2: *const c_char) -> c_int,
    pub utf8_charcasecmp: extern "C" fn(string1: *const c_char,
                                        string2: *const c_char) -> c_int,
    pub utf8_char_size_screen: extern "C" fn(string: *const c_char) -> c_int,
    pub utf8_add_offset: extern "C" fn(string: *const c_char, offset: c_int)
                             -> *const c_char,
    pub utf8_real_pos: extern "C" fn(string: *const c_char, pos: c_int)
                           -> c_int,
    pub utf8_pos: extern "C" fn(string: *const c_char, real_pos: c_int)
                      -> c_int,
    pub utf8_strndup: extern "C" fn(string: *const c_char, length: c_int)
                          -> *mut c_char,

    /* directories / files */
    pub mkdir_home: extern "C" fn(directory: *const c_char, mode: c_int)
                        -> c_int,
    pub mkdir: extern "C" fn(directory: *const c_char, mode: c_int) -> c_int,
    pub mkdir_parents: extern "C" fn(directory: *const c_char, mode: c_int)
                           -> c_int,
    pub exec_on_files: extern "C" fn(directory: *const c_char,
                                     hidden_files: c_int, data: *mut c_void,
                                     callback:
                                         Option<extern "C" fn(data: *mut c_void,
                                                              filename: *const c_char)>),
    pub file_get_content: extern "C" fn(filename: *const c_char)
                              -> *mut c_char,

    /* util */
    pub util_timeval_cmp: extern "C" fn(tv1: *mut timeval, tv2: *mut timeval)
                              -> c_int,
    pub util_timeval_diff: extern "C" fn(tv1: *mut timeval, tv2: *mut timeval)
                               -> c_longlong,
    pub util_timeval_add: extern "C" fn(tv: *mut timeval,
                                        interval: c_longlong),
    pub util_get_time_string: extern "C" fn(date: *const time_t)
                                  -> *const c_char,
    pub util_version_number: extern "C" fn(version: *const c_char) -> c_int,

    /* sorted lists */
    pub list_new: extern "C" fn() -> *mut t_weelist,
    pub list_add: extern "C" fn(weelist: *mut t_weelist, data: *const c_char,
                                _where: *const c_char, user_data: *mut c_void)
                      -> *mut t_weelist_item,
    pub list_search: extern "C" fn(weelist: *mut t_weelist,
                                   data: *const c_char)
                         -> *mut t_weelist_item,
    pub list_search_pos: extern "C" fn(weelist: *mut t_weelist,
                                       data: *const c_char) -> c_int,
    pub list_casesearch: extern "C" fn(weelist: *mut t_weelist,
                                       data: *const c_char)
                             -> *mut t_weelist_item,
    pub list_casesearch_pos: extern "C" fn(weelist: *mut t_weelist,
                                           data: *const c_char) -> c_int,
    pub list_get: extern "C" fn(weelist: *mut t_weelist, position: c_int)
                      -> *mut t_weelist_item,
    pub list_set: extern "C" fn(item: *mut t_weelist_item,
                                value: *const c_char),
    pub list_next: extern "C" fn(item: *mut t_weelist_item)
                       -> *mut t_weelist_item,
    pub list_prev: extern "C" fn(item: *mut t_weelist_item)
                       -> *mut t_weelist_item,
    pub list_string: extern "C" fn(item: *mut t_weelist_item)
                         -> *const c_char,
    pub list_size: extern "C" fn(weelist: *mut t_weelist) -> c_int,
    pub list_remove: extern "C" fn(weelist: *mut t_weelist,
                                   item: *mut t_weelist_item),
    pub list_remove_all: extern "C" fn(weelist: *mut t_weelist),
    pub list_free: extern "C" fn(weelist: *mut t_weelist),

    /* hash tables */
    pub hashtable_new: extern "C" fn(size: c_int, type_keys: *const c_char,
                                     type_values: *const c_char,
                                     callback_hash_key:
                                         Option<extern "C" fn(hashtable: *mut t_hashtable,
                                                              key: *const c_void)
                                                    -> c_ulonglong>,
                                     callback_keycmp:
                                         Option<extern "C" fn(hashtable: *mut t_hashtable,
                                                              key1: *const c_void,
                                                              key2: *const c_void)
                                                    -> c_int>)
                           -> *mut t_hashtable,
    pub hashtable_set_with_size: extern "C" fn(hashtable: *mut t_hashtable,
                                               key: *const c_void,
                                               key_size: c_int,
                                               value: *const c_void,
                                               value_size: c_int)
                                     -> *mut t_hashtable_item,
    pub hashtable_set: extern "C" fn(hashtable: *mut t_hashtable,
                                     key: *const c_void, value: *const c_void)
                           -> *mut t_hashtable_item,
    pub hashtable_get: extern "C" fn(hashtable: *mut t_hashtable,
                                     key: *const c_void) -> *mut c_void,
    pub hashtable_has_key: extern "C" fn(hashtable: *mut t_hashtable,
                                         key: *const c_void) -> c_int,
    pub hashtable_map: extern "C" fn(hashtable: *mut t_hashtable,
                                     callback_map:
                                         Option<extern "C" fn(data: *mut c_void,
                                                              hashtable: *mut t_hashtable,
                                                              key: *const c_void,
                                                              value: *const c_void)>,
                                     callback_map_data: *mut c_void),
    pub hashtable_map_string: extern "C" fn(hashtable: *mut t_hashtable,
                                            callback_map:
                                                Option<extern "C" fn(data: *mut c_void,
                                                                     hashtable: *mut t_hashtable,
                                                                     key: *const c_char,
                                                                     value: *const c_char)>,
                                            callback_map_data: *mut c_void),
    pub hashtable_dup: extern "C" fn(hashtable: *mut t_hashtable)
                           -> *mut t_hashtable,
    pub hashtable_get_integer: extern "C" fn(hashtable: *mut t_hashtable,
                                             property: *const c_char)
                                   -> c_int,
    pub hashtable_get_string: extern "C" fn(hashtable: *mut t_hashtable,
                                            property: *const c_char)
                                  -> *const c_char,
    pub hashtable_set_pointer: extern "C" fn(hashtable: *mut t_hashtable,
                                             property: *const c_char,
                                             pointer: *mut c_void),
    pub hashtable_add_to_infolist: extern "C" fn(hashtable: *mut t_hashtable,
                                                 infolist_item: *mut t_infolist_item,
                                                 prefix: *const c_char)
                                       -> c_int,
    pub hashtable_remove: extern "C" fn(hashtable: *mut t_hashtable,
                                        key: *const c_void),
    pub hashtable_remove_all: extern "C" fn(hashtable: *mut t_hashtable),
    pub hashtable_free: extern "C" fn(hashtable: *mut t_hashtable),

    /* config files */
    pub config_new: extern "C" fn(plugin: *mut t_weechat_plugin,
                                  name: *const c_char,
                                  callback_reload:
                                      Option<extern "C" fn(data: *mut c_void,
                                                           config_file: *mut t_config_file)
                                                 -> c_int>,
                                  callback_reload_data: *mut c_void)
                        -> *mut t_config_file,
    pub config_new_section: extern "C" fn(config_file: *mut t_config_file,
                                          name: *const c_char,
                                          user_can_add_options: c_int,
                                          user_can_delete_options: c_int,
                                          callback_read:
                                              Option<extern "C" fn(data: *mut c_void,
                                                                   config_file: *mut t_config_file,
                                                                   section: *mut t_config_section,
                                                                   option_name: *const c_char,
                                                                   value: *const c_char)
                                                         -> c_int>,
                                          callback_read_data: *mut c_void,
                                          callback_write:
                                              Option<extern "C" fn(data: *mut c_void,
                                                                   config_file: *mut t_config_file,
                                                                   section_name: *const c_char)
                                                         -> c_int>,
                                          callback_write_data: *mut c_void,
                                          callback_write_default:
                                              Option<extern "C" fn(data: *mut c_void,
                                                                   config_file: *mut t_config_file,
                                                                   section_name: *const c_char)
                                                         -> c_int>,
                                          callback_write_default_data: *mut c_void,
                                          callback_create_option:
                                              Option<extern "C" fn(data: *mut c_void,
                                                                   config_file: *mut t_config_file,
                                                                   section: *mut t_config_section,
                                                                   option_name: *const c_char,
                                                                   value: *const c_char)
                                                         -> c_int>,
                                          callback_create_option_data: *mut c_void,
                                          callback_delete_option:
                                              Option<extern "C" fn(data: *mut c_void,
                                                                   config_file: *mut t_config_file,
                                                                   section: *mut t_config_section,
                                                                   option: *mut t_config_option)
                                                         -> c_int>,
                                          callback_delete_option_data: *mut c_void)
                                -> *mut t_config_section,
    pub config_search_section: extern "C" fn(config_file: *mut t_config_file,
                                             section_name: *const c_char)
                                   -> *mut t_config_section,
    pub config_new_option: extern "C" fn(config_file: *mut t_config_file,
                                         section: *mut t_config_section,
                                         name: *const c_char,
                                         _type: *const c_char,
                                         description: *const c_char,
                                         string_values: *const c_char,
                                         min: c_int, max: c_int,
                                         default_value: *const c_char,
                                         value: *const c_char,
                                         null_value_allowed: c_int,
                                         callback_check_value:
                                             Option<extern "C" fn(data: *mut c_void,
                                                                  option: *mut t_config_option,
                                                                  value: *const c_char)
                                                        -> c_int>,
                                         callback_check_value_data: *mut c_void,
                                         callback_change:
                                             Option<extern "C" fn(data: *mut c_void,
                                                                  option: *mut t_config_option)>,
                                         callback_change_data: *mut c_void,
                                         callback_delete:
                                             Option<extern "C" fn(data: *mut c_void,
                                                                  option: *mut t_config_option)>,
                                         callback_delete_data: *mut c_void)
                               -> *mut t_config_option,
    pub config_search_option: extern "C" fn(config_file: *mut t_config_file,
                                            section: *mut t_config_section,
                                            option_name: *const c_char)
                                  -> *mut t_config_option,
    pub config_search_section_option: extern "C" fn(config_file: *mut t_config_file,
                                                    section: *mut t_config_section,
                                                    option_name: *const c_char,
                                                    section_found: *mut *mut t_config_section,
                                                    option_found: *mut *mut t_config_option),
    pub config_search_with_string: extern "C" fn(option_name: *const c_char,
                                                 config_file: *mut *mut t_config_file,
                                                 section: *mut *mut t_config_section,
                                                 option: *mut *mut t_config_option,
                                                 pos_option_name: *mut *mut c_char),
    pub config_string_to_boolean: extern "C" fn(text: *const c_char) -> c_int,
    pub config_option_reset: extern "C" fn(option: *mut t_config_option,
                                           run_callback: c_int) -> c_int,
    pub config_option_set: extern "C" fn(option: *mut t_config_option,
                                         value: *const c_char,
                                         run_callback: c_int) -> c_int,
    pub config_option_set_null: extern "C" fn(option: *mut t_config_option,
                                              run_callback: c_int) -> c_int,
    pub config_option_unset: extern "C" fn(option: *mut t_config_option)
                                 -> c_int,
    pub config_option_rename: extern "C" fn(option: *mut t_config_option,
                                            new_name: *const c_char),
    pub config_option_get_pointer: extern "C" fn(option: *mut t_config_option,
                                                 property: *const c_char)
                                       -> *mut c_void,
    pub config_option_is_null: extern "C" fn(option: *mut t_config_option)
                                   -> c_int,
    pub config_option_default_is_null: extern "C" fn(option: *mut t_config_option)
                                           -> c_int,
    pub config_boolean: extern "C" fn(option: *mut t_config_option) -> c_int,
    pub config_boolean_default: extern "C" fn(option: *mut t_config_option)
                                    -> c_int,
    pub config_integer: extern "C" fn(option: *mut t_config_option) -> c_int,
    pub config_integer_default: extern "C" fn(option: *mut t_config_option)
                                    -> c_int,
    pub config_string: extern "C" fn(option: *mut t_config_option)
                           -> *const c_char,
    pub config_string_default: extern "C" fn(option: *mut t_config_option)
                                   -> *const c_char,
    pub config_color: extern "C" fn(option: *mut t_config_option)
                          -> *const c_char,
    pub config_color_default: extern "C" fn(option: *mut t_config_option)
                                  -> *const c_char,
    pub config_write_option: extern "C" fn(config_file: *mut t_config_file,
                                           option: *mut t_config_option)
                                 -> c_int,
    pub config_write_line: extern "C" fn(config_file: *mut t_config_file,
                                         option_name: *const c_char,
                                         value: *const c_char, ...) -> c_int,
    pub config_write: extern "C" fn(config_file: *mut t_config_file) -> c_int,
    pub config_read: extern "C" fn(config_file: *mut t_config_file) -> c_int,
    pub config_reload: extern "C" fn(config_file: *mut t_config_file)
                           -> c_int,
    pub config_option_free: extern "C" fn(option: *mut t_config_option),
    pub config_section_free_options: extern "C" fn(section: *mut t_config_section),
    pub config_section_free: extern "C" fn(section: *mut t_config_section),
    pub config_free: extern "C" fn(config_file: *mut t_config_file),
    pub config_get: extern "C" fn(option_name: *const c_char)
                        -> *mut t_config_option,
    pub config_get_plugin: extern "C" fn(plugin: *mut t_weechat_plugin,
                                         option_name: *const c_char)
                               -> *const c_char,
    pub config_is_set_plugin: extern "C" fn(plugin: *mut t_weechat_plugin,
                                            option_name: *const c_char)
                                  -> c_int,
    pub config_set_plugin: extern "C" fn(plugin: *mut t_weechat_plugin,
                                         option_name: *const c_char,
                                         value: *const c_char) -> c_int,
    pub config_set_desc_plugin: extern "C" fn(plugin: *mut t_weechat_plugin,
                                              option_name: *const c_char,
                                              description: *const c_char),
    pub config_unset_plugin: extern "C" fn(plugin: *mut t_weechat_plugin,
                                           option_name: *const c_char)
                                 -> c_int,

    /* key bindings */
    pub key_bind: extern "C" fn(context: *const c_char,
                                keys: *mut t_hashtable) -> c_int,
    pub key_unbind: extern "C" fn(context: *const c_char, key: *const c_char)
                        -> c_int,

    /* display */
    pub prefix: extern "C" fn(prefix: *const c_char) -> *const c_char,
    pub color: extern "C" fn(color_name: *const c_char) -> *const c_char,
    pub printf_date_tags: extern "C" fn(buffer: *mut t_gui_buffer,
                                        date: time_t, tags: *const c_char,
                                        message: *const c_char, ...),
    pub printf_y: extern "C" fn(buffer: *mut t_gui_buffer, y: c_int,
                                message: *const c_char, ...),
    pub log_printf: extern "C" fn(message: *const c_char, ...),

    /* hooks */
    pub hook_command: extern "C" fn(plugin: *mut t_weechat_plugin,
                                    command: *const c_char,
                                    description: *const c_char,
                                    args: *const c_char,
                                    args_description: *const c_char,
                                    completion: *const c_char,
                                    callback:
                                        Option<extern "C" fn(data: *mut c_void,
                                                             buffer: *mut t_gui_buffer,
                                                             argc: c_int,
                                                             argv: *mut *mut c_char,
                                                             argv_eol: *mut *mut c_char)
                                                   -> c_int>,
                                    callback_data: *mut c_void)
                          -> *mut t_hook,
    pub hook_command_run: extern "C" fn(plugin: *mut t_weechat_plugin,
                                        command: *const c_char,
                                        callback:
                                            Option<extern "C" fn(data: *mut c_void,
                                                                 buffer: *mut t_gui_buffer,
                                                                 command: *const c_char)
                                                       -> c_int>,
                                        callback_data: *mut c_void)
                              -> *mut t_hook,
    pub hook_timer: extern "C" fn(plugin: *mut t_weechat_plugin,
                                  interval: c_long, align_second: c_int,
                                  max_calls: c_int,
                                  callback:
                                      Option<extern "C" fn(data: *mut c_void,
                                                           remaining_calls: c_int)
                                                 -> c_int>,
                                  callback_data: *mut c_void) -> *mut t_hook,
    pub hook_fd: extern "C" fn(plugin: *mut t_weechat_plugin, fd: c_int,
                               flag_read: c_int, flag_write: c_int,
                               flag_exception: c_int,
                               callback:
                                   Option<extern "C" fn(data: *mut c_void,
                                                        fd: c_int) -> c_int>,
                               callback_data: *mut c_void) -> *mut t_hook,
    pub hook_process: extern "C" fn(plugin: *mut t_weechat_plugin,
                                    command: *const c_char, timeout: c_int,
                                    callback:
                                        Option<extern "C" fn(data: *mut c_void,
                                                             command: *const c_char,
                                                             return_code: c_int,
                                                             out: *const c_char,
                                                             err: *const c_char)
                                                   -> c_int>,
                                    callback_data: *mut c_void)
                          -> *mut t_hook,
    pub hook_process_hashtable: extern "C" fn(plugin: *mut t_weechat_plugin,
                                              command: *const c_char,
                                              options: *mut t_hashtable,
                                              timeout: c_int,
                                              callback:
                                                  Option<extern "C" fn(data: *mut c_void,
                                                                       command: *const c_char,
                                                                       return_code: c_int,
                                                                       out: *const c_char,
                                                                       err: *const c_char)
                                                             -> c_int>,
                                              callback_data: *mut c_void)
                                    -> *mut t_hook,
    pub hook_connect: extern "C" fn(plugin: *mut t_weechat_plugin,
                                    proxy: *const c_char,
                                    address: *const c_char, port: c_int,
                                    ipv6: c_int, retry: c_int,
                                    gnutls_sess: *mut c_void,
                                    gnutls_cb: *mut c_void,
                                    gnutls_dhkey_size: c_int,
                                    gnutls_priorities: *const c_char,
                                    local_hostname: *const c_char,
                                    callback:
                                        Option<extern "C" fn(data: *mut c_void,
                                                             status: c_int,
                                                             gnutls_rc: c_int,
                                                             sock: c_int,
                                                             error: *const c_char,
                                                             ip_address: *const c_char)
                                                   -> c_int>,
                                    callback_data: *mut c_void)
                          -> *mut t_hook,
    pub hook_print: extern "C" fn(plugin: *mut t_weechat_plugin,
                                  buffer: *mut t_gui_buffer,
                                  tags: *const c_char, message: *const c_char,
                                  strip_colors: c_int,
                                  callback:
                                      Option<extern "C" fn(data: *mut c_void,
                                                           buffer: *mut t_gui_buffer,
                                                           date: time_t,
                                                           tags_count: c_int,
                                                           tags: *mut *const c_char,
                                                           displayed: c_int,
                                                           highlight: c_int,
                                                           prefix: *const c_char,
                                                           message: *const c_char)
                                                 -> c_int>,
                                  callback_data: *mut c_void) -> *mut t_hook,
    pub hook_signal: extern "C" fn(plugin: *mut t_weechat_plugin,
                                   signal: *const c_char,
                                   callback:
                                       Option<extern "C" fn(data: *mut c_void,
                                                            signal: *const c_char,
                                                            type_data: *const c_char,
                                                            signal_data: *mut c_void)
                                                  -> c_int>,
                                   callback_data: *mut c_void) -> *mut t_hook,
    pub hook_signal_send: extern "C" fn(signal: *const c_char,
                                        type_data: *const c_char,
                                        signal_data: *mut c_void) -> c_int,
    pub hook_hsignal: extern "C" fn(plugin: *mut t_weechat_plugin,
                                    signal: *const c_char,
                                    callback:
                                        Option<extern "C" fn(data: *mut c_void,
                                                             signal: *const c_char,
                                                             hashtable: *mut t_hashtable)
                                                   -> c_int>,
                                    callback_data: *mut c_void)
                          -> *mut t_hook,
    pub hook_hsignal_send: extern "C" fn(signal: *const c_char,
                                         hashtable: *mut t_hashtable)
                               -> c_int,
    pub hook_config: extern "C" fn(plugin: *mut t_weechat_plugin,
                                   option: *const c_char,
                                   callback:
                                       Option<extern "C" fn(data: *mut c_void,
                                                            option: *const c_char,
                                                            value: *const c_char)
                                                  -> c_int>,
                                   callback_data: *mut c_void) -> *mut t_hook,
    pub hook_completion: extern "C" fn(plugin: *mut t_weechat_plugin,
                                       completion_item: *const c_char,
                                       description: *const c_char,
                                       callback:
                                           Option<extern "C" fn(data: *mut c_void,
                                                                completion_item: *const c_char,
                                                                buffer: *mut t_gui_buffer,
                                                                completion: *mut t_gui_completion)
                                                      -> c_int>,
                                       callback_data: *mut c_void)
                             -> *mut t_hook,
    pub hook_completion_get_string: extern "C" fn(completion: *mut t_gui_completion,
                                                  property: *const c_char)
                                        -> *const c_char,
    pub hook_completion_list_add: extern "C" fn(completion: *mut t_gui_completion,
                                                word: *const c_char,
                                                nick_completion: c_int,
                                                _where: *const c_char),
    pub hook_modifier: extern "C" fn(plugin: *mut t_weechat_plugin,
                                     modifier: *const c_char,
                                     callback:
                                         Option<extern "C" fn(data: *mut c_void,
                                                              modifier: *const c_char,
                                                              modifier_data: *const c_char,
                                                              string: *const c_char)
                                                    -> *mut c_char>,
                                     callback_data: *mut c_void)
                           -> *mut t_hook,
    pub hook_modifier_exec: extern "C" fn(plugin: *mut t_weechat_plugin,
                                          modifier: *const c_char,
                                          modifier_data: *const c_char,
                                          string: *const c_char)
                                -> *mut c_char,
    pub hook_info: extern "C" fn(plugin: *mut t_weechat_plugin,
                                 info_name: *const c_char,
                                 description: *const c_char,
                                 args_description: *const c_char,
                                 callback:
                                     Option<extern "C" fn(data: *mut c_void,
                                                          info_name: *const c_char,
                                                          arguments: *const c_char)
                                                -> *const c_char>,
                                 callback_data: *mut c_void) -> *mut t_hook,
    pub hook_info_hashtable: extern "C" fn(plugin: *mut t_weechat_plugin,
                                           info_name: *const c_char,
                                           description: *const c_char,
                                           args_description: *const c_char,
                                           output_description: *const c_char,
                                           callback:
                                               Option<extern "C" fn(data: *mut c_void,
                                                                    info_name: *const c_char,
                                                                    hashtable: *mut t_hashtable)
                                                          ->
                                                              *mut t_hashtable>,
                                           callback_data: *mut c_void)
                                 -> *mut t_hook,
    pub hook_infolist: extern "C" fn(plugin: *mut t_weechat_plugin,
                                     infolist_name: *const c_char,
                                     description: *const c_char,
                                     pointer_description: *const c_char,
                                     args_description: *const c_char,
                                     callback:
                                         Option<extern "C" fn(data: *mut c_void,
                                                              infolist_name: *const c_char,
                                                              pointer: *mut c_void,
                                                              arguments: *const c_char)
                                                    -> *mut t_infolist>,
                                     callback_data: *mut c_void)
                           -> *mut t_hook,
    pub hook_hdata: extern "C" fn(plugin: *mut t_weechat_plugin,
                                  hdata_name: *const c_char,
                                  description: *const c_char,
                                  callback:
                                      Option<extern "C" fn(data: *mut c_void,
                                                           hdata_name: *const c_char)
                                                 -> *mut t_hdata>,
                                  callback_data: *mut c_void) -> *mut t_hook,
    pub hook_focus: extern "C" fn(plugin: *mut t_weechat_plugin,
                                  area: *const c_char,
                                  callback:
                                      Option<extern "C" fn(data: *mut c_void,
                                                           info: *mut t_hashtable)
                                                 -> *mut t_hashtable>,
                                  callback_data: *mut c_void) -> *mut t_hook,
    pub hook_set: extern "C" fn(hook: *mut t_hook, property: *const c_char,
                                value: *const c_char),
    pub unhook: extern "C" fn(hook: *mut t_hook),
    pub unhook_all: extern "C" fn(plugin: *mut t_weechat_plugin),

    /* buffers */
    pub buffer_new: extern "C" fn(plugin: *mut t_weechat_plugin,
                                  name: *const c_char,
                                  input_callback:
                                      Option<extern "C" fn(data: *mut c_void,
                                                           buffer: *mut t_gui_buffer,
                                                           input_data: *const c_char)
                                                 -> c_int>,
                                  input_callback_data: *mut c_void,
                                  close_callback:
                                      Option<extern "C" fn(data: *mut c_void,
                                                           buffer: *mut t_gui_buffer)
                                                 -> c_int>,
                                  close_callback_data: *mut c_void)
                        -> *mut t_gui_buffer,
    pub buffer_search: extern "C" fn(plugin: *const c_char,
                                     name: *const c_char)
                           -> *mut t_gui_buffer,
    pub buffer_search_main: extern "C" fn() -> *mut t_gui_buffer,
    pub buffer_clear: extern "C" fn(buffer: *mut t_gui_buffer),
    pub buffer_close: extern "C" fn(buffer: *mut t_gui_buffer),
    pub buffer_merge: extern "C" fn(buffer: *mut t_gui_buffer,
                                    target_buffer: *mut t_gui_buffer),
    pub buffer_unmerge: extern "C" fn(buffer: *mut t_gui_buffer,
                                      number: c_int),
    pub buffer_get_integer: extern "C" fn(buffer: *mut t_gui_buffer,
                                          property: *const c_char) -> c_int,
    pub buffer_get_string: extern "C" fn(buffer: *mut t_gui_buffer,
                                         property: *const c_char)
                               -> *const c_char,
    pub buffer_get_pointer: extern "C" fn(buffer: *mut t_gui_buffer,
                                          property: *const c_char)
                                -> *mut c_void,
    pub buffer_set: extern "C" fn(buffer: *mut t_gui_buffer,
                                  property: *const c_char,
                                  value: *const c_char),
    pub buffer_set_pointer: extern "C" fn(buffer: *mut t_gui_buffer,
                                          property: *const c_char,
                                          pointer: *mut c_void),
    pub buffer_string_replace_local_var: extern "C" fn(buffer: *mut t_gui_buffer,
                                                       string: *const c_char)
                                             -> *mut c_char,
    pub buffer_match_list: extern "C" fn(buffer: *mut t_gui_buffer,
                                         string: *const c_char) -> c_int,

    /* windows */
    pub window_search_with_buffer: extern "C" fn(buffer: *mut t_gui_buffer)
                                       -> *mut t_gui_window,
    pub window_get_integer: extern "C" fn(window: *mut t_gui_window,
                                          property: *const c_char) -> c_int,
    pub window_get_string: extern "C" fn(window: *mut t_gui_window,
                                         property: *const c_char)
                               -> *const c_char,
    pub window_get_pointer: extern "C" fn(window: *mut t_gui_window,
                                          property: *const c_char)
                                -> *mut c_void,
    pub window_set_title: extern "C" fn(title: *const c_char),

    /* nicklist */
    pub nicklist_add_group: extern "C" fn(buffer: *mut t_gui_buffer,
                                          parent_group: *mut t_gui_nick_group,
                                          name: *const c_char,
                                          color: *const c_char,
                                          visible: c_int)
                                -> *mut t_gui_nick_group,
    pub nicklist_search_group: extern "C" fn(buffer: *mut t_gui_buffer,
                                             from_group: *mut t_gui_nick_group,
                                             name: *const c_char)
                                   -> *mut t_gui_nick_group,
    pub nicklist_add_nick: extern "C" fn(buffer: *mut t_gui_buffer,
                                         group: *mut t_gui_nick_group,
                                         name: *const c_char,
                                         color: *const c_char,
                                         prefix: *const c_char,
                                         prefix_color: *const c_char,
                                         visible: c_int) -> *mut t_gui_nick,
    pub nicklist_search_nick: extern "C" fn(buffer: *mut t_gui_buffer,
                                            from_group: *mut t_gui_nick_group,
                                            name: *const c_char)
                                  -> *mut t_gui_nick,
    pub nicklist_remove_group: extern "C" fn(buffer: *mut t_gui_buffer,
                                             group: *mut t_gui_nick_group),
    pub nicklist_remove_nick: extern "C" fn(buffer: *mut t_gui_buffer,
                                            nick: *mut t_gui_nick),
    pub nicklist_remove_all: extern "C" fn(buffer: *mut t_gui_buffer),
    pub nicklist_get_next_item: extern "C" fn(buffer: *mut t_gui_buffer,
                                              group: *mut *mut t_gui_nick_group,
                                              nick: *mut *mut t_gui_nick),
    pub nicklist_group_get_integer: extern "C" fn(buffer: *mut t_gui_buffer,
                                                  group: *mut t_gui_nick_group,
                                                  property: *const c_char)
                                        -> c_int,
    pub nicklist_group_get_string: extern "C" fn(buffer: *mut t_gui_buffer,
                                                 group: *mut t_gui_nick_group,
                                                 property: *const c_char)
                                       -> *const c_char,
    pub nicklist_group_get_pointer: extern "C" fn(buffer: *mut t_gui_buffer,
                                                  group: *mut t_gui_nick_group,
                                                  property: *const c_char)
                                        -> *mut c_void,
    pub nicklist_group_set: extern "C" fn(buffer: *mut t_gui_buffer,
                                          group: *mut t_gui_nick_group,
                                          property: *const c_char,
                                          value: *const c_char),
    pub nicklist_nick_get_integer: extern "C" fn(buffer: *mut t_gui_buffer,
                                                 nick: *mut t_gui_nick,
                                                 property: *const c_char)
                                       -> c_int,
    pub nicklist_nick_get_string: extern "C" fn(buffer: *mut t_gui_buffer,
                                                nick: *mut t_gui_nick,
                                                property: *const c_char)
                                      -> *const c_char,
    pub nicklist_nick_get_pointer: extern "C" fn(buffer: *mut t_gui_buffer,
                                                 nick: *mut t_gui_nick,
                                                 property: *const c_char)
                                       -> *mut c_void,
    pub nicklist_nick_set: extern "C" fn(buffer: *mut t_gui_buffer,
                                         nick: *mut t_gui_nick,
                                         property: *const c_char,
                                         value: *const c_char),

    /* bars */
    pub bar_item_search: extern "C" fn(name: *const c_char)
                             -> *mut t_gui_bar_item,
    pub bar_item_new: extern "C" fn(plugin: *mut t_weechat_plugin,
                                    name: *const c_char,
                                    build_callback:
                                        Option<extern "C" fn(data: *mut c_void,
                                                             item: *mut t_gui_bar_item,
                                                             window: *mut t_gui_window,
                                                             buffer: *mut t_gui_buffer,
                                                             extra_info: *mut t_hashtable)
                                                   -> *mut c_char>,
                                    build_callback_data: *mut c_void)
                          -> *mut t_gui_bar_item,
    pub bar_item_update: extern "C" fn(name: *const c_char),
    pub bar_item_remove: extern "C" fn(item: *mut t_gui_bar_item),
    pub bar_search: extern "C" fn(name: *const c_char) -> *mut t_gui_bar,
    pub bar_new: extern "C" fn(name: *const c_char,
                               hidden: *const c_char,
                               priority: *const c_char,
                               _type: *const c_char,
                               condition: *const c_char,
                               position: *const c_char,
                               filling_top_bottom: *const c_char,
                               filling_left_right: *const c_char,
                               size: *const c_char,
                               size_max: *const c_char,
                               color_fg: *const c_char,
                               color_delim: *const c_char,
                               color_bg: *const c_char,
                               separator: *const c_char,
                               items: *const c_char)
                     -> *mut t_gui_bar,
    pub bar_set: extern "C" fn(bar: *mut t_gui_bar, property: *const c_char,
                               value: *const c_char) -> c_int,
    pub bar_update: extern "C" fn(name: *const c_char),
    pub bar_remove: extern "C" fn(bar: *mut t_gui_bar),

    /* command */
    pub command: extern "C" fn(plugin: *mut t_weechat_plugin,
                               buffer: *mut t_gui_buffer,
                               command: *const c_char) -> c_int,

    /* network */
    pub network_pass_proxy: extern "C" fn(proxy: *const c_char, sock: c_int,
                                          address: *const c_char, port: c_int)
                                -> c_int,
    pub network_connect_to: extern "C" fn(proxy: *const c_char,
                                          address: *mut sockaddr,
                                          address_length: socklen_t) -> c_int,

    /* infos */
    pub info_get: extern "C" fn(plugin: *mut t_weechat_plugin,
                                info_name: *const c_char,
                                arguments: *const c_char) -> *const c_char,
    pub info_get_hashtable: extern "C" fn(plugin: *mut t_weechat_plugin,
                                          info_name: *const c_char,
                                          hashtable: *mut t_hashtable)
                                -> *mut t_hashtable,

    /* infolists */
    pub infolist_new: extern "C" fn(plugin: *mut t_weechat_plugin)
                          -> *mut t_infolist,
    pub infolist_new_item: extern "C" fn(infolist: *mut t_infolist)
                               -> *mut t_infolist_item,
    pub infolist_new_var_integer: extern "C" fn(item: *mut t_infolist_item,
                                                name: *const c_char,
                                                value: c_int)
                                      -> *mut t_infolist_var,
    pub infolist_new_var_string: extern "C" fn(item: *mut t_infolist_item,
                                               name: *const c_char,
                                               value: *const c_char)
                                     -> *mut t_infolist_var,
    pub infolist_new_var_pointer: extern "C" fn(item: *mut t_infolist_item,
                                                name: *const c_char,
                                                pointer: *mut c_void)
                                      -> *mut t_infolist_var,
    pub infolist_new_var_buffer: extern "C" fn(item: *mut t_infolist_item,
                                               name: *const c_char,
                                               pointer: *mut c_void,
                                               size: c_int)
                                     -> *mut t_infolist_var,
    pub infolist_new_var_time: extern "C" fn(item: *mut t_infolist_item,
                                             name: *const c_char,
                                             time: time_t)
                                   -> *mut t_infolist_var,
    pub infolist_search_var: extern "C" fn(infolist: *mut t_infolist,
                                           name: *const c_char)
                                 -> *mut t_infolist_var,
    pub infolist_get: extern "C" fn(plugin: *mut t_weechat_plugin,
                                    infolist_name: *const c_char,
                                    pointer: *mut c_void,
                                    arguments: *const c_char)
                          -> *mut t_infolist,
    pub infolist_next: extern "C" fn(infolist: *mut t_infolist) -> c_int,
    pub infolist_prev: extern "C" fn(infolist: *mut t_infolist) -> c_int,
    pub infolist_reset_item_cursor: extern "C" fn(infolist: *mut t_infolist),
    pub infolist_fields: extern "C" fn(infolist: *mut t_infolist)
                             -> *const c_char,
    pub infolist_integer: extern "C" fn(infolist: *mut t_infolist,
                                        var: *const c_char) -> c_int,
    pub infolist_string: extern "C" fn(infolist: *mut t_infolist,
                                       var: *const c_char) -> *const c_char,
    pub infolist_pointer: extern "C" fn(infolist: *mut t_infolist,
                                        var: *const c_char) -> *mut c_void,
    pub infolist_buffer: extern "C" fn(infolist: *mut t_infolist,
                                       var: *const c_char, size: *mut c_int)
                             -> *mut c_void,
    pub infolist_time: extern "C" fn(infolist: *mut t_infolist,
                                     var: *const c_char) -> time_t,
    pub infolist_free: extern "C" fn(infolist: *mut t_infolist),

    /* hdata */
    pub hdata_new: extern "C" fn(plugin: *mut t_weechat_plugin,
                                 hdata_name: *const c_char,
                                 var_prev: *const c_char,
                                 var_next: *const c_char,
                                 create_allowed: c_int, delete_allowed: c_int,
                                 callback_update:
                                     Option<extern "C" fn(data: *mut c_void,
                                                          hdata: *mut t_hdata,
                                                          pointer: *mut c_void,
                                                          hashtable: *mut t_hashtable)
                                                -> c_int>,
                                 callback_update_data: *mut c_void)
                       -> *mut t_hdata,
    pub hdata_new_var: extern "C" fn(hdata: *mut t_hdata, name: *const c_char,
                                     offset: c_int, _type: c_int,
                                     update_allowed: c_int,
                                     array_size: *const c_char,
                                     hdata_name: *const c_char),
    pub hdata_new_list: extern "C" fn(hdata: *mut t_hdata,
                                      name: *const c_char,
                                      pointer: *mut c_void, flags: c_int),
    pub hdata_get: extern "C" fn(plugin: *mut t_weechat_plugin,
                                 hdata_name: *const c_char) -> *mut t_hdata,
    pub hdata_get_var_offset: extern "C" fn(hdata: *mut t_hdata,
                                            name: *const c_char) -> c_int,
    pub hdata_get_var_type: extern "C" fn(hdata: *mut t_hdata,
                                          name: *const c_char) -> c_int,
    pub hdata_get_var_type_string: extern "C" fn(hdata: *mut t_hdata,
                                                 name: *const c_char)
                                       -> *const c_char,
    pub hdata_get_var_array_size: extern "C" fn(hdata: *mut t_hdata,
                                                pointer: *mut c_void,
                                                name: *const c_char) -> c_int,
    pub hdata_get_var_array_size_string: extern "C" fn(hdata: *mut t_hdata,
                                                       pointer: *mut c_void,
                                                       name: *const c_char)
                                             -> *const c_char,
    pub hdata_get_var_hdata: extern "C" fn(hdata: *mut t_hdata,
                                           name: *const c_char)
                                 -> *const c_char,
    pub hdata_get_var: extern "C" fn(hdata: *mut t_hdata,
                                     pointer: *mut c_void,
                                     name: *const c_char) -> *mut c_void,
    pub hdata_get_var_at_offset: extern "C" fn(hdata: *mut t_hdata,
                                               pointer: *mut c_void,
                                               offset: c_int) -> *mut c_void,
    pub hdata_get_list: extern "C" fn(hdata: *mut t_hdata,
                                      name: *const c_char) -> *mut c_void,
    pub hdata_check_pointer: extern "C" fn(hdata: *mut t_hdata,
                                           list: *mut c_void,
                                           pointer: *mut c_void) -> c_int,
    pub hdata_move: extern "C" fn(hdata: *mut t_hdata, pointer: *mut c_void,
                                  count: c_int) -> *mut c_void,
    pub hdata_search: extern "C" fn(hdata: *mut t_hdata, pointer: *mut c_void,
                                    search: *const c_char, _move: c_int)
                          -> *mut c_void,
    pub hdata_char: extern "C" fn(hdata: *mut t_hdata, pointer: *mut c_void,
                                  name: *const c_char) -> c_char,
    pub hdata_integer: extern "C" fn(hdata: *mut t_hdata,
                                     pointer: *mut c_void,
                                     name: *const c_char) -> c_int,
    pub hdata_long: extern "C" fn(hdata: *mut t_hdata, pointer: *mut c_void,
                                  name: *const c_char) -> c_long,
    pub hdata_string: extern "C" fn(hdata: *mut t_hdata, pointer: *mut c_void,
                                    name: *const c_char) -> *const c_char,
    pub hdata_pointer: extern "C" fn(hdata: *mut t_hdata,
                                     pointer: *mut c_void,
                                     name: *const c_char) -> *mut c_void,
    pub hdata_time: extern "C" fn(hdata: *mut t_hdata, pointer: *mut c_void,
                                  name: *const c_char) -> time_t,
    pub hdata_hashtable: extern "C" fn(hdata: *mut t_hdata,
                                       pointer: *mut c_void,
                                       name: *const c_char)
                             -> *mut t_hashtable,
    pub hdata_set: extern "C" fn(hdata: *mut t_hdata, pointer: *mut c_void,
                                 name: *const c_char, value: *const c_char)
                       -> c_int,
    pub hdata_update: extern "C" fn(hdata: *mut t_hdata, pointer: *mut c_void,
                                    hashtable: *mut t_hashtable) -> c_int,
    pub hdata_get_string: extern "C" fn(hdata: *mut t_hdata,
                                        property: *const c_char)
                              -> *const c_char,

    /* upgrade */
    pub upgrade_new: extern "C" fn(filename: *const c_char, write: c_int)
                         -> *mut t_upgrade_file,
    pub upgrade_write_object: extern "C" fn(upgrade_file: *mut t_upgrade_file,
                                            object_id: c_int,
                                            infolist: *mut t_infolist)
                                  -> c_int,
    pub upgrade_read: extern "C" fn(upgrade_file: *mut t_upgrade_file,
                                    callback_read:
                                        Option<extern "C" fn(data: *mut c_void,
                                                             upgrade_file: *mut t_upgrade_file,
                                                             object_id: c_int,
                                                             infolist: *mut t_infolist)
                                                   -> c_int>,
                                    callback_read_data: *mut c_void) -> c_int,
    pub upgrade_close: extern "C" fn(upgrade_file: *mut t_upgrade_file),
}
