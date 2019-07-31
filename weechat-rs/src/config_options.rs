#![warn(missing_docs)]

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

pub trait ConfigOption {
    fn get_weechat(&self) -> Weechat;
    fn get_ptr(&self) -> *mut t_config_option;

    fn from_ptrs(
        ptr: *mut t_config_option,
        weechat_ptr: *mut t_weechat_plugin,
    ) -> Self;

    fn reset(&self, run_callback: bool) {
        let weechat = self.get_weechat();
        let option_reset = weechat.get().config_option_reset.unwrap();

        // TODO pass the value to the caller
        let ret = unsafe { option_reset(self.get_ptr(), run_callback as i32) };
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

pub struct StringOption {
    pub(crate) ptr: *mut t_config_option,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

pub struct BooleanOption {
    pub(crate) ptr: *mut t_config_option,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

pub struct IntegerOption {
    pub(crate) ptr: *mut t_config_option,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

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
