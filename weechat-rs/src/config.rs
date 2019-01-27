#![warn(missing_docs)]

//! Weechat Configuration module

use libc::{c_char, c_int};
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use config_options::{
    ConfigOption, IntegerOption, OptionDescription, OptionPointers, OptionType,
    StringOption,
};
use weechat::Weechat;
use weechat_sys::{
    t_config_file, t_config_option, t_config_section, t_weechat_plugin,
    WEECHAT_RC_OK,
};

/// Weechat configuration file
pub struct Config<T> {
    ptr: *mut t_config_file,
    weechat_ptr: *mut t_weechat_plugin,
    _config_data: Box<ConfigPointers<T>>,
    sections: HashMap<String, ConfigSection>,
}

struct ConfigPointers<T> {
    reload_cb: Option<fn(&mut T)>,
    reload_data: T,
}

/// Weechat Configuration section
pub struct ConfigSection {
    ptr: *mut t_config_section,
    config_ptr: *mut t_config_file,
    weechat_ptr: *mut t_weechat_plugin,
}

#[derive(Default)]
pub struct ConfigSectionInfo<'a, T> {
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

impl<T> Drop for Config<T> {
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

impl<T> Config<T> {
    /// Create a new section in the configuration file.
    /// * `name` - name of the new section.
    pub fn new_section<S: Default>(
        &mut self,
        section_info: ConfigSectionInfo<S>,
    ) -> &ConfigSection {
        unsafe extern "C" fn c_read_cb<S>(
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
        let section = ConfigSection {
            ptr,
            config_ptr: self.ptr,
            weechat_ptr: weechat.ptr,
        };
        self.sections.insert(section_info.name.to_string(), section);
        &self.sections[section_info.name]
    }
}

type WeechatOptChangeCbT = unsafe extern "C" fn(
    pointer: *const c_void,
    _data: *mut c_void,
    option_pointer: *mut t_config_option,
);

impl ConfigSection {
    /// Create a new Weechat configuration option.
    /// * `name` - Name of the new option
    pub fn new_string_option<D>(
        &self,
        name: &str,
        description: &str,
        default_value: &str,
        value: &str,
        null_allowed: bool,
        change_cb: Option<fn(&mut D, &StringOption)>,
        change_cb_data: Option<D>,
    ) -> StringOption
    where
        D: Default,
    {
        let ptr = self.new_option(
            OptionDescription {
                name,
                description,
                option_type: OptionType::String,
                default_value,
                value,
                null_allowed,
                ..Default::default()
            },
            change_cb,
            change_cb_data,
        );
        StringOption {
            ptr,
            weechat_ptr: self.weechat_ptr,
        }
    }

    pub fn new_integer_option<D>(
        &self,
        name: &str,
        description: &str,
        string_values: &str,
        min: i32,
        max: i32,
        default_value: &str,
        value: &str,
        null_allowed: bool,
        change_cb: Option<fn(&mut D, &IntegerOption)>,
        change_cb_data: Option<D>,
    ) -> IntegerOption
    where
        D: Default,
    {
        let ptr = self.new_option(
            OptionDescription {
                name,
                option_type: OptionType::Integer,
                description,
                string_values,
                min,
                max,
                default_value,
                value,
                null_allowed,
            },
            change_cb,
            change_cb_data,
        );
        IntegerOption {
            ptr,
            weechat_ptr: self.weechat_ptr,
        }
    }

    fn new_option<D, T>(
        &self,
        option_description: OptionDescription,
        change_cb: Option<fn(&mut D, &T)>,
        change_cb_data: Option<D>,
    ) -> *mut t_config_option
    where
        D: Default,
        T: ConfigOption,
    {
        unsafe extern "C" fn c_change_cb<D, T>(
            pointer: *const c_void,
            _data: *mut c_void,
            option_pointer: *mut t_config_option,
        ) where
            T: ConfigOption,
        {
            let pointers: &mut OptionPointers<D, T> =
                { &mut *(pointer as *mut OptionPointers<D, T>) };

            let option = T::from_ptrs(option_pointer, pointers.weechat_ptr);

            let data = &mut pointers.change_cb_data;

            if let Some(callback) = pointers.change_cb {
                callback(data, &option)
            };
        }

        let weechat = Weechat::from_ptr(self.weechat_ptr);

        let name = CString::new(option_description.name).unwrap();
        let description = CString::new(option_description.description).unwrap();
        let option_type =
            CString::new(option_description.option_type.as_str()).unwrap();
        let string_values =
            CString::new(option_description.string_values).unwrap();
        let default_value =
            CString::new(option_description.default_value).unwrap();
        let value = CString::new(option_description.value).unwrap();

        let option_pointers = Box::new(OptionPointers::<D, T> {
            weechat_ptr: self.weechat_ptr,
            change_cb: change_cb,
            change_cb_data: change_cb_data.unwrap_or_default(),
        });

        // TODO this leaks curently.
        let option_pointers_ref: &OptionPointers<D, T> =
            Box::leak(option_pointers);

        let c_change_cb: Option<WeechatOptChangeCbT> = match change_cb {
            Some(_) => Some(c_change_cb::<D, T>),
            None => None,
        };

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
                option_description.null_allowed as i32,
                None,
                option_pointers_ref as *const _ as *const c_void,
                ptr::null_mut(),
                c_change_cb,
                option_pointers_ref as *const _ as *const c_void,
                ptr::null_mut(),
                None,
                option_pointers_ref as *const _ as *const c_void,
                ptr::null_mut(),
            )
        }
    }
}

type WeechatReloadT = unsafe extern "C" fn(
    pointer: *const c_void,
    _data: *mut c_void,
    _config_pointer: *mut t_config_file,
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
    pub fn config_new<T: Default>(
        &self,
        name: &str,
        reload_callback: Option<fn(&mut T)>,
        reload_data: Option<T>,
    ) -> Config<T> {
        unsafe extern "C" fn c_reload_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            _config_pointer: *mut t_config_file,
        ) -> c_int {
            let pointers: &mut ConfigPointers<T> =
                { &mut *(pointer as *mut ConfigPointers<T>) };

            let data = &mut pointers.reload_data;

            if let Some(callback) = pointers.reload_cb {
                callback(data)
            }

            WEECHAT_RC_OK
        }

        let c_name = CString::new(name).unwrap();

        let config_pointers = Box::new(ConfigPointers::<T> {
            reload_cb: reload_callback,
            reload_data: reload_data.unwrap_or_default(),
        });
        let config_pointers_ref = Box::leak(config_pointers);

        let c_reload_cb: Option<WeechatReloadT> = match reload_callback {
            Some(_) => Some(c_reload_cb::<T>),
            None => None,
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

        let config_data = unsafe { Box::from_raw(config_pointers_ref) };
        Config {
            ptr: config_ptr,
            weechat_ptr: self.ptr,
            _config_data: config_data,
            sections: HashMap::new(),
        }
    }
}
