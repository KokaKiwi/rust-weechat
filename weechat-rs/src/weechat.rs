#![warn(missing_docs)]

//! Main weechat module

use weechat_sys::{
    t_weechat_plugin,
    t_gui_buffer,
    WEECHAT_RC_OK,
};
use std::ffi::{CStr, CString};
use std::os::unix::io::AsRawFd;
use libc::{c_char, c_int};
use std::os::raw::c_void;
use std::{ptr, vec};
use buffer::{Buffer};
use hooks::{
    Hook,
    CommandInfo,
    CommandHook,
    CommandHookData,
    FdHook,
    FdHookData,
    FdHookMode
};

/// An iterator over the arguments of a command, yielding a String value for
/// each argument.
pub struct ArgsWeechat {
    iter: vec::IntoIter<String>
}

impl ArgsWeechat {
    /// Create an ArgsWeechat object from the underlying weechat C types.
    /// Expects the strings in argv to be valid utf8, if not invalid UTF-8
    /// sequences are replaced with the replacement character.
    pub fn new(argc: c_int, argv: *mut *mut c_char) -> ArgsWeechat {
        let argc = argc as isize;
        let args: Vec<String> = (0..argc).map(|i| {
            let cstr = unsafe {
                CStr::from_ptr(*argv.offset(i) as *const libc::c_char)
            };

            String::from_utf8_lossy(&cstr.to_bytes().to_vec()).to_string()
        }).collect();
        ArgsWeechat { iter: args.clone().into_iter() }
    }
}

impl Iterator for ArgsWeechat {
    type Item = String;
    fn next(&mut self) -> Option<String> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl ExactSizeIterator for ArgsWeechat {
    fn len(&self) -> usize { self.iter.len() }
}

impl DoubleEndedIterator for ArgsWeechat {
    fn next_back(&mut self) -> Option<String> { self.iter.next_back() }
}

/// Main Weechat struct that encapsulates common weechat API functions.
/// It has a similar API as the weechat script API.
pub struct Weechat {
    pub(crate) ptr: *mut t_weechat_plugin,
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
        callback: fn(data: &T, buffer: Buffer, args: ArgsWeechat),
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
            let args = ArgsWeechat::new(argc, argv);

            callback(callback_data, buffer, args);

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
    /// Returns the hook object.
    /// * `fd_object` - An object for wich the file descriptor will be watched and the
    ///     callback called when read or write operations can happen on it.
    /// * `mode` - Configure the hook to watch for writes, reads or both on the file descriptor.
    /// * `callback` - A function that will be called if a watched event on the file descriptor
    ///     happends.
    /// * `callback_data` - Data that will be passed to the callback every time the callback runs.
    ///     This data will be freed when the hook is unhooked.
    pub fn hook_fd<T, F>(
        &self,
        fd_object: F,
        mode: FdHookMode,
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
        let (read, write) = mode.as_tuple();

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
