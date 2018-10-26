#![warn(missing_docs)]

//! Weechat Buffer module containing Buffer and Nick types.
use weechat_sys::{
    t_weechat_plugin,
    t_gui_buffer,
    t_gui_nick,
};
use std::ffi::{CString};
use std::ptr;
use weechat::Weechat;

/// A high level Buffer type encapsulating weechats C buffer pointer.
/// The buffer won't be closed if the object is destroyed.
pub struct Buffer {
    pub(crate) weechat: *mut t_weechat_plugin,
    pub(crate) ptr: *mut t_gui_buffer
}

/// Nick creation arguments
pub struct Nick<'a> {
    pub ptr: Option<*mut t_gui_nick>,
    pub buf_ptr: Option<*mut t_gui_buffer>,
    pub name: &'a str,
    pub color: &'a str,
    pub prefix: &'a str,
    pub prefix_color: &'a str,
    pub visible: bool,
}

impl<'a> Nick<'a> {
    fn from_ptr(ptr: *mut t_gui_nick, buf_ptr: *mut t_gui_nick) {
    }
}

impl<'a> Default for Nick<'a> {
    fn default() -> Nick<'a> {
        Nick {
            ptr: None,
            buf_ptr: None,
            name: "",
            color: "",
            prefix: "",
            prefix_color: "",
            visible: true
        }
    }
}

impl Buffer {
    /// Create a high level Buffer object from a C plugin pointer and the buffer pointer.
    pub(crate) fn from_ptr(weechat_ptr: *mut t_weechat_plugin, buffer_ptr: *mut t_gui_buffer) -> Buffer {
        Buffer {
            weechat: weechat_ptr,
            ptr: buffer_ptr
        }
    }

    /// Get the Weechat plugin object from a Buffer object.
    pub fn get_weechat(self) -> Weechat {
        Weechat::from_ptr(self.weechat)
    }

    /// Display a message on the buffer.
    pub fn print(&self, message: &str) {
        let weechat = Weechat::from_ptr(self.weechat);
        let printf_date_tags = weechat.get().printf_date_tags.unwrap();

        let c_message = CString::new(message).unwrap();

        unsafe {
            printf_date_tags(self.ptr, 0, ptr::null(), c_message.as_ptr())
        }

    }

    /// Create and add a new nick to the buffer nicklist. Returns the newly created nick.
    pub fn add_nick(&self, nick: &mut Nick) {
        let weechat = Weechat::from_ptr(self.weechat);

        let c_nick = CString::new(nick.name).unwrap();
        let color = CString::new("green").unwrap();
        let add_nick = weechat.get().nicklist_add_nick.unwrap();

        let nick_ptr = unsafe {
            add_nick(
                self.ptr,
                ptr::null_mut(),
                c_nick.as_ptr(),
                color.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                1,
            )
        };

        nick.ptr = Some(nick_ptr);
        nick.buf_ptr = Some(self.ptr);
    }
}
