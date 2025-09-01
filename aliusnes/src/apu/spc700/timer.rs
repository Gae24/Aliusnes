#[derive(Clone, Copy)]
pub struct Timer {
    enabled: bool,
    timer_target: u8,
    timer_output: u8,
}

impl Timer {
    pub(crate) fn new() -> Timer {
        Timer {
            enabled: false,
            timer_target: 0,
            timer_output: 0,
        }
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        // A transition from clear to set (0 -> 1) will reset the timer's internal counter and TxOUT to 0.
        if !self.enabled && enabled {}
        self.enabled = enabled;
    }

    pub(crate) fn set_timer_target(&mut self, value: u8) {
        // When enabled via $F1, the 3 timers will internally count at a rate of 8 KHz (timers 0,1) or 64 KHz (timer 2),
        // and when this interval value has been exceeded, they will increment their external counter result ($FD-FF) and begin again.
        self.timer_target = value;
        self.timer_output = value;
    }

    pub(crate) fn timer_output(&self) -> u8 {
        self.timer_output
    }
}
