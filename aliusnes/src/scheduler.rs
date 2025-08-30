use std::mem;

#[derive(Clone, Copy)]
pub enum PpuEvent {
    HDraw,
    HBlankStart,
    NewScanline,
}

#[derive(Clone, Copy)]
pub enum Event {
    Ppu(PpuEvent),
}

impl Event {
    fn index(&self) -> usize {
        match self {
            Event::Ppu(_) => 1,
        }
    }
}

#[derive(Clone, Copy)]
struct PendingEvent {
    event: Event,
    time: u64,
}

pub struct Scheduler {
    pub cycles: u64,
    events: [PendingEvent; 3],
    next_event_id: usize,
}

impl Scheduler {
    pub(crate) fn new() -> Self {
        Scheduler {
            cycles: 0,
            events: [PendingEvent {
                event: Event::Ppu(PpuEvent::HDraw),
                time: u64::MAX,
            }; 3],
            next_event_id: 0,
        }
    }

    fn find_next_event(&mut self) {
        let mut new_id = 0;

        for idx in 1..3 {
            if self.events[idx].time < self.events[new_id].time {
                new_id = idx;
            }
        }
        self.next_event_id = new_id;
    }

    pub(crate) fn add_event(&mut self, event: Event, time: u64) {
        let id = event.index();
        self.events[id].event = event;
        self.events[id].time = time;

        if id == self.next_event_id {
            self.find_next_event();
        } else if self.events[id].time < self.events[self.next_event_id].time {
            self.next_event_id = id;
        }
    }

    pub(crate) fn waiting_for_next_event(&self) -> bool {
        self.cycles < self.events[self.next_event_id].time
    }

    pub(crate) fn pop_event(&mut self) -> Option<(Event, u64)> {
        if self.waiting_for_next_event() {
            return None;
        }

        let id = self.next_event_id;

        self.find_next_event();

        let slot = &mut self.events[id];

        Some((slot.event, mem::replace(&mut slot.time, u64::MAX)))
    }

    pub(crate) fn tick(&mut self, ticks: u64) {
        self.cycles += ticks;
    }
}
