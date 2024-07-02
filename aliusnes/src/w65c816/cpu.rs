use super::{
    addressing::{Address, AddressingMode},
    functions::do_push,
    regsize::RegSize,
};
use crate::{bus::Bus, utils::int_traits::ManipulateU16};

pub enum Vector {
    Cop,
    Brk,
    Abort,
    Nmi,
    Irq,
    Reset,
}

impl Vector {
    const fn get_addr(self, emu_mode: bool) -> u16 {
        match (self, emu_mode) {
            (Vector::Cop, true) => 0xFFF4,
            (Vector::Cop, false) => 0xFFE4,
            (Vector::Brk | Vector::Irq, true) => 0xFFFE,
            (Vector::Brk, false) => 0xFFE6,
            (Vector::Abort, true) => 0xFFF8,
            (Vector::Abort, false) => 0xFFE8,
            (Vector::Nmi, true) => 0xFFFA,
            (Vector::Nmi, false) => 0xFFEA,
            (Vector::Irq, false) => 0xFFEE,
            (Vector::Reset, _) => 0xFFFC,
        }
    }
}

bitfield!(
    pub struct Status(pub u8) {
        pub carry: bool @ 0,
        pub zero: bool @ 1,
        pub irq_disable: bool @ 2,
        pub decimal: bool @ 3,
        pub index_regs_size: bool @ 4,
        pub a_reg_size: bool @ 5,
        pub overflow: bool @ 6,
        pub negative: bool @ 7,
    }
);

#[cfg(feature = "log")]
pub fn format_status(status: &Status) -> String {
    let mut string = String::with_capacity(8);
    string += if status.negative() { "N" } else { "n" };
    string += if status.overflow() { "O" } else { "o" };
    string += if status.a_reg_size() { "A" } else { "a" };
    string += if status.index_regs_size() { "X" } else { "x" };
    string += if status.decimal() { "D" } else { "d" };
    string += if status.irq_disable() { "I" } else { "i" };
    string += if status.zero() { "Z" } else { "z" };
    string += if status.carry() { "C" } else { "c" };
    string
}

pub struct Cpu {
    pub accumulator: u16,
    pub index_x: u16,
    pub index_y: u16,
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub status: Status,
    pub dpr: u16,
    pub pbr: u8,
    pub dbr: u8,
    pub emulation_mode: bool,
    pub stopped: bool,
    pub waiting_interrupt: bool,
}

impl Cpu {
    pub(super) fn new() -> Self {
        Cpu {
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            stack_pointer: 0x00,
            status: Status(0),
            dpr: 0x00,
            pbr: 0x00,
            dbr: 0x00,
            program_counter: 0x00,
            emulation_mode: true,
            stopped: false,
            waiting_interrupt: false,
        }
    }

    pub fn set_status_register(&mut self, bits: u8) {
        self.status = Status(bits);
        if self.status.index_regs_size() {
            self.index_x &= 0xFF;
            self.index_y &= 0xFF;
        }
    }

    pub fn set_nz<T: RegSize>(&mut self, val: T) {
        self.status.set_negative(val.is_negative());
        self.status.set_zero(val.is_zero());
    }

    pub fn set_accumulator<T: RegSize>(&mut self, val: T) {
        if T::IS_U16 {
            self.accumulator = val.as_u16();
        } else {
            self.accumulator.set_low_byte(val.as_u8());
        }
    }

    pub fn set_index_x<T: RegSize>(&mut self, val: T) {
        if T::IS_U16 {
            self.index_x = val.as_u16();
        } else {
            self.index_x.set_low_byte(val.as_u8());
        }
    }

    pub fn set_index_y<T: RegSize>(&mut self, val: T) {
        if T::IS_U16 {
            self.index_y = val.as_u16();
        } else {
            self.index_y.set_low_byte(val.as_u8());
        }
    }

    pub fn set_emulation_mode(&mut self, val: bool) {
        if val {
            // not supported
            self.status.set_a_reg_size(true);
            self.status.set_index_regs_size(true);
            self.stack_pointer.set_high_byte(0x01);
            self.index_y.set_high_byte(0x00);
            self.index_x.set_high_byte(0x00);
        }
        self.emulation_mode = val;
    }

    pub fn reset<B: Bus>(&mut self, bus: &mut B) {
        self.stopped = false;
        self.waiting_interrupt = false;
        self.set_emulation_mode(true);
        self.dpr = 0;
        self.dbr = 0;
        self.stack_pointer = 0x1FF;
        self.status.set_decimal(false);
        self.status.set_irq_disable(true);
        self.pbr = 0;
        self.program_counter = self.read_bank0(bus, Vector::Reset.get_addr(self.emulation_mode));
    }

    pub fn handle_interrupt<B: Bus>(&mut self, bus: &mut B, interrupt: Vector) {
        if !self.emulation_mode {
            do_push(self, bus, self.pbr);
        }
        do_push(self, bus, self.program_counter);
        do_push(self, bus, self.status.0);
        self.status.set_decimal(false);
        self.status.set_irq_disable(true);
        self.pbr = 0;
        self.program_counter = self.read_bank0(bus, interrupt.get_addr(self.emulation_mode));
    }

    pub fn read_16<B: Bus>(&mut self, bus: &mut B, addr: Address) -> u16 {
        u16::from_le_bytes([
            bus.read_and_tick(addr),
            bus.read_and_tick(addr.wrapping_add(1)),
        ])
    }

    pub fn write_16<B: Bus>(&mut self, bus: &mut B, addr: Address, data: u16) {
        bus.write_and_tick(addr, data.low_byte());
        bus.write_and_tick(addr.wrapping_add(1), data.high_byte());
    }

    pub fn do_rmw<T: RegSize, B: Bus>(
        &mut self,
        bus: &mut B,
        mode: &AddressingMode,
        f: fn(&mut Cpu, T) -> T,
    ) {
        match mode {
            AddressingMode::Direct
            | AddressingMode::DirectX
            | AddressingMode::DirectY
            | AddressingMode::StackRelative => {
                let (data, page) = self.read_from_direct_page::<T, B>(bus, mode);
                let result = f(self, data);
                if T::IS_U16 {
                    self.write_bank0(bus, page, result.as_u16());
                } else {
                    bus.write_and_tick(page.into(), result.as_u8());
                }
            }
            _ => {
                let addr = self.decode_addressing_mode::<true, B>(bus, *mode);
                if T::IS_U16 {
                    let data = self.read_16(bus, addr);
                    let result = f(self, T::from_u16(data)).as_u16();
                    self.write_16(bus, addr, result);
                } else {
                    let data = bus.read_and_tick(addr);
                    let result = f(self, T::from_u8(data)).as_u8();
                    bus.write_and_tick(addr, result);
                }
            }
        }
    }
}
