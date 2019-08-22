//! Weechat Hook module.
//! Weechat hooks are used for many different things, to create commands, to
//! listen to events on a file descriptor, add completions to weechat, etc.
//! This module contains hook creation methods for the `Weechat` object.

use libc::{c_char, c_int};
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::time::Duration;

use weechat_sys::{t_gui_buffer, t_hook, t_weechat_plugin, WEECHAT_RC_OK};

use crate::{ArgsWeechat, Buffer, LossyCString, ReturnCode, Weechat};

/// Weechat Hook type. The hook is unhooked automatically when the object is
/// dropped.
pub(crate) struct Hook {
    pub(crate) ptr: *mut t_hook,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

impl Drop for Hook {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let unhook = weechat.get().unhook.unwrap();
        unsafe { unhook(self.ptr) };
    }
}

/// Hook for a weechat command, the command is removed when the object is
/// dropped.
pub struct CommandHook<T> {
    _hook: Hook,
    _hook_data: Box<CommandHookData<T>>,
}

struct CommandHookData<T> {
    callback: fn(&T, Buffer, ArgsWeechat),
    callback_data: T,
    weechat_ptr: *mut t_weechat_plugin,
}

/// Setting for the FdHook.
pub enum FdHookMode {
    /// Catch read events.
    Read,
    /// Catch write events.
    Write,
    /// Catch read and write events.
    ReadWrite,
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

/// Hook for a file descriptor, the hook is removed when the object is dropped.
pub struct FdHook<T, F> {
    _hook: Hook,
    _hook_data: Box<FdHookData<T, F>>,
}

struct FdHookData<T, F> {
    callback: fn(&T, fd_object: &mut F),
    callback_data: T,
    fd_object: F,
}

/// Hook for a weechat command, the hook is removed when the object is dropped.
pub struct CommandRunHook<T> {
    _hook: Hook,
    _hook_data: Box<CommandRunHookData<T>>,
}

struct CommandRunHookData<T> {
    callback: fn(&T, Buffer, Cow<str>) -> ReturnCode,
    callback_data: T,
    weechat_ptr: *mut t_weechat_plugin,
}

/// Hook for a signal, the hook is removed when the object is dropped.
pub struct SignalHook<T> {
    _hook: Hook,
    _hook_data: Box<SignalHookData<T>>,
}

struct SignalHookData<T> {
    callback: fn(&T, &Weechat, SignalHookValue) -> ReturnCode,
    callback_data: T,
    weechat_ptr: *mut t_weechat_plugin,
}

/// The type of data returned by a signal
#[derive(Debug)]
pub enum SignalHookValue {
    /// String data
    String(String),
    /// Integer data
    Integer(i32),
    /// Pointer data
    Pointer(*mut c_void),
}

impl SignalHookValue {
    pub(crate) fn from_raw_with_type(
        data_type: &str,
        data: *mut c_void,
    ) -> Option<SignalHookValue> {
        match data_type {
            "string" => unsafe {
                Some(SignalHookValue::String(
                    CStr::from_ptr(data as *const c_char)
                        .to_string_lossy()
                        .into_owned(),
                ))
            },
            "integer" => {
                let data = data as *const c_int;
                if data.is_null() {
                    None
                } else {
                    unsafe { Some(SignalHookValue::Integer(*(data))) }
                }
            }
            "pointer" => Some(SignalHookValue::Pointer(data)),
            _ => None,
        }
    }
}

/// A hook for a timer, the hook will be removed when the object is dropped.
pub struct TimerHook<T> {
    _hook: Hook,
    _hook_data: Box<TimerHookData<T>>,
}

struct TimerHookData<T> {
    callback: fn(&T, &Weechat, i32),
    callback_data: T,
    weechat_ptr: *mut t_weechat_plugin,
}

#[derive(Default)]
/// Description for a weechat command that should will be hooked.
/// The fields of this struct accept the same string formats that are described
/// in the weechat API documentation.
pub struct CommandDescription<'a> {
    /// Name of the command.
    pub name: &'a str,
    /// Description for the command (displayed with `/help command`)
    pub description: &'a str,
    /// Arguments for the command (displayed with `/help command`)
    pub args: &'a str,
    /// Description for the command arguments (displayed with `/help command`)
    pub args_description: &'a str,
    /// Completion template for the command.
    pub completion: &'a str,
}

impl Weechat {
    /// Create a new weechat command. Returns the hook of the command. The
    /// command is unhooked if the hook is dropped.
    pub fn hook_command<T>(
        &self,
        command_info: CommandDescription,
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

        let name = LossyCString::new(command_info.name);
        let description = LossyCString::new(command_info.description);
        let args = LossyCString::new(command_info.args);
        let args_description = LossyCString::new(command_info.args_description);
        let completion = LossyCString::new(command_info.completion);

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
    /// * `fd_object` - An object for wich the file descriptor will be watched
    ///     and the callback called when read or write operations can happen
    ///     on it.
    /// * `mode` - Configure the hook to watch for writes, reads or both on the
    ///     file descriptor.
    /// * `callback` - A function that will be called if a watched event on the
    ///     file descriptor happends.
    /// * `callback_data` - Data that will be passed to the callback every time
    ///     the callback runs. This data will be freed when the hook is
    ///     unhooked.
    pub fn hook_fd<T, F>(
        &self,
        fd_object: F,
        mode: FdHookMode,
        callback: fn(data: &T, fd_object: &mut F),
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
            let fd_object = &mut hook_data.fd_object;

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

    /// Create a timer that will repeatedly fire.
    ///
    /// * `interval` - The delay between calls in milliseconds.
    /// * `align_second` - The alignment on a second. For example, if current time is 09:00, if
    ///     interval = 60000 (60 seconds), and align_second = 60, then timer is called each minute when
    ///     second is 0.
    /// * `max_calls` - The number of calls to timer (if 0, then timer has no end)
    /// * `callback` - A function that will be called when the timer fires, the `remaining` argument
    ///     will be -1 if the timer has no end.
    /// * `callback_data` - Data that will be passed to the callback every time
    ///     the callback runs. This data will be freed when the hook is unhooked.
    pub fn hook_timer<T>(
        &self,
        interval: Duration,
        align_second: i32,
        max_calls: i32,
        callback: fn(data: &T, weechat: &Weechat, remaining: i32),
        callback_data: Option<T>,
    ) -> TimerHook<T>
    where
        T: Default,
    {
        unsafe extern "C" fn c_hook_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            remaining: i32,
        ) -> c_int {
            let hook_data: &mut TimerHookData<T> =
                { &mut *(pointer as *mut TimerHookData<T>) };
            let callback = &hook_data.callback;
            let callback_data = &hook_data.callback_data;

            callback(
                callback_data,
                &Weechat::from_ptr(hook_data.weechat_ptr),
                remaining,
            );

            WEECHAT_RC_OK
        }

        let data = Box::new(TimerHookData::<T> {
            callback,
            callback_data: callback_data.unwrap_or_default(),
            weechat_ptr: self.ptr,
        });

        let data_ref = Box::leak(data);
        let hook_timer = self.get().hook_timer.unwrap();

        let hook_ptr = unsafe {
            hook_timer(
                self.ptr,
                interval.as_millis() as i64,
                align_second,
                max_calls,
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

        TimerHook {
            _hook: hook,
            _hook_data: hook_data,
        }
    }

    /// Hook a command when Weechat runs it.
    ///
    /// * `command` - The command to hook (wildcard `*` is allowed).
    /// * `callback` - A function that will be called when the command is run.
    /// * `callback_data` - Data that will be passed to the callback every time
    ///     the callback runs. This data will be freed when the hook is unhooked.
    pub fn hook_command_run<T>(
        &self,
        command: &str,
        callback: fn(data: &T, buffer: Buffer, command: Cow<str>) -> ReturnCode,
        callback_data: Option<T>,
    ) -> CommandRunHook<T>
    where
        T: Default,
    {
        unsafe extern "C" fn c_hook_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            buffer: *mut t_gui_buffer,
            command: *const std::os::raw::c_char,
        ) -> c_int {
            let hook_data: &mut CommandRunHookData<T> =
                { &mut *(pointer as *mut CommandRunHookData<T>) };
            let callback = hook_data.callback;
            let callback_data = &hook_data.callback_data;

            let buffer = Buffer::from_ptr(hook_data.weechat_ptr, buffer);
            let command = CStr::from_ptr(command).to_string_lossy();

            callback(callback_data, buffer, command) as isize as i32
        }

        let data = Box::new(CommandRunHookData {
            callback,
            callback_data: callback_data.unwrap_or_default(),
            weechat_ptr: self.ptr,
        });

        let data_ref = Box::leak(data);
        let hook_timer = self.get().hook_command_run.unwrap();

        let command = LossyCString::new(command);

        let hook_ptr = unsafe {
            hook_timer(
                self.ptr,
                command.as_ptr(),
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

        CommandRunHook::<T> {
            _hook: hook,
            _hook_data: hook_data,
        }
    }

    /// Hook a signal.
    ///
    /// * `signal` - The signal to hook (wildcard `*` is allowed).
    /// * `callback` - A function that will be called when the signal is received.
    /// * `callback_data` - Data that will be passed to the callback every time
    ///     the callback runs. This data will be freed when the hook is unhooked.
    pub fn hook_signal<T>(
        &self,
        signal: &str,
        callback: fn(
            data: &T,
            weechat: &Weechat,
            signal_value: SignalHookValue,
        ) -> ReturnCode,
        callback_data: Option<T>,
    ) -> SignalHook<T>
    where
        T: Default,
    {
        unsafe extern "C" fn c_hook_cb<T>(
            pointer: *const c_void,
            _data: *mut c_void,
            _signal: *const c_char,
            data_type: *const c_char,
            signal_data: *mut c_void,
        ) -> c_int {
            let hook_data: &mut SignalHookData<T> =
                { &mut *(pointer as *mut SignalHookData<T>) };
            let callback = hook_data.callback;
            let callback_data = &hook_data.callback_data;

            // this cannot contain invalid utf
            let data_type =
                CStr::from_ptr(data_type).to_str().unwrap_or_default();
            if let Some(value) =
                SignalHookValue::from_raw_with_type(data_type, signal_data)
            {
                callback(
                    callback_data,
                    &Weechat::from_ptr(hook_data.weechat_ptr),
                    value,
                ) as i32
            } else {
                WEECHAT_RC_OK
            }
        }

        let data = Box::new(SignalHookData {
            callback,
            callback_data: callback_data.unwrap_or_default(),
            weechat_ptr: self.ptr,
        });

        let data_ref = Box::leak(data);
        let hook_signal = self.get().hook_signal.unwrap();

        let signal = LossyCString::new(signal);

        let hook_ptr = unsafe {
            hook_signal(
                self.ptr,
                signal.as_ptr(),
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

        SignalHook::<T> {
            _hook: hook,
            _hook_data: hook_data,
        }
    }
}
