#![warn(missing_docs)]

//! Main weechat module

use weechat_sys::{
    t_weechat_plugin,
    t_gui_buffer,
    WEECHAT_RC_OK,
    WEECHAT_RC_ERROR
};
use std::ffi::{CStr, CString};
use std::os::unix::io::AsRawFd;
use libc::{c_char, c_int};
use std::os::raw::c_void;
use std::ptr;
use buffer::{Buffer, BufferPointers, WeechatInputCbT};
use hooks::{
    Hook,
    CommandInfo,
    CommandHook,
    CommandHookData,
    FdHook,
    FdHookData,
    FdHookMode
};

/// Main Weechat struct that encapsulates common weechat API functions.
/// It has a similar API as the weechat script API.
pub struct Weechat {
    ptr: *mut t_weechat_plugin,
}

impl Weechat {
    /// Create a Weechat object from a C t_weechat_plugin pointer.
    /// * `ptr` - Pointer of the weechat plugin.
    pub fn from_ptr(ptr: *mut t_weechat_plugin) -> Weechat {
        assert!(!ptr.is_null());

        Weechat {
            ptr: ptr,
        }
    }
}

impl Weechat {
    #[inline]
    pub(crate) fn get(&self) -> &t_weechat_plugin {
        unsafe {
            &*self.ptr
        }
    }

    /// Write a message in WeeChat log file (weechat.log).
    pub fn log(&self, msg: &str) {
        let log_printf = self.get().log_printf.unwrap();

        let fmt = CString::new("%s").unwrap();
        let msg = CString::new(msg).unwrap();

        unsafe {
            log_printf(fmt.as_ptr(), msg.as_ptr());
        }
    }

    /// Create a new Weechat buffer
    /// * `name` - Name of the new buffer
    /// * `input_cb` - Callback that will be called when something is entered into the input bar of
    /// the buffer
    /// * `input_data` - Data that will be taken over by weechat and passed to the input callback,
    /// this data will be freed when the buffer closes
    /// * `close_cb` - Callback that will be called when the buffer is closed.
    /// * `close_cb_data` - Reference to some data that will be passed to the close callback.
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

            match pointers.input_cb {
                Some(callback) => callback(data, buffer, input_data),
                None => {}
            };

            WEECHAT_RC_OK
        }

        unsafe extern "C" fn c_close_cb<A, B>(
            pointer: *const c_void,
            _data: *mut c_void,
            buffer: *mut t_gui_buffer,
        ) -> c_int {
            // We use from_raw() here so that the box get's freed at the end of this scope.
            let pointers = Box::from_raw(pointer as *mut BufferPointers<A, B>);
            let buffer = Buffer::from_ptr(pointers.weechat, buffer);
            let data = &pointers.close_cb_data;

            match pointers.close_cb {
                Some(callback) => callback(data, buffer),
                None => {}
            };
            WEECHAT_RC_OK
        }

        // We create a box and use leak to stop rust from freeing our data,
        // we are giving weechat ownership over the data and will free it in the buffer close
        // callback.
        let buffer_pointers = Box::new(BufferPointers::<A, B> {
            weechat: self.ptr,
            input_cb: input_cb,
            input_data: input_data.unwrap_or_default(),
            close_cb: close_cb,
            close_cb_data: close_cb_data.unwrap_or_default(),
        });
        let buffer_pointers_ref: &BufferPointers<A, B> = Box::leak(buffer_pointers);

        let buf_new = self.get().buffer_new.unwrap();
        let c_name = CString::new(name).unwrap();

        let c_input_cb: Option<WeechatInputCbT> = match input_cb {
                Some(_) => Some(c_input_cb::<A, B>),
                None => None
            };

        let buf_ptr = unsafe {
            buf_new(
                self.ptr,
                c_name.as_ptr(),
                c_input_cb,
                buffer_pointers_ref as *const _ as *const c_void,
                ptr::null_mut(),
                Some(c_close_cb::<A, B>),
                buffer_pointers_ref as *const _ as *const c_void,
                ptr::null_mut()
            )
        };

        let buffer_set = self.get().buffer_set.unwrap();
        let option = CString::new("nicklist").unwrap();
        let value = CString::new("1").unwrap();

        unsafe {
            buffer_set(buf_ptr, option.as_ptr(), value.as_ptr())
        };

        Buffer {
            weechat: self.ptr,
            ptr: buf_ptr
        }
    }

    /// Display a message on the core weechat buffer.
    pub fn print(&self, msg: &str) {
        let printf_date_tags = self.get().printf_date_tags.unwrap();

        let fmt = CString::new("%s").unwrap();
        let msg = CString::new(msg).unwrap();

        unsafe {
            printf_date_tags(ptr::null_mut(), 0, ptr::null(), fmt.as_ptr(), msg.as_ptr());
        }
    }

    /// Create a new weechat command. Returns the hook of the command. The command is unhooked if
    /// the hook is dropped.
    pub fn hook_command<T>(
        &self,
        command_info: CommandInfo,
        callback: fn(data: &T, buffer: Buffer),
        callback_data: Option<T>
    ) -> CommandHook<T> where
    T: Default {

        unsafe extern "C" fn c_hook_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            buffer: *mut t_gui_buffer,
            argc: i32,
            argv: *mut *mut c_char,
            _argv_eol: *mut *mut c_char,
        ) -> c_int {
            let hook_data: &mut CommandHookData<T> =
                { &mut *(pointer as *mut CommandHookData<T>) };
            let buffer = Buffer::from_ptr(hook_data.weechat_ptr, buffer);
            let callback = hook_data.callback;
            let callback_data = &hook_data.callback_data;

            callback(callback_data, buffer);

            WEECHAT_RC_OK
        }

        let name = CString::new(command_info.name).unwrap();
        let description = CString::new(command_info.description).unwrap();
        let args = CString::new(command_info.args).unwrap();
        let args_description = CString::new(command_info.args_description).unwrap();
        let completion = CString::new(command_info.completion).unwrap();

        let data = Box::new(
            CommandHookData {
                callback: callback,
                callback_data: callback_data.unwrap_or_default(),
                weechat_ptr: self.ptr
            }
        );

        let data_ref = Box::leak(data);

        let hook_command = self.get().hook_command.unwrap();
        let hook_ptr = unsafe {
            hook_command(
                self.ptr,
                name.as_ptr(),
                description.as_ptr(),
                args.as_ptr(),
                args_description.as_ptr(),
                completion.as_ptr(),
                Some(c_hook_cb::<T>),
                data_ref as *const _ as *const c_void,
                ptr::null_mut(),
            )
        };
        let hook_data = unsafe { Box::from_raw(data_ref) };
        let hook = Hook { ptr: hook_ptr, weechat_ptr: self.ptr };

        CommandHook::<T> { _hook: hook, _hook_data: hook_data}
    }

    /// Hook an object that can be turned into a raw file descriptor.
    /// Returns the hook. The hook is unhooked if the the object is dropped.
    pub fn hook_fd<T, F>(
        &self,
        fd_object: F,
        target: FdHookMode,
        callback: fn(data: &T, fd_object: &F),
        callback_data: Option<T>
    ) -> FdHook<T, F> where
    T: Default,
    F: AsRawFd {
        unsafe extern "C" fn c_hook_cb<T, F>(
            pointer: *const c_void,
            _data: *mut c_void,
            _fd: i32,
        ) -> c_int {
            let hook_data: &mut FdHookData<T, F> =
                { &mut *(pointer as *mut FdHookData<T, F>) };
            let callback = hook_data.callback;
            let callback_data = &hook_data.callback_data;
            let fd_object = &hook_data.fd_object;

            callback(callback_data, fd_object);

            WEECHAT_RC_OK
        }

        let fd = fd_object.as_raw_fd();

        let data = Box::new(
            FdHookData {
                callback: callback,
                callback_data: callback_data.unwrap_or_default(),
                fd_object
            }
        );

        let data_ref = Box::leak(data);

        let hook_fd = self.get().hook_fd.unwrap();

        let read = match target {
            FdHookMode::Read => 1,
            FdHookMode::ReadWrite => 1,
            FdHookMode::Write => 0
        };

        let write = match target {
            FdHookMode::Read => 0,
            FdHookMode::ReadWrite => 1,
            FdHookMode::Write => 1
        };

        let hook_ptr = unsafe {
            hook_fd(
                self.ptr,
                fd,
                read,
                write,
                0,
                Some(c_hook_cb::<T, F>),
                data_ref as *const _ as *const c_void,
                ptr::null_mut(),
            )
        };
        let hook_data = unsafe { Box::from_raw(data_ref) };
        let hook = Hook { ptr: hook_ptr, weechat_ptr: self.ptr };

        FdHook::<T, F> { _hook: hook, _hook_data: hook_data}
    }
}
