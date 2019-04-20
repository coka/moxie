#![windows_subsystem = "windows"]

use winapi::{
    ctypes::c_int,
    shared::minwindef::HINSTANCE,
    um::{
        winnt::PWSTR,
        winuser::{CreateWindowExW, MessageBoxW},
    },
};

fn main() {
    unsafe {
        // CreateWindowExW(
        //     dwExStyle,
        //     lpClassName,
        //     lpWindowName,
        //     dwStyle,
        //     x,
        //     y,
        //     nWidth,
        //     nHeight,
        //     hWndParent,
        //     hMenu,
        //     hInstance,
        //     lpParam,
        // );
        MessageBoxW(
            std::ptr::null(),
            // szCmdLine,
            // "Title",
            // MB_OK
        );
    }
}
