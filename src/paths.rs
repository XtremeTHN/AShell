
pub fn colors() -> Option<String> {
    let home = std::env::home_dir()?;
    Some(format!("{}/.config/amane/colors.json", home.to_str()?))
}