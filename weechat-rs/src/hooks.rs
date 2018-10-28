use weechat_sys::{
    t_weechat_plugin,
    t_hook,
};

use weechat::Weechat;
use buffer::Buffer;

/// Weechat Hook type. The hook is unhooked automatically when the object is dropped.
pub struct Hook<T> {
    pub(crate) ptr: *mut t_hook,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
    pub(crate) _hook_data: Box<HookData<T>>
}

impl<T> Drop for Hook<T> {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let unhook = weechat.get().unhook.unwrap();
        unsafe {
            unhook(self.ptr)
        };
    }
}

pub(crate) struct HookData <T> {
    pub(crate) callback: fn(&T, Buffer),
    pub(crate) callback_data: T,
    pub(crate) weechat_ptr: *mut t_weechat_plugin
}

pub struct CommandInfo<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub args: &'a str,
    pub args_description: &'a str,
    pub completion: &'a str,
}

impl<'a> Default for CommandInfo<'a> {
    fn default() -> CommandInfo<'a> {
        CommandInfo {
            name: "",
            description: "",
            args: "",
            args_description: "",
            completion: ""
        }
    }
}
