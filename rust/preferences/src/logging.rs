use std::fs::OpenOptions;
use std::io::Write;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

static LOG_PATH: OnceLock<String> = OnceLock::new();

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn init(app: &str) {
    let _ = std::fs::create_dir_all("/tmp/churros");
    let path = format!("/tmp/churros/{app}.log");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path);
    let _ = LOG_PATH.set(path);

    // 1. Ignorar SIGPIPE para evitar muerte por descriptores de salida rotos
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }

    // 2. Redirigir stdout (1) y stderr (2) al archivo de log si está abierto
    if let Ok(f) = file {
        use std::os::unix::io::AsRawFd;
        let fd = f.as_raw_fd();
        unsafe {
            libc::dup2(fd, 1);
            libc::dup2(fd, 2);
        }
    }

    log(&format!("inicio pid={}", std::process::id()));

    let path = LOG_PATH.get().cloned().unwrap_or_default();
    std::panic::set_hook(Box::new(move |info| {
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            let _ = writeln!(f, "[{}] PANIC: {info}", now());
        }
    }));
}

pub fn log(msg: &str) {
    if let Some(path) = LOG_PATH.get() {
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "[{}] {msg}", now());
        }
    }
}
