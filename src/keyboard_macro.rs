use enigo::{Key, KeyboardControllable};

fn enigo() -> std::sync::MutexGuard<'static, enigo::Enigo> {
    use enigo::Enigo;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    static ENIGO: Lazy<Mutex<Enigo>> = Lazy::new(|| Mutex::new(Enigo::new()));
    ENIGO.lock().unwrap()
}

fn quote(e: &mut std::sync::MutexGuard<'static, enigo::Enigo>) {
    e.key_down(Key::Shift);
    e.key_click(Key::Layout('\''));
    e.key_up(Key::Shift);
}

pub fn helix_change_directory(directory: &str, clipboard: bool) {
    let mut enigo = enigo();
    enigo.key_click(Key::Escape);
    enigo.key_down(Key::Shift);
    enigo.key_click(Key::Layout(';'));
    enigo.key_up(Key::Shift);
    if clipboard {
        paste_restore(&mut enigo, &format!(r#"cd "{directory}""#));
    } else {
        enigo.key_sequence("cd ");
        quote(&mut enigo);
        enigo.key_sequence(directory);
        quote(&mut enigo);
    }
    enigo.key_click(Key::Return)
}

pub fn helix_open_file(file: &str, line: u32, column: u32, clipboard: bool) {
    let mut enigo = enigo();
    enigo.key_click(Key::Escape);
    enigo.key_down(Key::Shift);
    enigo.key_click(Key::Layout(';'));
    enigo.key_up(Key::Shift);
    if clipboard {
        paste_restore(&mut enigo, &format!(r#"o {file}"#));
    } else {
        enigo.key_sequence("o ");
        quote(&mut enigo);
        enigo.key_sequence(file);
        quote(&mut enigo);
    }
    enigo.key_click(Key::Return);
    enigo.key_sequence(&format!("{line}gg")[..]);
    if column > 1 {
        enigo.key_sequence(&format!("{column}l")[..]);
    }
}

pub fn sleep(secs: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(secs))
}

pub fn paste_restore(enigo: &mut enigo::Enigo, msg: &str) {
    let mut clipboard_store = None;
    clipboard_win::with_clipboard_attempts(10, || {
        clipboard_store = ClipboardStore::save().expect("To get clipboard");
        clipboard_win::set(clipboard_win::formats::Unicode, msg).expect("To set clipboard")
    })
    .expect("To open clipboard");
    sleep(0.05);
    enigo.key_down(Key::Control);
    enigo.key_down(Key::Layout('v'));
    enigo.key_up(Key::Layout('v'));
    enigo.key_up(Key::Control);
    sleep(0.05);
    if let Some(store) = clipboard_store.take() {
        clipboard_win::with_clipboard_attempts(10, move || {
            store.restore().expect("To set clipboard")
        })
        .expect("To open clipboard")
    }
}

struct ClipboardStore(clipboard_win::types::c_uint, Vec<u8>);

impl ClipboardStore {
    fn save() -> clipboard_win::SysResult<Option<ClipboardStore>> {
        let mut enum_formats = clipboard_win::EnumFormats::new();
        let Some(format) = enum_formats.next() else {
            return Ok(None);
        };
        let content = clipboard_win::get(clipboard_win::formats::RawData(format))?;
        Ok(Some(ClipboardStore(format, content)))
    }
    fn restore(&self) -> clipboard_win::SysResult<()> {
        clipboard_win::set(clipboard_win::formats::RawData(self.0), self.1.to_vec())
    }
}
