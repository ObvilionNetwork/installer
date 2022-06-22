use std::path::PathBuf;

pub fn default_installation_directory(global: bool) -> Option<String> {
    Some(if global {
        if cfg!(windows) {
            "C:\\Program Files\\ObvilionNetwork"
        } else {
            "/usr/local/share/ObvilionNetwork"
        }.to_string()
    } else {
        let home_local = match dirs::data_local_dir() {
            Some(home_local) => home_local,
            None => return None
        }.join(PathBuf::from("ObvilionNetwork"));
        format!("{}", home_local.display())
    })
}
