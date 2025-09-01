use crate::apu::{
    dsp::Dsp,
    spc700::{timer::Timer, Spc700},
};
use crate::bus::{Access, Bus};
use crate::cart::info::Model;
use crate::scheduler::{Event, Scheduler};
use crate::w65c816::addressing::Address;

mod dsp;
pub mod spc700;

#[cfg(feature = "apu")]
static IPL_ROM: &[u8; 0x40] = include_bytes!("spc700/ipl_boot.rom");

#[cfg(not(feature = "apu"))]
static IPL_ROM: &[u8; 0x40] = &[0; 0x40];

const NTSC_MASTER_CLOCK: u128 = 21_477_270;
const PAL_MASTER_CLOCK: u128 = 21_281_370;

const SPC700_CLOCK: u128 = 1_024_000;

const APU_EVENT_PERIOD_NTSC: u64 = ((32 * NTSC_MASTER_CLOCK) / SPC700_CLOCK) as u64;
const APU_EVENT_PERIOD_PAL: u64 = ((32 * PAL_MASTER_CLOCK) / SPC700_CLOCK) as u64;

pub struct Apu {
    spc700: Spc700<ApuBus>,
    bus: ApuBus,
    model: Model,
}

struct ApuBus {
    aram: [u8; 0x10000],
    apuio: [u8; 4],
    cpuio: [u8; 4],
    bootrom_enabled: bool,
    cycles: u64,
    dsp: Dsp,
    timers: [Timer; 3],
}

impl Apu {
    pub fn new(model: Model) -> Apu {
        Apu {
            spc700: Spc700::new(),
            bus: ApuBus {
                aram: [0; 0x10000],
                apuio: [0; 4],
                cpuio: [0; 4],
                bootrom_enabled: true,
                cycles: 0,
                dsp: Dsp::new(),
                timers: [Timer::new(); 3],
            },
            model,
        }
    }

    pub(crate) fn catch_up_to_master(&mut self, time: u64) {
        let Apu {
            ref mut bus,
            ref mut spc700,
            ref model,
        } = self;

        let target_time = match model {
            Model::Ntsc => time as u128 * SPC700_CLOCK / NTSC_MASTER_CLOCK,
            Model::Pal => time as u128 * SPC700_CLOCK / PAL_MASTER_CLOCK,
        };

        while bus.cycles < target_time as u64 {
            spc700.step(bus);
        }
    }

    pub(crate) fn handle_event(&mut self, scheduler: &mut Scheduler, time: u64) {
        self.catch_up_to_master(time);

        let period = match self.model {
            Model::Ntsc => APU_EVENT_PERIOD_NTSC,
            Model::Pal => APU_EVENT_PERIOD_PAL,
        };
        scheduler.add_event(Event::Apu, time + period);
    }
}

impl Access for Apu {
    fn read(&mut self, addr: u16, time: u64) -> Option<u8> {
        self.catch_up_to_master(time);
        Some(self.bus.cpuio[addr as usize & 3])
    }

    fn write(&mut self, addr: u16, data: u8, time: u64) {
        self.catch_up_to_master(time);
        self.bus.apuio[addr as usize & 3] = data;
    }
}

impl ApuBus {
    fn write_control(&mut self, data: u8) {
        for (i, timer) in self.timers.iter_mut().enumerate() {
            timer.set_enabled(data & (1 << i) != 0);
        }

        if (data & (1 << 4)) != 0 {
            self.apuio[0..=1].fill(0);
        }
        if (data & (1 << 5)) != 0 {
            self.apuio[2..=3].fill(0);
        }

        self.bootrom_enabled = (data & 0x80) != 0;
    }

    fn read(&self, addr: u16) -> u8 {
        match addr {
            // write-only area
            0x00F0 | 0x00F1 | 0x00FA..=0x00FC => {
                println!("Attempted to read write-only registers, returning 0");
                0
            }
            0x00F2 => self.dsp.read_dsp_addr(),
            0x00F3 => self.dsp.read(),
            0x00F4..=0x00F7 => self.apuio[addr as usize & 3],
            0x00FD => self.timers[0].timer_output(),
            0x00FE => self.timers[1].timer_output(),
            0x00FF => self.timers[2].timer_output(),
            0xFFC0..=0xFFFF if self.bootrom_enabled => IPL_ROM[addr as usize & 0x3F],
            _ => self.aram[addr as usize],
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x00F0 => println!("Tried to write TEST: {data:#04x}"),
            0x00F1 => self.write_control(data),
            0x00F2 => self.dsp.set_dsp_addr(data),
            0x00F3 => self.dsp.write(data),
            0x00F4..=0x00F7 => {
                self.cpuio[addr as usize & 3] = data;
            }
            0x00FA => self.timers[0].set_timer_target(data),
            0x00FB => self.timers[1].set_timer_target(data),
            0x00FC => self.timers[2].set_timer_target(data),
            // read-only area
            0x00FD..=0x00FF => (),
            _ => self.aram[addr as usize] = data,
        }
    }
}

impl Bus for ApuBus {
    fn peek_at(&self, _addr: Address) -> Option<u8> {
        todo!()
    }

    fn read_and_tick(&mut self, addr: Address) -> u8 {
        self.cycles += 1;
        self.read(addr.offset)
    }

    fn write_and_tick(&mut self, addr: Address, data: u8) {
        self.cycles += 1;
        self.write(addr.offset, data);
    }

    fn add_io_cycles(&mut self, cycles: usize) {
        self.cycles += cycles as u64;
    }

    fn fired_nmi(&mut self) -> bool {
        todo!()
    }

    fn fired_irq(&mut self) -> bool {
        todo!()
    }
}
