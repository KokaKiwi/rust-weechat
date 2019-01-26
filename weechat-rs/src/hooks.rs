#![warn(missing_docs)]

use libc::{c_char, c_int};
use std::ffi::CString;
use std::os::raw::c_void;
use std::os::unix::io::AsRawFd;
use std::ptr;

use weechat_sys::{t_gui_buffer, t_hook, t_weechat_plugin, WEECHAT_RC_OK};

use buffer::Buffer;
use weechat::{ArgsWeechat, Weechat};

/// Weechat Hook type. The hook is unhooked automatically when the object is dropped.
pub(crate) struct Hook {
    pub(crate) ptr: *mut t_hook,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

pub struct CommandHook<T> {
    pub(crate) _hook: Hook,
    pub(crate) _hook_data: Box<CommandHookData<T>>,
}

pub enum FdHookMode {
    Read,
    Write,
    ReadWrite,
}

pub struct FdHook<T, F> {
    pub(crate) _hook: Hook,
    pub(crate) _hook_data: Box<FdHookData<T, F>>,
}

pub(crate) struct FdHookData<T, F> {
    pub(crate) callback: fn(&T, fd_object: &F),
    pub(crate) callback_data: T,
    pub(crate) fd_object: F,
}

pub(crate) struct CommandHookData<T> {
    pub(crate) callback: fn(&T, Buffer, ArgsWeechat),
    pub(crate) callback_data: T,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

impl FdHookMode {
    pub(crate) fn as_tuple(&self) -> (i32, i32) {
        let read = match self {
            FdHookMode::Read => 1,
            FdHookMode::ReadWrite => 1,
            FdHookMode::Write => 0,
        };

        let write = match self {
            FdHookMode::Read => 0,
            FdHookMode::ReadWrite => 1,
            FdHookMode::Write => 1,
        };
        (read, write)
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let unhook = weechat.get().unhook.unwrap();
        unsafe { unhook(self.ptr) };
    }
}

#[derive(Default)]
pub struct CommandInfo<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub args: &'a str,
    pub args_description: &'a str,
    pub completion: &'a str,
}

impl Weechat {
    /// Create a new weechat command. Returns the hook of the command. The command is unhooked if
    /// the hook is dropped.
    pub fn hook_command<T>(
        &self,
        command_info: CommandInfo,
        callback: fn(data: &T, buffer: Buffer, args: ArgsWeechat),
        callback_data: Option<T>,
    ) -> CommandHook<T>
    where
        T: Default,
    {
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
        let args_description =
            CString::new(command_info.args_description).unwrap();
        let completion = CString::new(command_info.completion).unwrap();

        let data = Box::new(CommandHookData {
            callback,
            callback_data: callback_data.unwrap_or_default(),
            weechat_ptr: self.ptr,
        });

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
        let hook = Hook {
            ptr: hook_ptr,
            weechat_ptr: self.ptr,
        };

        CommandHook::<T> {
            _hook: hook,
            _hook_data: hook_data,
        }
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
        callback_data: Option<T>,
    ) -> FdHook<T, F>
    where
        T: Default,
        F: AsRawFd,
    {
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

        let data = Box::new(FdHookData {
            callback,
            callback_data: callback_data.unwrap_or_default(),
            fd_object,
        });

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
        let hook = Hook {
            ptr: hook_ptr,
            weechat_ptr: self.ptr,
        };

        FdHook::<T, F> {
            _hook: hook,
            _hook_data: hook_data,
        }
    }
}
