use std::{fs, io, process};
use x11rb::{
    connection::Connection,
    protocol::xproto::*,
    rust_connection::RustConnection,
};

const WIN_CLASS: &str = "elisa";
const STATE_FILE: &str = "/tmp/amberol_state";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !is_process_running(WIN_CLASS)? {
        process::Command::new(WIN_CLASS).spawn()?;
        fs::write(STATE_FILE, "1")?;
        return Ok(());
    }

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let current_state = fs::read_to_string(STATE_FILE).unwrap_or("1".to_string());

    if let Some(win) = find_window(&conn, screen.root, WIN_CLASS)? {
        if current_state.trim() == "1" {
            hide_window(&conn, win)?;
            fs::write(STATE_FILE, "0")?;
        } else {
            show_window(&conn, win)?;
            fs::write(STATE_FILE, "1")?;
        }
    }

    Ok(())
}

fn is_process_running(process_name: &str) -> Result<bool, io::Error> {
    Ok(fs::read_dir("/proc")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter_map(|dir| {
            dir.file_name().to_str().and_then(|s| s.parse::<i32>().ok()).map(|pid| (pid, dir))
        })
        .filter_map(|(pid, dir)| {
            fs::read_to_string(dir.path().join("comm")).ok().map(|comm| (pid, comm.trim().to_string()))
        })
        .any(|(_, comm)| comm == process_name))
}

fn find_window(
    conn: &RustConnection,
    root: Window,
    class: &str,
) -> Result<Option<Window>, Box<dyn std::error::Error>> {
    let tree = conn.query_tree(root)?.reply()?;
    
    for window in tree.children {
        let name = conn.get_property(false, window, AtomEnum::WM_CLASS, AtomEnum::STRING, 0, 1024)?
            .reply()?;
        
        if let Ok(name_str) = String::from_utf8(name.value) {
            if name_str.to_lowercase().contains(&class.to_lowercase()) {
                return Ok(Some(window));
            }
        }
        
        if let Ok(Some(child)) = find_window(conn, window, class) {
            return Ok(Some(child));
        }
    }
    
    Ok(None)
}

fn hide_window(conn: &RustConnection, win: Window) -> Result<(), Box<dyn std::error::Error>> {
    conn.unmap_window(win)?.check()?;
    conn.flush()?;
    Ok(())
}

fn show_window(conn: &RustConnection, win: Window) -> Result<(), Box<dyn std::error::Error>> {
    conn.map_window(win)?.check()?;
    conn.configure_window(
        win,
        &ConfigureWindowAux::new()
            .x(1306)
            .y(53),
    )?.check()?;
    Ok(())
}

