use std::env;
use std::process::{Command, Stdio};
use std::path::Path;

fn command_exists(cmd: &str) -> bool {
    // If a path was provided, check the file directly
    if cmd.contains('/') || cmd.contains('\\') {
        return Path::new(cmd).exists();
    }

    #[cfg(target_family = "unix")]
    {
        Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {}", cmd))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    #[cfg(target_family = "windows")]
    {
        Command::new("where")
            .arg(cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}


pub fn open_with_spec(spec: &str, path: &Path) -> Result<(), String> {
    let mut parts = parse_cmdline(spec);
    if parts.is_empty() {
        return Err("empty command spec".into());
    }
    let cmd = parts.remove(0);
    if !command_exists(&cmd) {
        return Err(format!("command not found: {}", cmd));
    }
    let mut command = Command::new(cmd);
    if !parts.is_empty() {
        command.args(parts);
    }
    command.arg(path);
    command.spawn().map(|_| ()).map_err(|e| e.to_string())
}


pub fn open_path(path: &std::path::Path) -> Result<(),String> {

    if let Some(ext) = path.extension().and_then(|s| s.to_str()){
        let key = format!("FILE_PICKER_EXT_{}", ext.to_ascii_lowercase());
        if let Ok(spec) = env::var(&key) {
            let mut parts = parse_cmdline(&spec);
            if !parts.is_empty() {
                let cmd = parts.remove(0);
                let args = parts;
                if command_exists(&cmd) {
                    match Command::new(cmd).args(&args).arg(path).spawn() {
                        Ok(_) => return Ok(()),
                        Err(e) => eprintln!("FILE_PICKER_EXT spawn failed for {}: {}", key, e),
                    }
                } else {
                    // Command not found; skip and try fallbacks (no noisy spawn error)
                    eprintln!("FILE_PICKER_EXT {} command not found: {}", key, cmd);
                }
            }
        }
    }

   #[cfg(target_os = "macos")]
   {
        if let Ok(app) = env::var("FILE_PICKER_APP") {
            return Command::new("open")
                .args(["-a", &app])
                .arg(path)
                .spawn()
                .map(|_| ())
                .map_err(|e| e.to_string());
        }
        if let Ok(bundle) = env::var("FILE_PICKER_BUNDLE") {
            return Command::new("open")
                .args(["-b", &bundle])
                .arg(path)
                .spawn()
                .map(|_| ())
                .map_err(|e| e.to_string());
        }
        return Command::new("open")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string());
    }

    // 3) Other OS: xdg-open fallback
    #[cfg(not(target_os = "macos"))]
    {
        return Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string());
    }
    

}

fn parse_cmdline(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_quotes = false;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => in_quotes = !in_quotes,
            '\\' if in_quotes => {
                if let Some(&next) = chars.peek() {
                    buf.push(next);
                    chars.next();
                }
            }
            c if c.is_whitespace() && !in_quotes => {
                if !buf.is_empty() {
                    out.push(std::mem::take(&mut buf));
                }
            }
            c => buf.push(c),
        }
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}
