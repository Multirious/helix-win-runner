use std::{
    io::{self, Error as IoError, Result as IoResult},
    mem::{self, MaybeUninit},
    ptr::{addr_of_mut, NonNull},
};
use winapi::{
    ctypes::c_void,
    shared::{
        minwindef::{BOOL, FALSE, HMODULE, LPARAM, TRUE},
        windef::{self, HWND},
    },
    um::{processthreadsapi, winnt, winuser},
};

type WindowHandle = NonNull<windef::HWND__>;

pub struct Window {
    handle: WindowHandle,
    process: Process,
    thread_id: u32,
}

impl Window {
    pub fn open(handle: WindowHandle) -> IoResult<Self> {
        let (process_id, thread_id) = Window::get_process_thread_id(handle);
        let process = Process::open(process_id)?;
        Ok(Window {
            thread_id,
            handle,
            process,
        })
    }

    pub fn get_process_thread_id(handle: WindowHandle) -> (u32, u32) {
        let mut process_id = 0u32;
        // SAFETY: Handle is non-null
        let thread_id =
            unsafe { winuser::GetWindowThreadProcessId(handle.as_ptr(), &mut process_id) };
        (process_id, thread_id)
    }

    pub fn open_foreground_window() -> IoResult<Self> {
        // SAFETY: the call doesn't have dangerous side-effects.
        let handle = NonNull::new(unsafe { winuser::GetForegroundWindow() })
            .ok_or_else(|| IoError::from(io::ErrorKind::NotFound))?;
        // SAFETY: Handle has just been created should be valid
        let (process_id, thread_id) = Window::get_process_thread_id(handle);
        let process = Process::open(process_id)?;
        Ok(Window {
            handle,
            process,
            thread_id,
        })
    }

    pub fn thread_id(&self) -> u32 {
        self.thread_id
    }

    pub fn process(&self) -> &Process {
        &self.process
    }

    pub fn title(&self, capacity: usize) -> IoResult<String> {
        let mut buffer = vec![0; capacity];
        // SAFETY: the handle, module and buffer are all valid.
        let length = unsafe {
            winuser::GetWindowTextW(
                self.handle.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.capacity() as _,
            )
        };
        if length == 0 {
            return last_os_error();
        }

        buffer.truncate(length as usize);
        Ok(String::from_utf16_lossy(&buffer))
    }

    // If the window was previously visible, the return value is false.
    // If the window was previously hidden, the return value is true.

    pub fn show(&self, cmd: i32) -> bool {
        // SAFETY: handle is valid
        let res = unsafe { winuser::ShowWindow(self.handle.as_ptr(), cmd) };
        if res == FALSE {
            return false;
        }
        true
    }

    pub fn set_focus(&self) -> IoResult<()> {
        // SAFETY: Handle is valid
        let prev_window = unsafe { winuser::SetFocus(self.handle.as_ptr()) };

        if prev_window.is_null() {
            return last_os_error();
        }
        Ok(())
    }

    /// Return true if window were brought to foreground
    pub fn set_foreground(&self) -> bool {
        // SAFETY: Handle is valid
        TRUE == unsafe { winuser::SetForegroundWindow(self.handle.as_ptr()) }
    }

    pub fn set_active(&self) -> IoResult<WindowHandle> {
        // SAFETY: Handle is valid
        let res = unsafe { winuser::SetActiveWindow(self.handle.as_ptr()) };
        NonNull::new(res).ok_or_else(IoError::last_os_error)
    }

    pub fn lock_set_foreground(&self, lock: bool) -> IoResult<()> {
        let lock_code = if lock {
            winuser::LSFW_LOCK
        } else {
            winuser::LSFW_UNLOCK
        };
        let res = unsafe { winuser::LockSetForegroundWindow(lock_code) };
        if res == FALSE {
            return last_os_error();
        }
        Ok(())
    }

    pub fn set_position(
        &self,
        insert_after: WindowHandle,
        x: i32,
        y: i32,
        size_x: i32,
        size_y: i32,
        flags: u32,
    ) -> IoResult<()> {
        let res = unsafe {
            winuser::SetWindowPos(
                self.handle.as_ptr(),
                insert_after.as_ptr(),
                x,
                y,
                size_x,
                size_y,
                flags,
            )
        };
        if res == FALSE {
            return last_os_error();
        }
        Ok(())
    }

    /// Short cut through all the window's api bullshit to make this window topmost and focused.
    /// While only partially working.
    /// Doesn't work for minized window.
    pub fn pop_focus(&self) -> IoResult<()> {
        use winuser::{HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW};

        self.set_position(
            NonNull::new(HWND_TOPMOST).unwrap(),
            0,
            0,
            0,
            0,
            SWP_NOSIZE | SWP_NOMOVE,
        )?;
        self.set_position(
            NonNull::new(HWND_NOTOPMOST).unwrap(),
            0,
            0,
            0,
            0,
            SWP_SHOWWINDOW | SWP_NOSIZE | SWP_NOMOVE,
        )?;
        self.set_foreground();
        self.set_focus()?;
        self.set_active()?;
        Ok(())
    }
}

pub struct Process {
    #[allow(dead_code)]
    process_id: u32,
    handle: NonNull<c_void>,
}

impl Process {
    pub fn open(process_id: u32) -> IoResult<Self> {
        let rights = winnt::PROCESS_QUERY_INFORMATION | winnt::PROCESS_VM_READ;
        let process = unsafe { processthreadsapi::OpenProcess(rights, FALSE, process_id) };
        // SAFETY: the call doesn't have dangerous side-effects.
        NonNull::new(process)
            .map(|handle| Self { process_id, handle })
            .ok_or_else(IoError::last_os_error)
    }

    pub fn name(&self, capacity: usize) -> IoResult<String> {
        // EnumProcessModules takes a pointer to an array of HMODULE.
        // We could use a Vec of capacity one to hold the single module,
        // but in memory, a pointer a single item can be seen as a pointer to an array of items.
        // MaybeUninit helps us reserve enough memory for the one item we need.
        let mut module = MaybeUninit::<HMODULE>::uninit();
        let mut size = 0;
        // SAFETY: the pointer is valid and the size is correct.
        let res = unsafe {
            winapi::um::psapi::EnumProcessModules(
                self.handle.as_ptr(),
                module.as_mut_ptr(),
                mem::size_of::<HMODULE>() as u32,
                &mut size,
            )
        };
        if res == FALSE {
            return last_os_error();
        }

        // SAFETY: the call succeeded, so module is initialized.
        let module = unsafe { module.assume_init() };

        let mut buffer = Vec::<u16>::with_capacity(capacity);
        // SAFETY: the handle, module and buffer are all valid.
        let length = unsafe {
            winapi::um::psapi::GetModuleBaseNameW(
                self.handle.as_ptr(),
                module,
                buffer.as_mut_ptr().cast(),
                buffer.capacity() as u32,
            )
        };
        if length == 0 {
            return last_os_error();
        }

        // SAFETY: the call succeeded, length represents characters, and always less then capacity.
        unsafe { buffer.set_len(length as usize) };
        Ok(String::from_utf16_lossy(&buffer))
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        // SAFETY: the handle is valid and non-null.
        unsafe { winapi::um::handleapi::CloseHandle(self.handle.as_mut()) };
    }
}

pub fn attach_thread_input(
    thread_to_attach: u32,
    attach_with_thread: u32,
    attach: bool,
) -> IoResult<()> {
    let res = unsafe {
        winuser::AttachThreadInput(
            thread_to_attach,
            attach_with_thread,
            if attach { TRUE } else { FALSE },
        )
    };
    if res == FALSE {
        return last_os_error();
    }
    Ok(())
}

pub fn get_current_thread_id() -> u32 {
    // SAFETY: the call doesn't have dangerous side-effects.
    unsafe { processthreadsapi::GetCurrentThreadId() }
}

pub fn enumerate_windows() -> IoResult<Vec<HWND>> {
    unsafe extern "system" fn enum_window_call_back(window: HWND, vec: LPARAM) -> BOOL {
        let vec = &mut *(vec as *mut Vec<HWND>);
        vec.push(window);
        TRUE
    }

    let mut vec: Vec<HWND> = vec![];
    let res = unsafe { winuser::EnumWindows(Some(enum_window_call_back), addr_of_mut!(vec) as _) };
    if res == 0 {
        return last_os_error();
    }

    Ok(vec)
}

fn last_os_error<T>() -> IoResult<T> {
    Err(IoError::last_os_error())
}

pub fn get_windows(all: bool) -> IoResult<Vec<(String, String, Window)>> {
    enumerate_windows()?
        .into_iter()
        .filter_map(|window_handle| {
            let window = Window::open(std::ptr::NonNull::new(window_handle).unwrap()).ok()?;
            let process_name = match window.process().name(256) {
                Ok(process_name) => process_name,
                Err(e) => return Some(Err(e)),
            };
            let title = window.title(256).ok()?;
            if all || !(title.contains("Default IME") || title.contains("MSCTFIME UI")) {
                Some(Ok((process_name, title, window)))
            } else {
                None
            }
        })
        .collect()
}
