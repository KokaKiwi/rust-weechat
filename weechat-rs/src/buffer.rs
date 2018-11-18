#![warn(missing_docs)]

//! Weechat Buffer module containing Buffer and Nick types.
use weechat_sys::{
    t_weechat_plugin,
    t_gui_buffer,
    t_gui_nick_group,
    t_gui_nick,
};
use std::ffi::{CString, CStr};
use std::ptr;
use weechat::Weechat;
use libc::{c_char, c_int};
use std::os::raw::c_void;

/// A high level Buffer type encapsulating weechats C buffer pointer.
/// The buffer won't be closed if the object is destroyed.
pub struct Buffer {
    pub(crate) weechat: *mut t_weechat_plugin,
    pub(crate) ptr: *mut t_gui_buffer
}

pub(crate) struct BufferPointers<A, B> {
    pub(crate) weechat: *mut t_weechat_plugin,
    pub(crate) input_cb: Option<fn(&mut A, Buffer, &str)>,
    pub(crate) input_data: A,
    pub(crate) close_cb: Option<fn(&B, Buffer)>,
    pub(crate) close_cb_data: B
}

pub(crate) type WeechatInputCbT = unsafe extern "C" fn(
    pointer: *const c_void,
    data: *mut c_void,
    buffer: *mut t_gui_buffer,
    input_data: *const c_char
) -> c_int;


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
    weechat_ptr: *mut t_weechat_plugin,
}

impl Nick {
    /// Create a high level Nick object from C nick and buffer pointers.
    pub(crate) fn from_ptr(
        ptr: *mut t_gui_nick,
        buf_ptr: *mut t_gui_buffer,
        weechat_ptr: *mut t_weechat_plugin,
    ) -> Nick {
        Nick { ptr, buf_ptr, weechat_ptr }
    }

    /// Get a Weechat object out of the nick.
    fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat_ptr)
    }

    /// Get a string property of the nick.
    /// * `property` - The name of the property to get the value for, this can be one of name,
    /// color, prefix or prefix_color. If a unknown property is requested an empty string is
    /// returned.
    pub fn get_string(&self, property: &str) -> &str {
        let weechat = self.get_weechat();
        let get_string = weechat.get().nicklist_nick_get_string.unwrap();
        let c_property = CString::new(property).unwrap();
        let value = unsafe {
            let ret = get_string(self.buf_ptr, self.ptr, c_property.as_ptr());
            CStr::from_ptr(ret)
        };
        value.to_str().unwrap_or_default()
    }

    /// Get the name property of the nick.
    pub fn get_name(&self) -> &str {
        self.get_string("name")
    }
}

/// Weechat nicklist Group type.
pub struct NickGroup {
    pub(crate) ptr: *mut t_gui_nick_group,
    buf_ptr: *mut t_gui_buffer,
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
    /// * `nick` - Nick arguments struct for the nick that should be added.
    /// * `group` - Nicklist group that the nick should be added to. If no group is provided the
    /// nick is added to the root group.
    pub fn add_nick(&self, nick: NickArgs, group: Option<&NickGroup>) -> Nick {
        let weechat = Weechat::from_ptr(self.weechat);

        // TODO this conversions can fail if any of those strings contain a null byte.
        let c_nick = CString::new(nick.name).unwrap();
        let color = CString::new(nick.color).unwrap();
        let prefix = CString::new(nick.prefix).unwrap();
        let prefix_color = CString::new(nick.prefix_color).unwrap();
        let add_nick = weechat.get().nicklist_add_nick.unwrap();

        let group_ptr = match group {
            Some(g) => g.ptr,
            None => ptr::null_mut()
        };

        let nick_ptr = unsafe {
            add_nick(
                self.ptr,
                group_ptr,
                c_nick.as_ptr(),
                color.as_ptr(),
                prefix.as_ptr(),
                prefix_color.as_ptr(),
                nick.visible as i32,
            )
        };

        Nick::from_ptr(nick_ptr, self.ptr, self.weechat)
    }

    /// Create and add a new nicklist group to the buffers nicklist.
    /// * `name` - Name of the new group.
    /// * `color` - Color of the new group.
    /// * `visible` - Should the group be visible in the nicklist.
    /// * `parent_group` - Parent group that the group should be added to. If no group is provided the
    /// group is added to the root group.
    pub fn add_group(&self, name: &str, color: &str, visible: bool, parent_group: Option<&NickGroup>) -> NickGroup {
        let weechat = Weechat::from_ptr(self.weechat);
        let add_group = weechat.get().nicklist_add_group.unwrap();

        let c_name = CString::new(name).unwrap();
        let c_color = CString::new(color).unwrap();

        let group_ptr = match parent_group {
            Some(g) => g.ptr,
            None => ptr::null_mut()
        };

        let group_ptr = unsafe {
            add_group(self.ptr, group_ptr, c_name.as_ptr(), c_color.as_ptr(), visible as i32)
        };

        NickGroup { ptr: group_ptr, buf_ptr: self.ptr }
    }
}
