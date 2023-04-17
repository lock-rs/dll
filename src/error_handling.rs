//== Use ==//
use windows::Win32::System::LibraryLoader::{ GetModuleHandleA, GetProcAddress };
use windows::Win32::System::Diagnostics::Debug::LPTOP_LEVEL_EXCEPTION_FILTER;
use windows::Win32::UI::WindowsAndMessaging::{ MessageBoxW, MB_OK, MB_TOPMOST };
use windows::{ w, s };
use std::mem::transmute;

type RtlsetunhandledexceptionFilter = unsafe extern "system" fn(
    filter: LPTOP_LEVEL_EXCEPTION_FILTER
) -> LPTOP_LEVEL_EXCEPTION_FILTER;

pub fn init_errorhandler() {
    // get module handle of ntdll
    let ntdll = (unsafe { GetModuleHandleA(s!("ntdll.dll")) }).expect(
        "Failed to get ntdll module handle"
    );

    // get the unhandledexception_filter function
    let rtl_set_unhandled_exception_filter_address = unsafe {
        GetProcAddress(ntdll, s!("RtlSetUnhandledExceptionFilter")).expect(
            "Failed to get SetUnhandledexception_filter address"
        )
    };

    let rtl_set_unhandled_exception_filter: RtlsetunhandledexceptionFilter = unsafe {
        transmute(
            crate::make_fn!(
                rtl_set_unhandled_exception_filter_address,
                LPTOP_LEVEL_EXCEPTION_FILTER,
                LPTOP_LEVEL_EXCEPTION_FILTER
            )
        )
    };

    let exception_filter: LPTOP_LEVEL_EXCEPTION_FILTER = Some(exception_filter);
    unsafe {
        rtl_set_unhandled_exception_filter(exception_filter);
    }
}
/* use std::fs::File;
use std::io::{Write,BufWriter};
use std::write; */
use std::process::exit;

unsafe extern "system" fn exception_filter(
    _info: *const windows::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS
) -> i32 {
    MessageBoxW(None, w!("Lock.rs has crashed :skull:"), w!("Lock.rs error"), MB_OK | MB_TOPMOST);

    exit(0)
}