use std::collections::HashMap;

pub fn return_state<T: Clone + Default>(states: HashMap<&'static str, T>, id: &'static str) -> T {
    states.get(id).cloned().unwrap_or_default()
}
