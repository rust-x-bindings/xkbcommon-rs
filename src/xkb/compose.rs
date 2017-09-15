
use std::str;
use std::mem;
use std::ffi::CString;
use xkb::ffi::compose::*;
use super::{Context, Keysym};

pub type CompileFlags = u32;
pub const COMPILE_NO_FLAGS : CompileFlags = 0;

pub type Format = u32;
pub const FORMAT_TEXT_V1 : Format = 1;

pub type StateFlags = u32;
pub const STATE_NO_FLAGS : StateFlags = 0;


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum Status {
  Nothing       = 0,
  Composing     = 1,
  Composed,
  Cancelled,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum FeedResult {
  Ignored,
  Accepted,
}


pub struct Table {
  ptr: *mut xkb_compose_table,
}

impl Table {

    pub fn new_from_locale(context: &Context, locale: &str, flags: CompileFlags)
            -> Result<Table, ()>
    {
        let locale = CString::new(locale).unwrap();
        let ptr = unsafe {
            xkb_compose_table_new_from_locale(
                context.get_raw_ptr(),
                locale.as_ptr(), flags
            )
        };
        if ptr.is_null() { Err ( () ) }
        else { Ok( Table { ptr: ptr } ) }
    }

    pub fn new_from_buffer<T: AsRef<[u8]>>(context: &Context, buffer: T,
                                           locale: &str, format: Format,
                                           flags: CompileFlags)
            -> Result<Table, ()>
    {
        let buffer = buffer.as_ref();
        let locale = CString::new(locale).unwrap();
        let ptr = unsafe {
            xkb_compose_table_new_from_buffer(
                context.get_raw_ptr(),
                buffer.as_ptr() as *const _, buffer.len() as _,
                locale.as_ptr(), format, flags
            )
        };
        if ptr.is_null() { Err ( () ) }
        else { Ok( Table { ptr: ptr } ) }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        unsafe {
            xkb_compose_table_unref(self.ptr)
        }
    }
}

impl Clone for Table {
    fn clone(&self) -> Table {
        Table {
            ptr: unsafe { xkb_compose_table_ref(self.ptr) }
        }
    }
}


pub struct State {
  ptr: *mut xkb_compose_state,
}

impl State {

    pub fn new (table: &Table, flags: StateFlags) -> State {
        State {
            ptr: unsafe { xkb_compose_state_new(table.ptr, flags) }
        }
    }

    pub fn compose_table(&self) -> Table {
        Table {
            ptr: unsafe {
                xkb_compose_table_ref(
                    xkb_compose_state_get_compose_table(self.ptr)
                )
            }
        }
    }

    pub fn feed(&mut self, keysym: Keysym) -> FeedResult {
        unsafe {
            mem::transmute(xkb_compose_state_feed(self.ptr, keysym))
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            xkb_compose_state_reset(self.ptr);
        }
    }

    pub fn status(&self) -> Status {
        unsafe {
            mem::transmute(xkb_compose_state_get_status(self.ptr))
        }
    }

    pub fn utf8(&self) -> Option<String> {
        let mut buffer = [0u8; 256];

        unsafe {
            match xkb_compose_state_get_utf8(self.ptr, buffer.as_mut_ptr() as *mut _, buffer.len()) {
                0 => None,
                n => Some(str::from_utf8_unchecked(&buffer[.. n as usize]).into())
            }
        }
    }

    pub fn keysym(&self) -> Option<Keysym> {
        unsafe {
            match xkb_compose_state_get_one_sym(self.ptr) {
                super::KEY_NoSymbol => None,
                value               => Some(value)
            }
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe { xkb_compose_state_unref(self.ptr) }
    }
}

impl Clone for State {
    fn clone(&self) -> State {
        State {
            ptr: unsafe { xkb_compose_state_ref(self.ptr) }
        }
    }
}
