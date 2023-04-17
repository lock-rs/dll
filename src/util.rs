
pub fn is_debug_mode() -> bool {
    cfg!(debug_assertions)
}

pub unsafe fn CreateConsole() {
    let mut consoleOldProtect: windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS = 0;
    windows::Win32::System::Memory::VirtualProtect(windows::Win32::System::Console::FreeConsole, 1,windows::Win32::System::Memory::PAGE_EXECUTE_READWRITE,consoleOldProtect);

    
}