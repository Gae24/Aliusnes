use crate::apu::spc700::Spc700;
use crate::bus::{Access, Bus};
use crate::cart::info::Model;
use crate::scheduler::{Event, Scheduler};
use crate::w65c816::addressing::Address;

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
    cycles: u64,
}

impl Apu {
    pub fn new(model: Model) -> Apu {
        Apu {
            spc700: Spc700::new(),
            bus: ApuBus {
                aram: [0; 0x10000],
                apuio: [0; 4],
                cpuio: [0; 4],
                cycles: 0,
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
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // write-only area
            0x00F0 | 0x00F1 | 0x00FA..=0x00FC => {
                println!("Attempted to read write-only registers, returning 0");
                0
            }
            // dsp registers
            0x00F2 | 0x00F3 => {
                println!("Attempted to read dsp registers, returning 0");
                0
            }
            0x00F4..=0x00F7 => self.apuio[addr as usize & 3],
            // timer registers
            0x00FD..=0x00FF => {
                println!("Attempted to read timer registers, returning 0");
                0
            }
            _ => self.aram[addr as usize],
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x00F0 => todo!(),
            0x00F1 => todo!(),
            // dsp registers
            0x00F2 | 0x00F3 => println!("Attempted to write dsp registers"),
            0x00F4..=0x00F7 => {
                self.cpuio[addr as usize & 3] = data;
            }
            // timer registers
            0x00FA..=0x00FC => {
                println!("Attempted to write timer registers");
            }
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
