//Checks if a key is pressed based on key code
//https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
pub fn pressed(key: i32) -> bool {
    let mut _status: bool = false;
    unsafe { _status = winapi::um::winuser::GetAsyncKeyState(key) != 0 }
    return _status;
}
