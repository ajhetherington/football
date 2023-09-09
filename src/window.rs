pub struct ScreenSize {
    pub width: usize,
    pub height: usize,
}
impl ScreenSize {
    pub fn new() -> Self {
        ScreenSize {
            width: (640),
            height: (480),
        }
    }
}
