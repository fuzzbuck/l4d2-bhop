use std::ffi::{c_int, c_void, CStr, CString};
use std::sync::atomic::AtomicBool;
use inputbot::KeybdKey::CapsLockKey;
use proc_mem::Process;
use winapi::um::winuser::{SendMessageW, WM_KEYDOWN, WM_KEYUP};

const PLAYER_OFFSET: u32 = 0x724B58;
const MFLAGS_OFFSET: u32 = 0xF0;
const PROCESS_NAME: &str = "left4dead2.exe";


pub static mut TOGGLED: bool = false;

fn main() {
    let handle = Process::with_name(PROCESS_NAME).expect("run the game first");
    let module = handle.module("client.dll").expect("client.dll not found, wait for the game to load");

    let null: *const i8 = std::ptr::null();
    let title_cstr = CString::new("Left 4 Dead 2 - Direct3D 9").unwrap();
    let l4d2_hwnd = unsafe { winapi::um::winuser::FindWindowA(null, title_cstr.as_ptr()) };
    println!("hwnd: {:?}", l4d2_hwnd);

    let mut player_ptr: u32 = 0;
    let mut m_flags: u32 = 0;

    let mut debounce = 0;

    loop {

        // toggle mechanism
        unsafe {
            if winapi::um::winuser::GetKeyState(0x50) < 0 {
                unsafe {
                    if debounce > 50000 {
                        TOGGLED = !TOGGLED;
                        println!("TOGGLED: {}", TOGGLED);
                        debounce = 0;
                    } else {
                        debounce += 1;
                    }
                }
            }
        }


        if !handle.read_ptr(&mut player_ptr, (module.base_address() as u32 + PLAYER_OFFSET) as usize) {
            panic!("read_ptr failed for player_ptr");
        }

        if !handle.read_ptr(&mut m_flags, player_ptr as usize + MFLAGS_OFFSET as usize) {
            continue;
            // not in game, but in lobby probably
        }


        unsafe {
            if TOGGLED {
                // dbg!(winapi::um::winuser::GetKeyState(0x20));

                if winapi::um::winuser::GetKeyState(0x20) < 0 {
                    if m_flags != 0x80 && m_flags != 0x82 && m_flags != 0x280 &&
                        m_flags != 0x282 {
                        SendMessageW(l4d2_hwnd, WM_KEYDOWN, 0x20, 0x390000);
                    } else if m_flags == 0x80 || m_flags == 0x82 || m_flags == 0x280 ||
                        m_flags == 0x282 {
                        SendMessageW(l4d2_hwnd, WM_KEYUP, 0x20, 0x390000);
                    }
                }
            }
        }

        // sleep
        std::thread::sleep(std::time::Duration::from_micros(0));
    }
}

