#![warn(missing_docs)]

//! Main weechat module

use weechat_sys::t_weechat_plugin;

use crate::LossyCString;
use libc::{c_char, c_int};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::{ptr, vec};

/// An iterator over the arguments of a command, yielding a String value for
/// each argument.
pub struct ArgsWeechat {
    iter: vec::IntoIter<String>,
}

impl ArgsWeechat {
    /// Create an ArgsWeechat object from the underlying weechat C types.
    /// Expects the strings in argv to be valid utf8, if not invalid UTF-8
    /// sequences are replaced with the replacement character.
    pub fn new(argc: c_int, argv: *mut *mut c_char) -> ArgsWeechat {
        let argc = argc as isize;
        let args: Vec<String> = (0..argc)
            .map(|i| {
                let cstr = unsafe {
                    CStr::from_ptr(*argv.offset(i) as *const libc::c_char)
                };

                String::from_utf8_lossy(&cstr.to_bytes().to_vec()).to_string()
            })
            .collect();
        ArgsWeechat {
            iter: args.clone().into_iter(),
        }
    }
}

impl Iterator for ArgsWeechat {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl ExactSizeIterator for ArgsWeechat {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl DoubleEndedIterator for ArgsWeechat {
    fn next_back(&mut self) -> Option<String> {
        self.iter.next_back()
    }
}

/// Status for updating options
pub enum OptionChanged {
    Changed = weechat_sys::WEECHAT_CONFIG_OPTION_SET_OK_CHANGED as isize,
    Unchanged = weechat_sys::WEECHAT_CONFIG_OPTION_SET_OK_SAME_VALUE as isize,
    NotFound = weechat_sys::WEECHAT_CONFIG_OPTION_SET_OPTION_NOT_FOUND as isize,
    Error = weechat_sys::WEECHAT_CONFIG_OPTION_SET_ERROR as isize,
}

impl OptionChanged {
    pub fn from_int(v: i32) -> OptionChanged {
        use OptionChanged::*;
        match v {
            weechat_sys::WEECHAT_CONFIG_OPTION_SET_OK_CHANGED => Changed,
            weechat_sys::WEECHAT_CONFIG_OPTION_SET_OK_SAME_VALUE => Unchanged,
            weechat_sys::WEECHAT_CONFIG_OPTION_SET_OPTION_NOT_FOUND => NotFound,
            weechat_sys::WEECHAT_CONFIG_OPTION_SET_ERROR => Error,
            _ => unreachable!(),
        }
    }
}

/// Main Weechat struct that encapsulates common weechat API functions.
/// It has a similar API as the weechat script API.
pub struct Weechat {
    pub(crate) ptr: *mut t_weechat_plugin,
}

impl Weechat {
    /// Create a Weechat object from a C t_weechat_plugin pointer.
    /// * `ptr` - Pointer of the weechat plugin.
    pub fn from_ptr(ptr: *mut t_weechat_plugin) -> Weechat {
        assert!(!ptr.is_null());

        Weechat { ptr }
    }

    #[inline]
    pub(crate) fn get(&self) -> &t_weechat_plugin {
        unsafe { &*self.ptr }
    }

    /// Write a message in WeeChat log file (weechat.log).
    pub fn log(&self, msg: &str) {
        let log_printf = self.get().log_printf.unwrap();

        let fmt = CString::new("%s").unwrap();
        let msg = LossyCString::new(msg);

        unsafe {
            log_printf(fmt.as_ptr(), msg.as_ptr());
        }
    }

    /// Display a message on the core weechat buffer.
    pub fn print(&self, msg: &str) {
        let printf_date_tags = self.get().printf_date_tags.unwrap();

        let fmt = CString::new("%s").unwrap();
        let msg = LossyCString::new(msg);

        unsafe {
            printf_date_tags(
                ptr::null_mut(),
                0,
                ptr::null(),
                fmt.as_ptr(),
                msg.as_ptr(),
            );
        }
    }

    /// Return a string color code for display.
    /// * `color_name` - name the color
    pub fn color(&self, color_name: &str) -> Cow<str> {
        let weechat_color = self.get().color.unwrap();

        let color_name = LossyCString::new(color_name);
        unsafe {
            let color = weechat_color(color_name.as_ptr());
            CStr::from_ptr(color).to_string_lossy()
        }
    }

    /// Get some info from Weechat or a plugin.
    /// * `info_name` - name the info
    /// * `arguments` - arguments for the info
    pub fn info_get(
        &self,
        info_name: &str,
        arguments: &str,
    ) -> Option<Cow<str>> {
        let info_get = self.get().info_get.unwrap();

        let info_name = LossyCString::new(info_name);
        let arguments = LossyCString::new(arguments);

        unsafe {
            let info =
                info_get(self.ptr, info_name.as_ptr(), arguments.as_ptr());
            if info.is_null() {
                None
            } else {
                Some(CStr::from_ptr(info).to_string_lossy())
            }
        }
    }

    /// Get value of a plugin option
    pub fn get_plugin_option(&self, option: &str) -> Option<&str> {
        let config_get_plugin = self.get().config_get_plugin.unwrap();

        let option_name = CString::new(option).unwrap_or_default();

        unsafe {
            let option = config_get_plugin(self.ptr, option_name.as_ptr());
            if option.is_null() {
                None
            } else {
                Some(CStr::from_ptr(option).to_str().unwrap_or_default())
            }
        }
    }

    /// Set the value of a plugin option
    pub fn set_plugin_option(
        &self,
        option: &str,
        value: &str,
    ) -> OptionChanged {
        let config_set_plugin = self.get().config_set_plugin.unwrap();

        let option_name = CString::new(option).unwrap_or_default();
        let value = CString::new(value).unwrap_or_default();

        unsafe {
            let result = config_set_plugin(
                self.ptr,
                option_name.as_ptr(),
                value.as_ptr(),
            );

            OptionChanged::from_int(result as i32)
        }
    }

    /// Evaluate a weechat expression and return the result
    //
    // TODO: Add hashtable options
    pub fn eval_string_expression(&self, expr: &str) -> Option<&str> {
        let string_eval_expression = self.get().string_eval_expression.unwrap();

        let expr = CString::new(expr).unwrap_or_default();

        unsafe {
            let result = string_eval_expression(
                expr.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if result.is_null() {
                None
            } else {
                Some(CStr::from_ptr(result).to_str().unwrap_or_default())
            }
        }
    }
}
