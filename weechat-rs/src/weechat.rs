use weechat_sys::{
    t_weechat_plugin,
    t_gui_buffer,
    WEECHAT_RC_OK,
    WEECHAT_RC_ERROR
};
use std::ffi::{CStr, CString};
use libc::{c_char, c_int};
use std::os::raw::c_void;
use std::ptr;
use buffer::Buffer;

pub struct Weechat {
    inner: *mut t_weechat_plugin,
}

struct BufferPointers<'a, A: 'a, B, C: 'a> {
    weechat: *mut t_weechat_plugin,
    input_cb: Option<fn(&Option<A>, &mut Option<B>, Buffer, &str)>,
    input_data: Option<B>,
    input_data_ref: &'a Option<A>,
    close_cb: Option<fn(&Option<C>, Buffer)>,
    close_cb_data: &'a Option<C>,
}

type WeechatInputCbT = unsafe extern "C" fn(
    pointer: *const c_void,
    data: *mut c_void,
    buffer: *mut t_gui_buffer,
    input_data: *const c_char
) -> c_int;

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
    pub(crate) fn get(&self) -> &t_weechat_plugin {
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

    pub fn buffer_new<A, B, C>(
        &self,
        name: &str,
        input_cb: Option<fn(&Option<A>, &mut Option<B>, Buffer, &str)>,
        input_data_ref: &'static Option<A>,
        input_data: Option<B>,
        close_cb: Option<fn(&Option<C>, Buffer)>,
        close_cb_data: &'static Option<C>,
    ) -> Buffer {
        unsafe extern "C" fn c_input_cb<A, B, C>(
            pointer: *const c_void,
            _data: *mut c_void,
            buffer: *mut t_gui_buffer,
            input_data: *const c_char,
        ) -> c_int {
            let input_data = CStr::from_ptr(input_data).to_str();

            let pointers: &mut BufferPointers<A, B, C> =
                { &mut *(pointer as *mut BufferPointers<A, B, C>) };

            let input_data = match input_data {
                Ok(x) => x,
                Err(_) => return WEECHAT_RC_ERROR,
            };

            let buffer = Buffer::from_ptr(pointers.weechat, buffer);
            let data_ref = pointers.input_data_ref;
            let data = &mut pointers.input_data;

            match pointers.input_cb {
                Some(callback) => callback(data_ref, data, buffer, input_data),
                None => {}
            };

            WEECHAT_RC_OK
        }

        unsafe extern "C" fn c_close_cb<A, B, C>(
            pointer: *const c_void,
            _data: *mut c_void,
            buffer: *mut t_gui_buffer,
        ) -> c_int {
            // We use from_raw() here so that the box get's freed at the end of this scope.
            let pointers = Box::from_raw(pointer as *mut BufferPointers<A, B, C>);
            let buffer = Buffer::from_ptr(pointers.weechat, buffer);

            let data_ref = pointers.close_cb_data;

            match pointers.close_cb {
                Some(callback) => callback(data_ref, buffer),
                None => {}
            };
            WEECHAT_RC_OK
        }

        // We create a box and use leak to stop rust from freeing our data,
        // we are giving weechat ownership over the data and will free it in the buffer close
        // callback.
        let buffer_pointers = Box::new(BufferPointers::<A, B, C> {
            weechat: self.inner,
            input_cb: input_cb,
            input_data: input_data,
            input_data_ref: input_data_ref,
            close_cb: close_cb,
            close_cb_data: close_cb_data,
        });
        let buffer_pointers_ref: &BufferPointers<A, B, C> = Box::leak(buffer_pointers);

        let buf_new = self.get().buffer_new.unwrap();
        let c_name = CString::new(name).unwrap();

        let c_input_cb: Option<WeechatInputCbT> = match input_cb {
                Some(_) => Some(c_input_cb::<A, B, C>),
                None => None
            };

        let buf_ptr = unsafe {
            buf_new(
                self.inner,
                c_name.as_ptr(),
                c_input_cb,
                buffer_pointers_ref as *const _ as *const c_void,
                ptr::null_mut(),
                Some(c_close_cb::<A, B, C>),
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
