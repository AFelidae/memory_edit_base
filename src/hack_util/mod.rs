use std::mem;
use std::ptr;

#[derive(Clone, Copy)]
pub struct Process {
    m_h_process: winapi::um::winnt::HANDLE,
}

pub struct Module {
    pub m_dw_base: u64,
    pub m_dw_size: u64,
}

impl Process {
    fn new_handle(h_process: winapi::um::winnt::HANDLE) -> Process {
        return Process {
            m_h_process: h_process,
        };
    }

    //Writes to memory at specified address
    pub fn write_memory<T>(self, address: u64, value: T) {
        let mut input = value;
        unsafe {
            winapi::um::memoryapi::WriteProcessMemory(
                self.m_h_process,
                address as *mut _,
                &mut input as *mut _ as *mut _,
                mem::size_of::<T>() as winapi::shared::basetsd::SIZE_T,
                ptr::null_mut(),
            );
        }
    }

    //Reads from memory at specified address and returns value
    pub fn read_memory<T>(self, address: u64) -> T {
        let mut number = unsafe { mem::zeroed() };
        unsafe {
            winapi::um::memoryapi::ReadProcessMemory(
                self.m_h_process,
                address as *const _,
                &mut number as *mut _ as *mut _,
                mem::size_of::<T>() as winapi::shared::basetsd::SIZE_T,
                ptr::null_mut(),
            )
        };
        return number;
    }

    //Evaluates multi level pointer returns result
    pub fn pointer_from_offsets(self, base_address: u32, offsets: Vec<u64>) -> u64 {
        let mut pointer: u64 = base_address as u64;
        for o in offsets.iter() {
            pointer += o;
            pointer = Process::read_memory::<u32>(self, pointer) as u64;
        }
        return pointer;
    }
}

impl Module {
    pub fn get_module(target_process_name: &str, target_module_name: &str) -> Module {
        let dword_pid = get_pid(target_process_name);

        let h_module = unsafe {
            winapi::um::tlhelp32::CreateToolhelp32Snapshot(
                winapi::um::tlhelp32::TH32CS_SNAPMODULE,
                dword_pid,
            )
        };
        if h_module != winapi::um::handleapi::INVALID_HANDLE_VALUE {
            let mut entry: winapi::um::tlhelp32::MODULEENTRY32W = unsafe { mem::zeroed() };
            entry.dwSize = mem::size_of::<winapi::um::tlhelp32::MODULEENTRY32W>() as u32;

            while unsafe { winapi::um::tlhelp32::Module32NextW(h_module, &mut entry) } != 0 {
                let module_name = String::from_utf16_lossy(&entry.szModule);
                if module_name.contains(target_module_name) {
                    unsafe { winapi::um::handleapi::CloseHandle(h_module) };

                    return Module {
                        m_dw_base: entry.modBaseAddr as u64,
                        m_dw_size: entry.modBaseSize as u64,
                    };
                }
            }
        }

        return Module {
            m_dw_base: 0x0,
            m_dw_size: 0x0,
        };
    }
}

//Gets handle to with all_access
pub fn attach(target_process_name: &str) -> Process {
    let dword_pid = get_pid(target_process_name);
    return Process::new_handle(unsafe {
        winapi::um::processthreadsapi::OpenProcess(
            winapi::um::winnt::PROCESS_ALL_ACCESS,
            0,
            dword_pid,
        )
    });
}

//Checks if a key is pressed based on key code
//https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
pub fn pressed(key: i32) -> bool {
    let mut _status: bool = false;
    unsafe { _status = winapi::um::winuser::GetAsyncKeyState(key) != 0 }
    return _status;
}

//Gets a process id from name
fn get_pid(target_name: &str) -> u32 {
    let h_process = unsafe {
        winapi::um::tlhelp32::CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0)
    };

    let mut entry: winapi::um::tlhelp32::PROCESSENTRY32W = unsafe { mem::zeroed() };

    entry.dwSize = mem::size_of::<winapi::um::tlhelp32::PROCESSENTRY32W>() as u32;

    while unsafe { winapi::um::tlhelp32::Process32NextW(h_process, &mut entry) } != 0 {
        let process_name: String = String::from_utf16_lossy(&entry.szExeFile);
        if process_name.contains(target_name) {
            unsafe { winapi::um::handleapi::CloseHandle(h_process) };
            return entry.th32ProcessID;
        }
    }
    return 0;
}
