use std::sync::Arc;

pub type PopupMenuItemCallback = Arc<dyn Fn() + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemRole {
    Button,
    Separator,
}

pub struct PopupMenuItem {
    pub owner: String,
    pub icon_name: Option<String>,
    pub label: Option<String>,
    pub callback: Option<PopupMenuItemCallback>,
    pub role: ItemRole,
}

impl PopupMenuItem {
    pub fn new(owner: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            label: None,
            icon_name: None,
            callback: None,
            role: ItemRole::Button,
        }
    }

    pub fn callback<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(callback) as PopupMenuItemCallback);
        self
    }

    pub fn icon_name(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = Some(icon_name.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn new_separator() -> Self {
        Self {
            owner: String::from("separator"),
            icon_name: None,
            label: None,
            role: ItemRole::Separator,
            callback: None,
        }
    }
}
