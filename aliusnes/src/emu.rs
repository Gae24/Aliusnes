use crate::{
    bus::{dma::Dma, system_bus::SystemBus},
    cart::Cart,
    scheduler::{Event, PpuEvent},
    w65c816::W65C816,
};

pub struct Emu {
    w65c816: W65C816<SystemBus>,
    bus: SystemBus,
}

impl Emu {
    pub fn new(cart: Cart) -> Self {
        #[cfg(feature = "log")]
        init_log();

        let mut emu = Emu {
            bus: SystemBus::new(cart),
            w65c816: W65C816::new(),
        };
        emu.reset();
        emu.bus
            .scheduler
            .add_event(Event::Ppu(PpuEvent::NewScanline), 0);
        emu.bus.scheduler.add_event(Event::Apu, 0);
        emu
    }

    pub fn reset(&mut self) {
        self.w65c816.reset(&mut self.bus);
    }

    pub fn step(&mut self) {
        let Emu {
            ref mut bus,
            ref mut w65c816,
            ..
        } = self;

        if bus.dma.enable_channels > 0 {
            Dma::do_dma(bus);
        }
        w65c816.step(bus);

        while let Some((event, time)) = bus.scheduler.pop_event() {
            match event {
                Event::Ppu(ppu_event) => bus.ppu.handle_event(&mut bus.scheduler, ppu_event, time),
                Event::Apu => bus.apu.handle_event(&mut bus.scheduler, time),
            }
        }
    }

    pub fn run_cpu_until_next_event(&mut self) {
        let Emu {
            ref mut bus,
            ref mut w65c816,
            ..
        } = self;

        while bus.scheduler.waiting_for_next_event() {
            if bus.dma.enable_channels > 0 {
                Dma::do_dma(bus);
            }
            w65c816.step(bus);
        }
    }

    pub fn run_frame(&mut self) {
        while !self.frame_ready() {
            self.run_cpu_until_next_event();
            while let Some((event, time)) = self.bus.scheduler.pop_event() {
                match event {
                    Event::Ppu(ppu_event) => {
                        self.bus
                            .ppu
                            .handle_event(&mut self.bus.scheduler, ppu_event, time)
                    }
                    Event::Apu => self.bus.apu.handle_event(&mut self.bus.scheduler, time),
                }
            }
        }
        self.bus.ppu.frame_ready = false;
    }

    pub fn run_for_frames(&mut self, frames: u64) {
        for _ in 0..frames {
            self.run_frame();
        }
    }

    pub fn frame_ready(&self) -> bool {
        self.bus.ppu.frame_ready
    }

    pub fn frame_width(&self) -> usize {
        self.bus.ppu.screen_width
    }

    pub fn frame_height(&self) -> usize {
        self.bus.ppu.screen_height
    }

    pub fn frame(&self) -> &[[u8; 3]] {
        self.bus.ppu.frame_buffer.as_slice()
    }
}

#[cfg(feature = "log")]
fn init_log() {
    use simplelog::{
        ColorChoice, CombinedLogger, Config, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
    };

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
                .set_location_level(log::LevelFilter::Off)
                .build(),
            std::fs::File::create("cpu_trace.log").unwrap(),
        ),
    ])
    .unwrap();
}
