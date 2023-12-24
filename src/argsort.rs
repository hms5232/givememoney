//! Modify from https://stackoverflow.com/a/69764256

/// Returns the indices that would sort a vector.
///
/// # Examples
///
/// ```rs
/// let input = vec![5, 3, 1, 2, 4];
///
/// assert_eq!(argsort(&input), vec![2, 3, 1, 4, 0])
/// ```
pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

#[cfg(test)]
mod tests {
    use super::argsort;

    #[test]
    fn correct() {
        let input = vec![5, 3, 1, 2, 4];
        assert_eq!(argsort(&input), vec![2, 3, 1, 4, 0])
    }
}
