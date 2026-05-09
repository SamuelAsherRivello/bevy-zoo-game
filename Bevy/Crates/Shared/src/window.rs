pub const DEFAULT_WINDOW_WIDTH: u32 = 1024;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 768;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_window_size_is_standard_xga() {
        assert_eq!(DEFAULT_WINDOW_WIDTH, 1024);
        assert_eq!(DEFAULT_WINDOW_HEIGHT, 768);
    }
}
