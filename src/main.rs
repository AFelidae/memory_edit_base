mod hack_util;

fn main() {
    let process = hack_util::attach("SomeExecutable.exe");
    let module = hack_util::Module::get_module("SomeExecutable.exe", "SomeDllOrExe.dll");

    let some_base_ptr = process.pointer_from_offsets(
        module.m_dw_base as u32,
        vec![0xDEADBEEF, 0xF0, 0x0, 0xCC], //Evaluate multi level pointer
    );

    let offset_from_base_ptr = some_base_ptr + 0x8; //8 bytes ahead of first the multi level pointer

    let _read_value_float = process.read_memory::<f32>(offset_from_base_ptr); //Reads value of memory as a float
    let _read_value_unsigned_int = process.read_memory::<u32>(offset_from_base_ptr); //Reads value of memory as unsigned 32 bit integer

    //Is A key pressed?
    //https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    if hack_util::pressed(0x41) {
        process.write_memory::<f32>(offset_from_base_ptr, 123.0) //Writes 123.0 as float to memory location,
    }
}
