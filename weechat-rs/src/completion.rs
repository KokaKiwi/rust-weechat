use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;

use weechat_sys::{t_gui_buffer, t_gui_completion, t_weechat_plugin};

use crate::hooks::Hook;
use crate::{Buffer, LossyCString, ReturnCode, Weechat};

pub struct Completion {
    weechat_ptr: *mut t_weechat_plugin,
    ptr: *mut t_gui_completion,
}

#[derive(Clone, Copy)]
pub enum CompletionInsertionMethod {
    Sorted,
    Beginning,
    End,
}

impl CompletionInsertionMethod {
    pub(crate) fn value(&self) -> &str {
        match self {
            CompletionInsertionMethod::Sorted => "sort",
            CompletionInsertionMethod::Beginning => "beginning",
            CompletionInsertionMethod::End => "end",
        }
    }
}

impl Completion {
    pub(crate) fn from_raw(
        weechat: *mut t_weechat_plugin,
        completion: *mut t_gui_completion,
    ) -> Completion {
        Completion {
            weechat_ptr: weechat,
            ptr: completion,
        }
    }

    pub fn add(&self, word: &str) {
        self.add_with_options(word, false, CompletionInsertionMethod::Sorted)
    }

    pub fn add_with_options(
        &self,
        word: &str,
        nick: bool,
        method: CompletionInsertionMethod,
    ) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);

        let hook_completion_list_add =
            weechat.get().hook_completion_list_add.unwrap();

        let word = LossyCString::new(word);
        let method = CString::new(method.value()).unwrap();

        unsafe {
            hook_completion_list_add(
                self.ptr,
                word.as_ptr(),
                nick as i32,
                method.as_ptr(),
            );
        }
    }
}

pub struct CompletionHook<T> {
    _hook: Hook,
    _hook_data: Box<CompletionHookData<T>>,
}

struct CompletionHookData<T> {
    callback: fn(&T, Buffer, &str, Completion) -> ReturnCode,
    callback_data: T,
    weechat_ptr: *mut t_weechat_plugin,
}

impl Weechat {
    pub fn hook_completion<T>(
        &self,
        completion_item: &str,
        description: &str,
        callback: fn(
            data: &T,
            buffer: Buffer,
            item: &str,
            completion: Completion,
        ) -> ReturnCode,
        callback_data: Option<T>,
    ) -> CompletionHook<T>
    where
        T: Default,
    {
        unsafe extern "C" fn c_hook_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            completion_item: *const c_char,
            buffer: *mut t_gui_buffer,
            completion: *mut t_gui_completion,
        ) -> c_int {
            let hook_data: &mut CompletionHookData<T> =
                { &mut *(pointer as *mut CompletionHookData<T>) };
            let callback = hook_data.callback;
            let callback_data = &hook_data.callback_data;
            let buffer = Buffer::from_ptr(hook_data.weechat_ptr, buffer);

            let completion_item =
                CStr::from_ptr(completion_item).to_str().unwrap_or_default();

            callback(
                callback_data,
                buffer,
                completion_item,
                Completion::from_raw(hook_data.weechat_ptr, completion),
            ) as i32
        }

        let data = Box::new(CompletionHookData {
            callback,
            callback_data: callback_data.unwrap_or_default(),
            weechat_ptr: self.ptr,
        });

        let data_ref = Box::leak(data);
        let hook_completion = self.get().hook_completion.unwrap();

        let completion_item = LossyCString::new(completion_item);
        let description = LossyCString::new(description);

        let hook_ptr = unsafe {
            hook_completion(
                self.ptr,
                completion_item.as_ptr(),
                description.as_ptr(),
                Some(c_hook_cb::<T>),
                data_ref as *const _ as *const c_void,
                ptr::null_mut(),
            )
        };
        let hook_data = unsafe { Box::from_raw(data_ref) };
        let hook = Hook {
            ptr: hook_ptr,
            weechat_ptr: self.ptr,
        };

        CompletionHook::<T> {
            _hook: hook,
            _hook_data: hook_data,
        }
    }
}
