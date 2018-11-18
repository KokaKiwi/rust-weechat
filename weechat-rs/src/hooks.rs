use weechat_sys::{
    t_weechat_plugin,
    t_hook,
};

use weechat::Weechat;
use buffer::Buffer;

/// Weechat Hook type. The hook is unhooked automatically when the object is dropped.
pub(crate) struct Hook {
    pub(crate) ptr: *mut t_hook,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

pub struct CommandHook<T> {
    pub(crate) _hook: Hook,
    pub(crate) _hook_data: Box<CommandHookData<T>>
}

pub enum FdHookMode {
    Read,
    Write,
    ReadWrite
}

pub struct FdHook<T, F> {
    pub(crate) _hook: Hook,
    pub(crate) _hook_data: Box<FdHookData<T, F>>
}

pub(crate) struct FdHookData <T, F> {
    pub(crate) callback: fn(&T, fd_object: &F),
    pub(crate) callback_data: T,
    pub(crate) fd_object: F,
}

pub(crate) struct CommandHookData <T> {
    pub(crate) callback: fn(&T, Buffer),
    pub(crate) callback_data: T,
    pub(crate) weechat_ptr: *mut t_weechat_plugin
}

impl FdHookMode {
    pub(crate) fn as_tuple(&self) -> (i32, i32) {
        let read = match self {
            FdHookMode::Read => 1,
            FdHookMode::ReadWrite => 1,
            FdHookMode::Write => 0
        };

        let write = match self {
            FdHookMode::Read => 0,
            FdHookMode::ReadWrite => 1,
            FdHookMode::Write => 1
        };
        (read, write)
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let unhook = weechat.get().unhook.unwrap();
        unsafe {
            unhook(self.ptr)
        };
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
