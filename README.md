# Simple way to edit memory with rust
 
Simple base I use to get a handle to a process and write to its memory. It can also get what keys are being pressed.

Written all in rust.

#### Example

Example of main.rs thats read and writes to some pointers in memory

```Rust
#[path = "mem_util/input.rs"]
mod input;

#[path = "mem_util/memory.rs"]
mod memory;

fn main() {
    /*for i in 1..100 {
        input::setCursorPos(i, 2*i);
    }*/

    let process = memory::attach("SomeExecutable.exe");
    let module = memory::Module::get_module("SomeExecutable.exe", "SomeDllOrExe.dll");

    //Evaluate multi level pointer for a 32 bit app
    let some_base_ptr =
        process.pointer_from_offsets32(vec![module.m_dw_base + 0xDEADBEEF, 0xF0, 0x0, 0xCC]);

    let offset_from_base_ptr = some_base_ptr + 0x8; //8 bytes ahead of first the multi level pointer

    let _read_value_float = process.read_memory::<f32>(offset_from_base_ptr); //Reads value of memory as a float
    let _read_value_unsigned_int = process.read_memory::<u32>(offset_from_base_ptr); //Reads value of memory as unsigned 32 bit integer

    //Write some NOP instructions to memory, or any other byte sequence
    process.write_memory::<[u8; 4]>(module.m_dw_base + 0x13370, [0x90, 0x90, 0x90, 0x90]);
    //Is A key pressed?
    //https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    if input::pressed(0x41) {
        process.write_memory::<f32>(offset_from_base_ptr, 123.0) //Writes 123.0 as float to memory location,
    }
}
```
