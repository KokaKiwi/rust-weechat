#![warn(missing_docs)]

//! Weechat Configuration module

use std::ffi::CString;
use std::os::raw::c_void;
use libc::{c_int, c_char};
use std::ptr;
use std::collections::HashMap;

use weechat_sys::{
    t_weechat_plugin,
    t_config_file,
    t_config_section,
    WEECHAT_RC_OK
};
use weechat::{Weechat};


/// Weechat configuration file
pub struct Config {
    pub(crate) ptr: *mut t_config_file,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
    sections: HashMap<String, ConfigSection>,
}

struct ConfigPointers<T> {
    pub(crate) reload_cb: Option<fn(&mut T)>,
    pub(crate) reload_data: T,
}

/// Weechat Configuration section
pub struct ConfigSection {
    pub(crate) ptr: *mut t_config_section,
    pub(crate) config_ptr: *mut t_config_file,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

#[derive(Default)]
pub struct OptionDescription<'a> {
    pub name: &'a str,
    pub option_type: &'a str,
    pub description: &'a str,
    pub string_values: &'a str,
    pub min: i32,
    pub max: i32,
    pub default_value: &'a str,
    pub value: &'a str,
    pub null_alowed: bool,
}

#[derive(Default)]
pub struct ConfigSectionInfo<'a, T>  {
    pub name: &'a str,

    pub user_can_add_options: bool,
    pub user_can_delete_option: bool,

    pub read_callback: Option<fn(&T)>,
    pub read_callback_data: Option<T>,

    pub write_callbck: Option<fn(&T)>,
    pub write_callback_data: Option<T>,

    pub write_default_callbck: Option<fn(&T)>,
    pub write_default_callback_data: Option<T>,

    pub create_option_callback: Option<fn(&T)>,
    pub create_option_callback_data: Option<T>,

    pub delete_option_callback: Option<fn(&T)>,
    pub delete_option_callback_data: Option<T>,
}

impl Drop for Config {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let config_free = weechat.get().config_free.unwrap();

        // Drop the sections first.
        self.sections.clear();

        unsafe {
            // Now drop the config.
            config_free(self.ptr)
        };
    }
}

impl Drop for ConfigSection {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);

        let options_free = weechat.get().config_section_free_options.unwrap();
        let section_free = weechat.get().config_section_free.unwrap();

        unsafe {
            options_free(self.ptr);
            section_free(self.ptr);
        };
    }
}

impl Config {
    /// Create a new section in the configuration file.
    /// * `name` - Name of the new configuration file
    pub fn new_section<T: Default>(
        &mut self,
        section_info: ConfigSectionInfo<T>
    ) -> &ConfigSection {
        unsafe extern "C" fn c_read_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            _config: *mut t_config_file,
            _section: *mut t_config_section,
            option_name: *mut *mut c_char,
            value: *mut *mut c_char,
        ) -> c_int {
            WEECHAT_RC_OK
        }

        let weechat = Weechat::from_ptr(self.weechat_ptr);

        let new_section = weechat.get().config_new_section.unwrap();

        let name = CString::new(section_info.name).unwrap_or_default();

        let ptr = unsafe {
            new_section(
                self.ptr,
                name.as_ptr(),
                section_info.user_can_add_options as i32,
                section_info.user_can_delete_option as i32,
                None,
                ptr::null_mut(),
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        let section = ConfigSection { ptr, config_ptr: self.ptr, weechat_ptr: weechat.ptr };
        self.sections.insert(section_info.name.to_string(), section);
        &self.sections[section_info.name]
    }
}

impl ConfigSection {
    pub fn new_option(
        &self,
        option_description: OptionDescription
    ) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);

        let name = CString::new(option_description.name).unwrap();
        let description = CString::new(option_description.description).unwrap();
        let option_type = CString::new(option_description.option_type).unwrap();
        let string_values = CString::new(option_description.string_values).unwrap();
        let default_value = CString::new(option_description.default_value).unwrap();
        let value = CString::new(option_description.value).unwrap();

        let config_new_option = weechat.get().config_new_option.unwrap();
        unsafe {
            config_new_option(
                self.config_ptr,
                self.ptr,
                name.as_ptr(),
                option_type.as_ptr(),
                description.as_ptr(),
                string_values.as_ptr(),
                option_description.min,
                option_description.max,
                default_value.as_ptr(),
                value.as_ptr(),
                option_description.null_alowed as i32,
                None,
                ptr::null_mut(),
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
    }
}

type WeechatReloadT = unsafe extern "C" fn(
    pointer: *const c_void,
    _data: *mut c_void,
    _config_pointer: *mut t_config_file
) -> c_int;

/// Configuration file part of the weechat API.
impl Weechat {
    /// Create a new Weechat configuration file, returns a `Config` object.
    /// The configuration file is freed when the `Config` object is dropped.
    /// * `name` - Name of the new configuration file
    /// * `reload_callback` - Callback that will be called when the
    /// configuration file is reloaded.
    /// * `reload_data` - Data that will be taken over by weechat and passed
    /// to the reload callback, this data will be freed when the `Config`
    /// object returned by this method is dropped.
    pub fn config_new <T: Default>(
        &self,
        name: &str,
        reload_callback: Option<fn(&mut T)>,
        reload_data: Option<T>) -> Config
    {

        unsafe extern "C" fn c_reload_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            _config_pointer: *mut t_config_file
        ) -> c_int {
            let pointers: &mut ConfigPointers<T> =
                { &mut *(pointer as *mut ConfigPointers<T>) };

            let data = &mut pointers.reload_data;

            if let Some(callback) = pointers.reload_cb { callback(data) }

            WEECHAT_RC_OK
        }

        let c_name = CString::new(name).unwrap();

        let config_pointers = Box::new(ConfigPointers::<T> {
            reload_cb: reload_callback,
            reload_data: reload_data.unwrap_or_default(),
        });
        let config_pointers_ref: &ConfigPointers<T> = Box::leak(config_pointers);

        let c_reload_cb: Option<WeechatReloadT> = match reload_callback {
                Some(_) => Some(c_reload_cb::<T>),
                None => None
        };

        let config_new = self.get().config_new.unwrap();
        let config_ptr = unsafe {
            config_new(
                self.ptr,
                c_name.as_ptr(),
                c_reload_cb,
                config_pointers_ref as *const _ as *const c_void,
                ptr::null_mut(),
            )
        };
        Config { ptr: config_ptr, weechat_ptr: self.ptr , sections: HashMap::new()}
    }
}
