
pub mod ffi;

use self::ffi::*;
use super::{Context, Keymap, KeymapCompileFlags, State};
use xcb;
use std::mem;

pub const MIN_MAJOR_XKB_VERSION: u16 = 1;
pub const MIN_MINOR_XKB_VERSION: u16 = 0;


#[repr(C)]
pub enum SetupXkbExtensionFlags {
    /** Do not apply any flags. */
    NoFlags = 0
}

pub fn setup_xkb_extension(connection: &xcb::Connection,
                           major_xkb_version: u16,
                           minor_xkb_version: u16,
                           flags: SetupXkbExtensionFlags,
                           major_xkb_version_out: &mut u16,
                           minor_xkb_version_out: &mut u16,
                           base_event_out: &mut u8,
                           base_error_out: &mut u8) -> bool {
    unsafe {
        xkb_x11_setup_xkb_extension(connection.get_raw_conn(),
                                    major_xkb_version, minor_xkb_version,
                                    mem::transmute(flags),
                                    major_xkb_version_out, minor_xkb_version_out,
                                    base_event_out, base_error_out) != 0
    }
}


pub fn get_core_keyboard_device_id(connection: &xcb::Connection) -> i32 {
    unsafe {
        xkb_x11_get_core_keyboard_device_id(connection.get_raw_conn()) as i32
    }
}


pub fn keymap_new_from_device(context: &Context, connection: &xcb::Connection,
                              device_id: i32, flags: KeymapCompileFlags) -> Keymap {
    unsafe {
        Keymap::from_raw_ptr(
            xkb_x11_keymap_new_from_device(context.get_raw_ptr(),
                                           connection.get_raw_conn(),
                                           device_id, flags)
        )
    }
}


pub fn state_new_from_device(keymap: &Keymap,
                             connection: &xcb::Connection,
                             device_id: i32) -> State {
    unsafe {
        State::from_raw_ptr(
            xkb_x11_state_new_from_device(keymap.get_raw_ptr(), connection.get_raw_conn(), device_id)
        )
    }
}

