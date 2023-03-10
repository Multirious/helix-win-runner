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

pub fn helix_change_directory(directory: &str) {
    let mut enigo = enigo();
    enigo.key_sequence(":cd ");
    quote(&mut enigo);
    enigo.key_sequence(directory);
    quote(&mut enigo);
    enigo.key_click(Key::Return)
}

pub fn helix_open_file(file: &str, line: u32, column: u32) {
    let mut enigo = enigo();
    enigo.key_sequence(":o ");
    quote(&mut enigo);
    enigo.key_sequence(file);
    quote(&mut enigo);
    enigo.key_click(Key::Return);
    enigo.key_sequence(&format!("{line}gg")[..]);
    if column > 1 {
        enigo.key_sequence(&format!("{column}l")[..]);
    }
    enigo.key_click(Key::Return)
}

pub fn sleep(secs: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(secs))
}
