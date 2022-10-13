
#[cfg(target_os = "windows")]
use windows::{
    Win32::{
        Foundation::{GetLastError, WIN32_ERROR},
        Globalization::GetUserDefaultUILanguage,
        System::Threading::{CreateMutexW, OpenMutexW, SYNCHRONIZATION_ACCESS_RIGHTS},
        UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
    },
    w,
};

/// make sure app running at single-case
#[cfg(target_os = "windows")]
pub fn make_sure_single_case() {
    unsafe {
        let _ = OpenMutexW(
            SYNCHRONIZATION_ACCESS_RIGHTS(0),
            true,
            w!("link.xsa.bs@fuyoo"),
        );
        let WIN32_ERROR(code) = GetLastError();
        if code == 2 {
            // generate lock
            let _ = CreateMutexW(None, true, w!("link.xsa.bs@fuyoo"));
        } else {
            // get local language
            let lang = GetUserDefaultUILanguage();
            // Adapt to Chinese and english
            if lang == 2052u16 {
                MessageBoxW(
                    None,
                    w!("Rd 正在运行中, 请不要重复运行！"),
                    w!("提示"),
                    MB_OK,
                );
            } else {
                MessageBoxW(
                    None,
                    w!("Rd is Running, Please do not run it again!"),
                    w!("Tips"),
                    MB_OK,
                );
            }
            std::process::exit(0);
        }
    }
}

/// make sure app running at single-case
#[cfg(not(target_os = "windows"))]
pub fn make_sure_single_case() {
    //todo: unix system should be complete
}
