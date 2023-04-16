use windows::Win32::System::LibraryLoader::{ GetModuleHandleA, GetProcAddress };
use windows::Win32::System::Diagnostics::Debug::LPTOP_LEVEL_EXCEPTION_FILTER;
use windows::Win32::System::Diagnostics::Debug::SetUnhandledExceptionFilter;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW,MB_OK,MB_TOPMOST};
use windows::{w,s};

use backtrace::Backtrace;
use std::mem::transmute;
use std::ffi::c_void;

type rtlsetunhandledexceptionfilter = unsafe extern "system" fn(filter: LPTOP_LEVEL_EXCEPTION_FILTER) -> LPTOP_LEVEL_EXCEPTION_FILTER;

pub fn init_errorhandler() {
    // get module handle of ntdll
    let ntdll = unsafe { GetModuleHandleA(s!("ntdll.dll")) }.expect("Failed to get ntdll module handle");
    
    // get the unhandledexceptionfilter function
    let rtl_set_unhandled_exception_filter_address =
        unsafe { GetProcAddress(ntdll, s!("RtlSetUnhandledExceptionFilter")).expect("Failed to get SetUnhandledExceptionFilter address") 
    };

    // transmute it
/*     let rtl_set_unhandled_exception_filter: rtlsetunhandledexceptionfilter = unsafe {
        transmute(rtl_set_unhandled_exception_filter_address)
    }; */

    let rtl_set_unhandled_exception_filter: rtlsetunhandledexceptionfilter = unsafe {
        transmute(crate::make_fn!(rtl_set_unhandled_exception_filter_address,LPTOP_LEVEL_EXCEPTION_FILTER,LPTOP_LEVEL_EXCEPTION_FILTER))
    };
    

    let exception_filter: LPTOP_LEVEL_EXCEPTION_FILTER = Some(ExceptionFilter);
    unsafe { rtl_set_unhandled_exception_filter(exception_filter); }
}
/* use std::fs::File;
use std::io::{Write,BufWriter};
use std::write; */
use std::process::exit;

unsafe extern "system" fn ExceptionFilter(_: *const windows::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS) -> i32 {   

    MessageBoxW(None, w!("Lock.rs has crashed :skull:"), w!("Lock.rs error"), MB_OK | MB_TOPMOST);

    exit(0);

    0
}