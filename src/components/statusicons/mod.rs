pub mod all;
pub mod audio;
pub mod network;
pub mod temps;

pub enum Compare {
    LESS,
    GREATER,
}

pub fn threshold<T: PartialOrd, R>(
    value: T,
    values: &[T],
    return_values: Vec<R>,
    default_value: R,
    compare: Compare,
) -> R {
    let mut found_index = None;
    for (index, x) in values.iter().enumerate() {
        let matches = match compare {
            Compare::LESS => value < *x,
            Compare::GREATER => value > *x,
        };

        if matches {
            found_index = Some(index);
            break;
        }
    }

    found_index
        .and_then(|i| return_values.into_iter().nth(i))
        .unwrap_or(default_value)
}
