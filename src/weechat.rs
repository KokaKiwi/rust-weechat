use bindings::{t_weechat_plugin, t_gui_buffer, WEECHAT_RC_OK, WEECHAT_RC_ERROR};
use std::ffi::{CStr, CString};
use libc::{c_char, c_int};
use std::os::raw::c_void;
use std::ptr;

pub struct Weechat {
    inner: *mut t_weechat_plugin,
}

pub struct Buffer {
    weechat: *mut t_weechat_plugin,
    ptr: *mut t_gui_buffer
}

struct BufferPointers {
    weechat: *mut t_weechat_plugin,
    input_cb: Option<fn(Buffer, &str)>,
    close_cb: Option<fn(Buffer)>
}

type WeechatInputCbT = unsafe extern "C" fn(
    pointer: *const c_void,
    data: *mut c_void,
    buffer: *mut t_gui_buffer,
    input_data: *const c_char
) -> c_int;

impl Buffer {
    pub fn from_ptr(weechat_ptr: *mut t_weechat_plugin, buffer_ptr: *mut t_gui_buffer) -> Buffer {
        Buffer {
            weechat: weechat_ptr,
            ptr: buffer_ptr
        }
    }

    pub fn print(&self, message: &str) {
        let weechat = Weechat::from_ptr(self.weechat);
        let printf_date_tags = weechat.get().printf_date_tags.unwrap();

        let c_message = CString::new(message).unwrap();

        unsafe {
            printf_date_tags(self.ptr, 0, ptr::null(), c_message.as_ptr())
        }

    }

    pub fn add_nick(&self, nick: &str) {
        let weechat = Weechat::from_ptr(self.weechat);

        let c_nick = CString::new(nick).unwrap();
        let color = CString::new("green").unwrap();
        let add_nick = weechat.get().nicklist_add_nick.unwrap();

        unsafe {
            add_nick(self.ptr, ptr::null_mut(), c_nick.as_ptr(), color.as_ptr(), ptr::null_mut(), ptr::null_mut(), 1);
        }

    }
}

impl Weechat {
    pub fn from_ptr(inner: *mut t_weechat_plugin) -> Weechat {
        assert!(!inner.is_null());

        Weechat {
            inner: inner,
        }
    }
}

impl Weechat {
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

    pub fn buffer_new(&self, name: &str, input_cb: Option<fn (Buffer, &str)>) -> Buffer {
        unsafe extern "C" fn c_input_cb(pointer: *const c_void,
                               _data: *mut c_void,
                               buffer: *mut t_gui_buffer,
                               input_data: *const c_char)
                               -> c_int {
            let input_data = CStr::from_ptr(input_data).to_str();

            let pointers: &mut BufferPointers = { &mut *(pointer as *mut BufferPointers) };

            let input_data = match input_data {
                Ok(x) => x,
                Err(_) => return WEECHAT_RC_ERROR,
            };

            let buffer = Buffer { weechat: pointers.weechat, ptr: buffer };

            match pointers.input_cb {
                Some(callback) => callback(buffer, input_data),
                None => {}
            };

            WEECHAT_RC_OK
        }

        unsafe extern "C" fn c_close_cb(pointer: *const c_void,
                               _data: *mut c_void,
                               buffer: *mut t_gui_buffer)
                               -> c_int {
            // We use from_raw() here so that the box get's freed at the end of this scope.
            let pointers = Box::from_raw(pointer as *mut BufferPointers);
            let buffer = Buffer { weechat: pointers.weechat, ptr: buffer };

            match pointers.close_cb {
                Some(callback) => callback(buffer),
                None => {}
            };
            WEECHAT_RC_OK
        }

        // We create a box and use leak with a static lifetime to stop rust from freeing our data,
        // we are giving weechat ownership over the data and will free it in the buffer close
        // callback.
        let pointers = Box::new(BufferPointers {
            weechat: self.inner,
            input_cb: input_cb,
            close_cb: None
        });
        let pointer_ref: &'static mut BufferPointers = Box::leak(pointers);

        let buf_new = self.get().buffer_new.unwrap();
        let c_name = CString::new(name).unwrap();

        let c_input_cb: Option<WeechatInputCbT> = match input_cb {
                Some(_) => Some(c_input_cb),
                None => None
            };

        let buf_ptr = unsafe {
            buf_new(
                self.inner,
                c_name.as_ptr(),
                c_input_cb,
                pointer_ref as *const _ as *const c_void,
                ptr::null_mut(),
                Some(c_close_cb),
                pointer_ref as *const _ as *const c_void,
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
            weechat: self.inner,
            ptr: buf_ptr
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
