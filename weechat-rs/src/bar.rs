use core::ptr;
use libc::c_char;
use std::os::raw::c_void;
use weechat_sys::{
    t_gui_bar_item, t_gui_buffer, t_gui_window, t_hashtable, t_weechat_plugin,
};

use crate::{Buffer, LossyCString, Weechat};

struct BarItemCbData<T> {
    callback: fn(&T, &LightBarItem, &Buffer) -> String,
    callback_data: T,
    weechat_ptr: *mut t_weechat_plugin,
}

/// A handle to a bar item. The bar item is automatically removed when the object is
/// dropped.
pub struct BarItem<T> {
    item: LightBarItem,
    _data: Box<BarItemCbData<T>>,
}

/// A handle to a bar item that is passed to callbacks.
pub struct LightBarItem {
    ptr: *mut t_gui_bar_item,
    weechat_ptr: *mut t_weechat_plugin,
}

impl<T> Drop for BarItem<T> {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.item.weechat_ptr);
        let bar_item_remove = weechat.get().bar_item_remove.unwrap();
        unsafe { bar_item_remove(self.item.ptr) };
    }
}

impl Weechat {
    /// Create a new bar item that can be added by a user.
    // TODO: Provide window object, the callback should accept a Window object wrapping a t_gui_window
    pub fn new_bar_item<T>(
        &self,
        name: &str,
        callback: fn(data: &T, item: &LightBarItem, buffer: &Buffer) -> String,
        callback_data: Option<T>,
    ) -> BarItem<T>
    where
        T: Default,
    {
        unsafe extern "C" fn c_item_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            bar_item: *mut t_gui_bar_item,
            _window: *mut t_gui_window,
            buffer: *mut t_gui_buffer,
            _extra_info: *mut t_hashtable,
        ) -> *mut c_char {
            let data: &mut BarItemCbData<T> =
                { &mut *(pointer as *mut BarItemCbData<T>) };
            let callback = data.callback;
            let callback_data = &data.callback_data;
            let buffer = Buffer::from_ptr(data.weechat_ptr, buffer);

            let item = LightBarItem {
                ptr: bar_item,
                weechat_ptr: data.weechat_ptr,
            };

            let ret = callback(&callback_data, &item, &buffer);
            // weechat wants malloc'ed string
            libc::strdup(LossyCString::new(ret).as_ptr())
        }

        let data = Box::new(BarItemCbData::<T> {
            callback,
            callback_data: callback_data.unwrap_or_default(),
            weechat_ptr: self.ptr,
        });

        let data_ref = Box::leak(data);
        let bar_item_new = self.get().bar_item_new.unwrap();

        let bar_item_name = LossyCString::new(name);

        let hook_ptr = unsafe {
            bar_item_new(
                self.ptr,
                bar_item_name.as_ptr(),
                Some(c_item_cb::<T>),
                data_ref as *const _ as *const c_void,
                ptr::null_mut(),
            )
        };

        let hook_data = unsafe { Box::from_raw(data_ref) };

        BarItem {
            item: LightBarItem {
                ptr: hook_ptr,
                weechat_ptr: self.ptr,
            },
            _data: hook_data,
        }
    }

    /// Triggers a bar update to update by calling its callback
    pub fn update_bar_item(&self, name: &str) {
        let bar_item_update = self.get().bar_item_update.unwrap();

        let name = LossyCString::new(name);

        unsafe { bar_item_update(name.as_ptr()) }
    }
}
