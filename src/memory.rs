use winapi::shared::minwindef::DWORD;
use winapi::{ 
    um::{ 
        winuser::{ 
            FindWindowA, 
            GetWindowThreadProcessId }, 
        winnt::HANDLE,
        processthreadsapi::OpenProcess,
        memoryapi::{ 
            ReadProcessMemory,
            WriteProcessMemory },
        tlhelp32::{ 
            MODULEENTRY32,
            CreateToolhelp32Snapshot,
            Module32First,
            Module32Next } 
    },
    ctypes::c_void };

use std::{
    ptr::null_mut as nullptr,
    ffi::CString };

//////////////////////////////////////////////////
pub static mut PID: DWORD = 0;
pub static mut BASE: DWORD = 0;
pub static mut HAND: HANDLE = nullptr();

pub fn inject(title: &str, module_name: &str) {
    loop {
        find_window(title);
        unsafe {
            if PID != 0 {
                open_process();
                if find_module(module_name) {
                    break;
                }
            }
        }
    }
}
//////////////////////////////////////////////////
pub fn read<T: Default>(address: u32) -> T {
    let mut ret: T = Default::default();
    unsafe {
        ReadProcessMemory(
            HAND, 
            address as *const c_void, 
            &mut ret as *mut T as *mut c_void, 
            std::mem::size_of::<T>(), 
            nullptr()
        );
        return ret;
    }
}

pub fn write<T: Default>(address: u32, mut value: T) {
    unsafe {
        WriteProcessMemory(
            HAND, 
            address as *mut c_void, 
            &mut value as *mut T as *const c_void, 
            std::mem::size_of::<T>(), 
            nullptr()
        );
    }
}
//////////////////////////////////////////////////

fn find_window(title: &str) {
    let _title = CString::new(title).unwrap();
    let wnd_name = _title.as_ptr();

    unsafe {
        let h_wnd = FindWindowA(nullptr(), wnd_name);

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(h_wnd, &mut pid);
        PID = pid;
    }
}

fn open_process() {
    unsafe {
        HAND = OpenProcess(0x1f0ff as DWORD, 0, PID);
    }
}

fn find_module(name: &str) -> bool {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(0x8 as DWORD, PID);

        let mut module = MODULEENTRY32 {
            dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
            th32ModuleID: 0,
            th32ProcessID: 0,
            GlblcntUsage: 0,
            ProccntUsage: 0,
            modBaseAddr: nullptr(),
            modBaseSize: 0,
            hModule: nullptr(),
            szModule: [0; 256],
            szExePath: [0; 260]
        };

        Module32First(snapshot, &mut module);

        loop {
            let _u8slice = &*(&mut module.szModule[..] as *mut [i8] as *mut [u8]);
            let module_name = std::str::from_utf8(_u8slice).unwrap();

            if module_name.find(name) == Some(0) {
                BASE = module.modBaseAddr as DWORD;
                return true;
            }
            if Module32Next(snapshot, &mut module) == 0 {
                return false;
            }
        }
    }
}