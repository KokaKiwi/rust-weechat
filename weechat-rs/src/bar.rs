use core::ptr;
use libc::c_char;
use std::os::raw::c_void;
use weechat_sys::{
    t_gui_bar_item, t_gui_buffer, t_gui_window, t_hashtable, t_weechat_plugin,
};

use crate::{Buffer, LossyCString, Weechat};

struct BarItemCbData {
    callback: fn(item: &BarItem, buffer: &Buffer) -> String,
    weechat_ptr: *mut t_weechat_plugin,
}

/// A handle to a bar item. The bar item is automatically removed when the object is
/// dropped.
pub struct BarItem {
    ptr: *mut t_gui_bar_item,
    weechat_ptr: *mut t_weechat_plugin,
    _data: Option<Box<BarItemCbData>>,
}

impl Drop for BarItem {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let bar_item_remove = weechat.get().bar_item_remove.unwrap();
        unsafe { bar_item_remove(self.ptr) };
    }
}

impl Weechat {
    // TODO: Provide window object, the callback should accept a Window object wrapping a t_gui_window
    pub fn new_bar_item(
        &self,
        name: &str,
        callback: fn(item: &BarItem, buffer: &Buffer) -> String,
    ) -> BarItem {
        unsafe extern "C" fn c_item_cb(
            pointer: *const c_void,
            _data: *mut c_void,
            bar_item: *mut t_gui_bar_item,
            _window: *mut t_gui_window,
            buffer: *mut t_gui_buffer,
            _extra_info: *mut t_hashtable,
        ) -> *mut c_char {
            let data: &mut BarItemCbData =
                { &mut *(pointer as *mut BarItemCbData) };
            let callback = data.callback;
            let buffer = Buffer::from_ptr(data.weechat_ptr, buffer);

            let item = BarItem {
                ptr: bar_item,
                weechat_ptr: data.weechat_ptr,
                _data: None,
            };

            // weechat wants malloc'ed string
            libc::strdup(LossyCString::new(callback(&item, &buffer)).as_ptr())
        }

        let data = Box::new(BarItemCbData {
            callback,
            weechat_ptr: self.ptr,
        });

        let data_ref = Box::leak(data);
        let bar_item_new = self.get().bar_item_new.unwrap();

        let bar_item_name = LossyCString::new(name);

        let hook_ptr = unsafe {
            bar_item_new(
                self.ptr,
                bar_item_name.as_ptr(),
                Some(c_item_cb),
                data_ref as *const _ as *const c_void,
                ptr::null_mut(),
            )
        };

        let hook_data = unsafe { Box::from_raw(data_ref) };

        BarItem {
            ptr: hook_ptr,
            weechat_ptr: self.ptr,
            _data: Some(hook_data),
        }
    }

    /// Triggers a bar update to update by calling its callback
    pub fn update_bar_item(&self, name: &str) {
        let bar_item_update = self.get().bar_item_update.unwrap();

        let name = LossyCString::new(name);

        unsafe { bar_item_update(name.as_ptr()) }
    }
}
