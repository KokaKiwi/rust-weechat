#![warn(missing_docs)]

//! Weechat Infolist module.

use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr;

use weechat_sys::{t_gui_buffer, t_infolist, t_weechat_plugin};

use crate::{Buffer, LossyCString, Weechat};

/// Weechat Infolist type.
pub struct Infolist {
    pub(crate) ptr: *mut t_infolist,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

impl Drop for Infolist {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let free = weechat.get().infolist_free.unwrap();
        unsafe { free(self.ptr) }
    }
}

impl Weechat {
    /// Get an infolist.
    /// * `name` - The name of the infolist.
    /// * `arguments` - Optional arguments for the infolist. See the weechat
    ///     C API documentation for valid values.
    /// Returns an Infolist object that behaves like a cursor that can be moved
    /// back and forth to access individual Infolist items.
    pub fn infolist_get(
        &self,
        name: &str,
        arguments: &str,
    ) -> Option<Infolist> {
        let name = LossyCString::new(name);
        let arguments = LossyCString::new(arguments);

        let infolist_get = self.get().infolist_get.unwrap();
        let ptr = unsafe {
            infolist_get(
                self.ptr,
                name.as_ptr(),
                ptr::null_mut(),
                arguments.as_ptr(),
            )
        };

        if ptr.is_null() {
            None
        } else {
            Some(Infolist {
                ptr,
                weechat_ptr: self.ptr,
            })
        }
    }
}

impl Infolist {
    /// Move the "cursor" to the next item in an infolist.
    pub fn next(&self) -> bool {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let infolist_next = weechat.get().infolist_next.unwrap();
        let ret = unsafe { infolist_next(self.ptr) };
        ret == 1
    }

    /// Move the "cursor" to the previous item in an infolist.
    pub fn prev(&self) -> bool {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let infolist_prev = weechat.get().infolist_prev.unwrap();
        let ret = unsafe { infolist_prev(self.ptr) };
        ret == 1
    }

    /// Get the list of fields for current infolist item.
    /// Returns a string with the comma separated list of type/name tuple
    /// separated by a colon.
    /// The types are: "i" (integer), "s" (string), "p" (pointer), "b" (buffer),
    /// "t" (time).
    /// Example: "i:my_integer,s:my_string"
    pub fn fields(&self) -> Option<&str> {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let infolist_fields = weechat.get().infolist_fields.unwrap();
        unsafe {
            let ret = infolist_fields(self.ptr);
            if ret.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ret).to_str().unwrap_or_default())
            }
        }
    }

    fn get_pointer(&self, name: &str) -> *mut c_void {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let infolist_pointer = weechat.get().infolist_pointer.unwrap();

        let name = LossyCString::new(name);

        unsafe { infolist_pointer(self.ptr, name.as_ptr()) }
    }

    /// Get the buffer of the current infolist item.
    /// If the infolist item doesn't have a buffer None is returned.
    pub fn get_buffer(&self) -> Option<Buffer> {
        let ptr = self.get_pointer("buffer");

        if ptr.is_null() {
            None
        } else {
            Some(Buffer::from_ptr(self.weechat_ptr, ptr as *mut t_gui_buffer))
        }
    }

    /// Get the value of a string variable in the current infolist item.
    /// * `name` - The variable name of the infolist item.
    pub fn get_string(&self, name: &str) -> Option<&str> {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let infolist_string = weechat.get().infolist_string.unwrap();

        let name = LossyCString::new(name);

        unsafe {
            let ret = infolist_string(self.ptr, name.as_ptr());
            if ret.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ret).to_str().unwrap_or_default())
            }
        }
    }
}
