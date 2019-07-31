#![warn(missing_docs)]

//! Weechat Buffer module containing Buffer and Nick types.
use crate::Weechat;
use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;
use weechat_sys::{
    t_gui_buffer, t_gui_nick, t_gui_nick_group, t_weechat_plugin,
    WEECHAT_RC_ERROR, WEECHAT_RC_OK,
};

/// A high level Buffer type encapsulating weechats C buffer pointer.
/// The buffer won't be closed if the object is destroyed.
#[derive(Eq)]
pub struct Buffer {
    pub(crate) weechat: *mut t_weechat_plugin,
    pub(crate) ptr: *mut t_gui_buffer,
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Buffer) -> bool {
        self.ptr == other.ptr
    }
}

pub(crate) struct BufferPointers<A, B> {
    pub(crate) weechat: *mut t_weechat_plugin,
    pub(crate) input_cb: Option<fn(&mut A, Buffer, &str)>,
    pub(crate) input_data: A,
    pub(crate) close_cb: Option<fn(&B, Buffer)>,
    pub(crate) close_cb_data: B,
}

impl Weechat {
    /// Search a buffer by plugin and/or name.
    /// * `plugin_name` - name of a plugin, the following special value is
    ///     allowed: "==", the buffer name used is the buffers full name.
    /// * `buffer_name` - name of a buffer, if this is an empty string,
    ///     the current buffer is returned (buffer displayed by current
    ///     window); if the name starts with (?i), the search is case
    ///     insensitive.
    /// Returns a Buffer if one is found, otherwise None.
    pub fn buffer_search(
        &self,
        plugin_name: &str,
        buffer_name: &str,
    ) -> Option<Buffer> {
        let buffer_search = self.get().buffer_search.unwrap();

        let plugin_name = CString::new(plugin_name).unwrap_or_default();
        let buffer_name = CString::new(buffer_name).unwrap_or_default();

        let buf_ptr = unsafe {
            buffer_search(plugin_name.as_ptr(), buffer_name.as_ptr())
        };
        if buf_ptr.is_null() {
            None
        } else {
            Some(Buffer::from_ptr(self.ptr, buf_ptr))
        }
    }

    /// Create a new Weechat buffer
    /// * `name` - Name of the new buffer
    /// * `input_cb` - Callback that will be called when something is entered
    ///     into the input bar of the buffer
    /// * `input_data` - Data that will be taken over by weechat and passed to
    ///     the input callback, this data will be freed when the buffer closes
    /// * `close_cb` - Callback that will be called when the buffer is closed.
    /// * `close_cb_data` - Reference to some data that will be passed to the
    ///     close callback.
    pub fn buffer_new<A: Default, B: Default>(
        &self,
        name: &str,
        input_cb: Option<fn(&mut A, Buffer, &str)>,
        input_data: Option<A>,
        close_cb: Option<fn(&B, Buffer)>,
        close_cb_data: Option<B>,
    ) -> Buffer {
        unsafe extern "C" fn c_input_cb<A, B>(
            pointer: *const c_void,
            _data: *mut c_void,
            buffer: *mut t_gui_buffer,
            input_data: *const c_char,
        ) -> c_int {
            let input_data = CStr::from_ptr(input_data).to_str();

            let pointers: &mut BufferPointers<A, B> =
                { &mut *(pointer as *mut BufferPointers<A, B>) };

            let input_data = match input_data {
                Ok(x) => x,
                Err(_) => return WEECHAT_RC_ERROR,
            };

            let buffer = Buffer::from_ptr(pointers.weechat, buffer);
            let data = &mut pointers.input_data;

            if let Some(callback) = pointers.input_cb {
                callback(data, buffer, input_data)
            }

            WEECHAT_RC_OK
        }

        unsafe extern "C" fn c_close_cb<A, B>(
            pointer: *const c_void,
            _data: *mut c_void,
            buffer: *mut t_gui_buffer,
        ) -> c_int {
            // We use from_raw() here so that the box get's freed at the end
            // of this scope.
            let pointers = Box::from_raw(pointer as *mut BufferPointers<A, B>);
            let buffer = Buffer::from_ptr(pointers.weechat, buffer);
            let data = &pointers.close_cb_data;

            if let Some(callback) = pointers.close_cb {
                callback(data, buffer)
            }
            WEECHAT_RC_OK
        }

        // We create a box and use leak to stop rust from freeing our data,
        // we are giving weechat ownership over the data and will free it in
        // the buffer close callback.
        let buffer_pointers = Box::new(BufferPointers::<A, B> {
            weechat: self.ptr,
            input_cb,
            input_data: input_data.unwrap_or_default(),
            close_cb,
            close_cb_data: close_cb_data.unwrap_or_default(),
        });
        let buffer_pointers_ref: &BufferPointers<A, B> =
            Box::leak(buffer_pointers);

        let buf_new = self.get().buffer_new.unwrap();
        let c_name = CString::new(name).unwrap();

        let c_input_cb: Option<WeechatInputCbT> = match input_cb {
            Some(_) => Some(c_input_cb::<A, B>),
            None => None,
        };

        // TODO this can fail, return a Option type
        let buf_ptr = unsafe {
            buf_new(
                self.ptr,
                c_name.as_ptr(),
                c_input_cb,
                buffer_pointers_ref as *const _ as *const c_void,
                ptr::null_mut(),
                Some(c_close_cb::<A, B>),
                buffer_pointers_ref as *const _ as *const c_void,
                ptr::null_mut(),
            )
        };

        Buffer {
            weechat: self.ptr,
            ptr: buf_ptr,
        }
    }
}

pub(crate) type WeechatInputCbT = unsafe extern "C" fn(
    pointer: *const c_void,
    data: *mut c_void,
    buffer: *mut t_gui_buffer,
    input_data: *const c_char,
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
        Nick {
            ptr,
            buf_ptr,
            weechat_ptr,
        }
    }

    /// Get a Weechat object out of the nick.
    fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat_ptr)
    }

    /// Get a string property of the nick.
    /// * `property` - The name of the property to get the value for, this can
    ///     be one of name, color, prefix or prefix_color. If a unknown
    ///     property is requested an empty string is returned.
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
            visible: true,
        }
    }
}

impl Buffer {
    /// Create a high level Buffer object from a C plugin pointer and the
    /// buffer pointer.
    pub(crate) fn from_ptr(
        weechat_ptr: *mut t_weechat_plugin,
        buffer_ptr: *mut t_gui_buffer,
    ) -> Buffer {
        Buffer {
            weechat: weechat_ptr,
            ptr: buffer_ptr,
        }
    }

    /// Get the Weechat plugin object from a Buffer object.
    pub fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat)
    }

    /// Display a message on the buffer.
    pub fn print(&self, message: &str) {
        let weechat = Weechat::from_ptr(self.weechat);
        let printf_date_tags = weechat.get().printf_date_tags.unwrap();

        let c_message = CString::new(message).unwrap_or_default();

        unsafe {
            printf_date_tags(self.ptr, 0, ptr::null(), c_message.as_ptr())
        }
    }

    /// Create and add a new nick to the buffer nicklist. Returns the newly
    /// created nick.
    /// The nick won't be removed from the nicklist if the returned nick is
    /// dropped.
    /// * `nick` - Nick arguments struct for the nick that should be added.
    /// * `group` - Nicklist group that the nick should be added to. If no
    ///     group is provided the nick is added to the root group.
    pub fn add_nick(&self, nick: NickArgs, group: Option<&NickGroup>) -> Nick {
        let weechat = Weechat::from_ptr(self.weechat);

        // TODO this conversions can fail if any of those strings contain a
        // null byte.
        let c_nick = CString::new(nick.name).unwrap();
        let color = CString::new(nick.color).unwrap();
        let prefix = CString::new(nick.prefix).unwrap();
        let prefix_color = CString::new(nick.prefix_color).unwrap();
        let add_nick = weechat.get().nicklist_add_nick.unwrap();

        let group_ptr = match group {
            Some(g) => g.ptr,
            None => ptr::null_mut(),
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
    /// * `parent_group` - Parent group that the group should be added to.
    ///     If no group is provided the group is added to the root group.
    /// Returns the new nicklist group. The group is not removed if the object
    /// is dropped.
    pub fn add_group(
        &self,
        name: &str,
        color: &str,
        visible: bool,
        parent_group: Option<&NickGroup>,
    ) -> NickGroup {
        let weechat = Weechat::from_ptr(self.weechat);
        let add_group = weechat.get().nicklist_add_group.unwrap();

        let c_name = CString::new(name).unwrap();
        let c_color = CString::new(color).unwrap();

        let group_ptr = match parent_group {
            Some(g) => g.ptr,
            None => ptr::null_mut(),
        };

        let group_ptr = unsafe {
            add_group(
                self.ptr,
                group_ptr,
                c_name.as_ptr(),
                c_color.as_ptr(),
                visible as i32,
            )
        };

        NickGroup {
            ptr: group_ptr,
            buf_ptr: self.ptr,
        }
    }

    fn set(&self, property: &str, value: &str) {
        let weechat = Weechat::from_ptr(self.weechat);

        let buffer_set = weechat.get().buffer_set.unwrap();
        let option = CString::new(property).unwrap();
        let value = CString::new(value).unwrap();

        unsafe { buffer_set(self.ptr, option.as_ptr(), value.as_ptr()) };
    }

    fn get_string(&self, property: &str) -> &str {
        let weechat = Weechat::from_ptr(self.weechat);

        let buffer_get = weechat.get().buffer_get_string.unwrap();
        let property = CString::new(property).unwrap();

        unsafe {
            let value = buffer_get(self.ptr, property.as_ptr());
            if value.is_null() {
                ""
            } else {
                CStr::from_ptr(value).to_str().unwrap_or_default()
            }
        }
    }

    /// Get the full name of the buffer.
    pub fn full_name(&self) -> &str {
        self.get_string("full_name")
    }

    /// Get the name of the buffer.
    pub fn name(&self) -> &str {
        self.get_string("name")
    }

    /// Get the plugin name of the plugin that owns this buffer.
    pub fn plugin_name(&self) -> &str {
        self.get_string("plugin")
    }

    /// Hide time for all lines in the buffer.
    pub fn disable_time_for_each_line(&self) {
        self.set("time_for_each_line", "0");
    }

    /// Disable the nicklist for this buffer.
    pub fn disable_nicklist(&self) {
        self.set("nicklist", "0")
    }

    /// Set the title of the buffer.
    /// * `title` - The new title that will be set.
    pub fn set_title(&self, title: &str) {
        self.set("title", title);
    }

    /// Disable logging for this buffer.
    pub fn disable_log(&self) {
        self.set("localvar_set_no_log", "1");
    }
}
