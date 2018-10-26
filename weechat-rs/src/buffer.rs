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
pub struct NickArgs<'a> {
    /// Name of the new nick.
    pub name: &'a str,
    /// Color for the nick.
    pub color: &'a str,
    /// Prefix that will be shown before the name.
    pub prefix: &'a str,
    /// Color of the prefix.
    pub prefix_color: &'a str,
    /// Should the nick be visible in the nicklist.
    pub visible: bool,
}

/// Weechat Nick type
pub struct Nick {
    ptr: *mut t_gui_nick,
    buf_ptr: *mut t_gui_buffer,
}

impl Nick {
    /// Create a high level Nick object from C nick and buffer pointers.
    pub(crate) fn from_ptr(ptr: *mut t_gui_nick, buf_ptr: *mut t_gui_buffer) -> Nick {
        Nick { ptr, buf_ptr }
    }
}

impl<'a> Default for NickArgs<'a> {
    fn default() -> NickArgs<'a> {
        NickArgs {
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
    /// The nick won't be removed from the nicklist if the returned nick is dropped.
    pub fn add_nick(&self, nick: NickArgs) -> Nick {
        let weechat = Weechat::from_ptr(self.weechat);

        let c_nick = CString::new(nick.name).unwrap();
        let color = CString::new(nick.color).unwrap();
        let prefix = CString::new(nick.color).unwrap();
        let prefix_color = CString::new(nick.color).unwrap();
        let add_nick = weechat.get().nicklist_add_nick.unwrap();

        let nick_ptr = unsafe {
            add_nick(
                self.ptr,
                ptr::null_mut(),
                c_nick.as_ptr(),
                color.as_ptr(),
                prefix.as_ptr(),
                prefix_color.as_ptr(),
                nick.visible as i32,
            )
        };

        Nick::from_ptr(nick_ptr, self.ptr)
    }
}
