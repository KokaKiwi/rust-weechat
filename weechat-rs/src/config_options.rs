#![warn(missing_docs)]

//! A module providing a typed api for Weechat configuration files

use crate::Weechat;
use std::borrow::Cow;
use weechat_sys::{t_config_option, t_weechat_plugin};

#[derive(Default)]
pub(crate) struct OptionDescription<'a> {
    pub name: &'a str,
    pub option_type: OptionType,
    pub description: &'a str,
    pub string_values: &'a str,
    pub min: i32,
    pub max: i32,
    pub default_value: &'a str,
    pub value: &'a str,
    pub null_allowed: bool,
}

pub(crate) enum OptionType {
    Boolean,
    Integer,
    String,
    Color,
}

impl OptionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OptionType::Boolean => "boolean",
            OptionType::Integer => "integer",
            OptionType::String => "string",
            OptionType::Color => "color",
        }
    }
}

impl Default for OptionType {
    fn default() -> Self {
        OptionType::String
    }
}

/// A trait that defines common behavior for the different data types of config options.
pub trait ConfigOption {
    /// Returns the weechat object that this config option was created with.
    fn get_weechat(&self) -> Weechat;
    /// Returns the raw pointer to the config option.
    fn get_ptr(&self) -> *mut t_config_option;

    /// Constructs a ConfigOption from its raw pointer and a weechat pointer.
    fn from_ptrs(
        ptr: *mut t_config_option,
        weechat_ptr: *mut t_weechat_plugin,
    ) -> Self;

    /// Resets the option to its default value.
    fn reset(&self, run_callback: bool) -> crate::OptionChanged {
        let weechat = self.get_weechat();
        let option_reset = weechat.get().config_option_reset.unwrap();

        let ret = unsafe { option_reset(self.get_ptr(), run_callback as i32) };

        crate::OptionChanged::from_int(ret)
    }
}

pub(crate) struct OptionPointers<T, A, B, C> {
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
    pub(crate) check_cb: Option<fn(&mut A, &T, Cow<str>)>,
    pub(crate) check_cb_data: A,
    pub(crate) change_cb: Option<fn(&mut B, &T)>,
    pub(crate) change_cb_data: B,
    pub(crate) delete_cb: Option<fn(&mut C, &T)>,
    pub(crate) delete_cb_data: C,
}

/// A config option with a string value.
pub struct StringOption {
    pub(crate) ptr: *mut t_config_option,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

/// A config option with a boolean value.
pub struct BooleanOption {
    pub(crate) ptr: *mut t_config_option,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

/// A config option with a integer value.
pub struct IntegerOption {
    pub(crate) ptr: *mut t_config_option,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

/// A config option with a color value.
pub struct ColorOption {
    pub(crate) ptr: *mut t_config_option,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

impl ConfigOption for StringOption {
    fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat_ptr)
    }
    fn get_ptr(&self) -> *mut t_config_option {
        self.ptr
    }
    fn from_ptrs(
        ptr: *mut t_config_option,
        weechat_ptr: *mut t_weechat_plugin,
    ) -> StringOption {
        StringOption { ptr, weechat_ptr }
    }
}

impl ConfigOption for BooleanOption {
    fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat_ptr)
    }
    fn get_ptr(&self) -> *mut t_config_option {
        self.ptr
    }
    fn from_ptrs(
        ptr: *mut t_config_option,
        weechat_ptr: *mut t_weechat_plugin,
    ) -> BooleanOption {
        BooleanOption { ptr, weechat_ptr }
    }
}

impl ConfigOption for IntegerOption {
    fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat_ptr)
    }
    fn get_ptr(&self) -> *mut t_config_option {
        self.ptr
    }
    fn from_ptrs(
        ptr: *mut t_config_option,
        weechat_ptr: *mut t_weechat_plugin,
    ) -> IntegerOption {
        IntegerOption { ptr, weechat_ptr }
    }
}

impl ConfigOption for ColorOption {
    fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat_ptr)
    }
    fn get_ptr(&self) -> *mut t_config_option {
        self.ptr
    }
    fn from_ptrs(
        ptr: *mut t_config_option,
        weechat_ptr: *mut t_weechat_plugin,
    ) -> ColorOption {
        ColorOption { ptr, weechat_ptr }
    }
}
