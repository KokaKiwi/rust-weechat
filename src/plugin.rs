use bindings::{t_weechat_plugin, t_gui_buffer};
use std::ffi::{CString};
use std::ptr;

pub struct WeechatPlugin {
    inner: *mut t_weechat_plugin,
}

impl WeechatPlugin {
    pub fn from_ptr(inner: *mut t_weechat_plugin) -> WeechatPlugin {
        assert!(!inner.is_null());

        WeechatPlugin {
            inner: inner,
        }
    }
}

impl WeechatPlugin {
    #[inline]
    fn get(&self) -> &t_weechat_plugin {
        unsafe {
            &*self.inner
        }
    }

    pub fn log(&self, msg: &str) {
        let log_printf = self.get().log_printf.unwrap();

        let fmt = CString::new("%s").unwrap();
        let msg = CString::new(msg).unwrap();

        unsafe {
            log_printf(fmt.as_ptr(), msg.as_ptr());
        }
    }

    pub fn buffer_new(&self, name: &str) -> *mut t_gui_buffer {
        let buf_new = self.get().buffer_new.unwrap();
        let c_name = CString::new(name).unwrap();
        unsafe {
            buf_new(self.inner, c_name.as_ptr(), None, ptr::null_mut(), ptr::null_mut(), None, ptr::null_mut(), ptr::null_mut())
        }
    }

    pub fn print(&self, msg: &str) {
        let printf_date_tags = self.get().printf_date_tags.unwrap();

        let fmt = CString::new("%s").unwrap();
        let msg = CString::new(msg).unwrap();

        unsafe {
            printf_date_tags(ptr::null_mut(), 0, ptr::null(), fmt.as_ptr(), msg.as_ptr());
        }
    }
}
