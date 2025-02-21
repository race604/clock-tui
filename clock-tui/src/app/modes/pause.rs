/// Trait for implementing pause/resume functionality
pub trait Pause {
    /// Returns true if the widget is currently paused
    fn is_paused(&self) -> bool;

    /// Pauses the widget
    fn pause(&mut self);

    /// Resumes the widget
    fn resume(&mut self);

    /// Toggles the pause state
    fn toggle_paused(&mut self) {
        if self.is_paused() {
            self.resume();
        } else {
            self.pause();
        }
    }
}
