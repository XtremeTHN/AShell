use std::{
    env::{home_dir, var},
    path::PathBuf,
};

pub fn colors() -> Option<String> {
    let home = home_dir()?;
    Some(format!("{}/.config/amane/colors.json", home.to_str()?))
}

pub fn pictures() -> Option<PathBuf> {
    let xdg_pictures = var("XDG_PICTURES_DIR");

    match xdg_pictures {
        Ok(dir) => Some(PathBuf::from(dir)),
        Err(e) => {
            log::warn!("Couldn't get xdg pictures dir: {}", e);
            Some(home_dir()?.join("Pictures"))
        }
    }
}
