use std::error::Error;
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt as _, OsStringExt as _};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use winapi::shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE};
use winapi::um::libloaderapi::{GetModuleFileNameW, LoadLibraryW};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

pub mod proxy;
mod exports;

const DLLS: [&str; 1] = [
    "coldloader.dll"
];

static DLL_PATH: OnceLock<PathBuf> = OnceLock::new();

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(
    module: HMODULE,
    call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            let dll_path = {
                let mut buffer = [0u16; 1024];
                let len = unsafe { GetModuleFileNameW(module, buffer.as_mut_ptr(), buffer.len() as u32) };
                let path = OsString::from_wide(&buffer[..len as usize]).into_string().unwrap();
                let path = Path::new(&path);
                path.parent().unwrap().to_owned()
            };

            DLL_PATH.set(dll_path).ok();

            initialize();

            TRUE
        }
        DLL_PROCESS_DETACH => {
            unsafe { proxy::cleanup_proxied_dll() };
            TRUE
        }
        _ => TRUE,
    }
}

fn initialize() {
    let dll_path = DLL_PATH.get().expect("DLL_PATH not set");

    for dll in DLLS {
        let dll = dll_path.join(dll);
        if dll.exists() {
            let _ = load_dll(&dll);
        }
    }
}

fn load_dll(dll_path: &Path) -> Result<(), Box<dyn Error>> {
    let path_wide: Vec<u16> = dll_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    let lib = unsafe { LoadLibraryW(path_wide.as_ptr()) };
    if lib.is_null() {
        return Err("Failed to load library".into());
    }

    Ok(())
}
