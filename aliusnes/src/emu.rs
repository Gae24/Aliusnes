use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};

use crate::{bus::SystemBus, cart::Cart, w65c816::W65C816};

pub struct Emu {
    w65c816: W65C816<SystemBus>,
    bus: SystemBus,
}

impl Emu {
    pub fn new(cart: Cart) -> Self {
        let mut emu = Emu {
            bus: SystemBus::new(cart),
            w65c816: W65C816::new(),
        };
        emu.reset();
        emu
    }

    pub fn reset(&mut self) {
        self.w65c816.cpu.reset(&mut self.bus);
    }

    pub fn step(&mut self) {
        let Emu {
            ref mut bus,
            ref mut w65c816,
        } = self;
        let ticks = w65c816.step(bus);

        for _ in 0..ticks {
            bus.tick();
        }
    }

    pub fn frame_ready(&self) -> bool {
        self.bus.ppu.frame_ready()
    }

    pub fn frame_width(&self) -> usize {
        self.bus.ppu.screen_width
    }

    pub fn frame_height(&self) -> usize {
        self.bus.ppu.screen_height
    }

    pub fn frame(&self) -> &[u16] {
        self.bus.ppu.frame_buffer.as_slice()
    }

    fn init_log() {
        CombinedLogger::init(vec![
            TermLogger::new(
                log::LevelFilter::Warn,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                log::LevelFilter::Trace,
                ConfigBuilder::new()
                    .set_time_level(log::LevelFilter::Off)
                    .build(),
                std::fs::File::create("cpu_trace.log").unwrap(),
            ),
        ])
        .unwrap();
    }
}
