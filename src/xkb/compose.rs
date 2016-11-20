use std::str;
use std::mem;
use std::ffi::CString;
use xkb::ffi::*;
use super::{Context, Keysym};


pub type CompileFlags = xkb_compose_compile_flags;
pub const COMPILE_NO_FLAGS: CompileFlags = XKB_COMPOSE_COMPILE_NO_FLAGS;

pub type Format = xkb_compose_format;
pub const FORMAT_TEXT_V1: Format = XKB_COMPOSE_FORMAT_TEXT_V1;

pub type StateFlags = xkb_compose_state_flags;
pub const STATE_NO_FLAGS: StateFlags = XKB_COMPOSE_STATE_NO_FLAGS;


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum Status {
  Nothing,
  Composing,
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
  pub fn new_from_locale(context: &Context, locale: &str, flags: CompileFlags) -> Result<Table, ()> {
    let locale = CString::new(locale).unwrap();

    unsafe {
      let ptr = xkb_compose_table_new_from_locale(context.get_raw_ptr(), locale.as_ptr(), flags);

      if ptr.is_null() {
        return Err(());
      }

      Ok(Table { ptr: ptr })
    }
  }

  pub fn new_from_buffer<T: AsRef<[u8]>>(context: &Context, locale: &str, buffer: T, format: Format, flags: CompileFlags) -> Result<Table, ()> {
    let buffer = buffer.as_ref();
    let locale = CString::new(locale).unwrap();

    unsafe {
        let ptr = xkb_compose_table_new_from_buffer(context.get_raw_ptr(),
                                                    buffer.as_ptr() as *const _,
                                                    buffer.len() as _,
                                                    locale.as_ptr(),
                                                    mem::transmute(format),
                                                    mem::transmute(flags));

        if ptr.is_null() {
            return Err(());
        }

        Ok(Table { ptr: ptr })
    }
  }

  pub fn state(&mut self, flags: StateFlags) -> State {
    unsafe {
      State {
        ptr: xkb_compose_state_new(self.ptr, flags)
      }
    }
  }
}

impl Drop for Table {
  fn drop(&mut self) {
    unsafe {
      xkb_compose_table_unref(self.ptr)
    }
  }
}


pub struct State {
  ptr: *mut xkb_compose_state,
}

impl State {
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
    unsafe {
      xkb_compose_state_unref(self.ptr)
    }
  }
}
