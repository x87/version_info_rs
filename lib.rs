/// Returns a file version composed of four numbers if the given file has the version information.
///
/// Returns `None` if the file does not have the version information or if the version information is invalid.
///
/// ## Example
///
/// ```rust
/// let (a1, a2, a3, a4) = version_info::get_file_version("mylib.dll")?;
/// println!("mylib.dll's version is {}.{}.{}.{}", a1, a2, a3, a4);
/// ```
pub fn get_file_version(file_name: &str) -> Option<(u32, u32, u32, u32)> {
    use winapi::um::winver::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW};

    #[allow(non_snake_case)]
    #[repr(C)]
    struct VS_FIXEDFILEINFO {
        pub dwSignature: u32,
        pub dwStrucVersion: u32,
        pub dwFileVersionMS: u32,
        pub dwFileVersionLS: u32,
        pub dwProductVersionMS: u32,
        pub dwProductVersionLS: u32,
        pub dwFileFlagsMask: u32,
        pub dwFileFlags: u32,
        pub dwFileOS: i32,
        pub dwFileType: i32,
        pub dwFileSubtype: i32,
        pub dwFileDateMS: u32,
        pub dwFileDateLS: u32,
    }

    unsafe {
        let filename = to_wide(file_name);

        let mut handle = 0;
        let size = GetFileVersionInfoSizeW(filename.as_ptr(), &mut handle);

        if size == 0 {
            return None;
        }

        let mut buf = vec![0u8; size as usize];
        let pbuf = buf.as_mut_ptr() as *mut _;

        if GetFileVersionInfoW(filename.as_ptr(), 0, size, pbuf) == 0 {
            return None;
        }

        let mut pinfo: winapi::um::winnt::PVOID = std::ptr::null_mut();
        let mut length = 0;

        let path: Vec<u16> = to_wide("\\");

        if VerQueryValueW(pbuf, path.as_ptr(), &mut pinfo, &mut length) == 0 {
            return None;
        }

        let info = &*(pinfo as *const VS_FIXEDFILEINFO);

        let v1 = info.dwFileVersionMS >> 16 & 0xFFFF;
        let v2 = info.dwFileVersionMS >> 0 & 0xFFFF;
        let v3 = info.dwFileVersionLS >> 16 & 0xFFFF;
        let v4 = info.dwFileVersionLS >> 0 & 0xFFFF;
        return Some((v1, v2, v3, v4));
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::{ffi::OsStr, iter::once, os::windows::prelude::OsStrExt};

    OsStr::new(s).encode_wide().chain(once(0)).collect()
}
