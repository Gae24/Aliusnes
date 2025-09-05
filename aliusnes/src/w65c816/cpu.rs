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
    pub(crate) struct Status(pub u8) {
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

pub(crate) struct Cpu {
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

    pub fn read<B: Bus, T: RegSize>(&mut self, bus: &mut B, addr: Address) -> T {
        if T::IS_U16 {
            let value = u16::from_le_bytes([
                bus.read_and_tick(addr),
                bus.read_and_tick(addr.wrapping_add(1)),
            ]);
            T::from_u16(value)
        } else {
            T::from_u8(bus.read_and_tick(addr))
        }
    }

    pub fn write<B: Bus, T: RegSize>(&mut self, bus: &mut B, addr: Address, data: T) {
        if T::IS_U16 {
            bus.write_and_tick(addr, data.as_u16().low_byte());
            bus.write_and_tick(addr.wrapping_add(1), data.as_u16().high_byte());
        } else {
            bus.write_and_tick(addr, data.as_u8());
        }
    }

    pub fn do_rmw<T: RegSize, B: Bus>(
        &mut self,
        bus: &mut B,
        mode: &AddressingMode,
        f: fn(&mut Cpu, T) -> T,
    ) {
        match mode {
            AddressingMode::Accumulator => {
                let result = f(self, T::from_u16(self.accumulator));
                self.set_accumulator(result);
            }
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
                let data = self.read(bus, addr);
                let result = f(self, data);
                self.write(bus, addr, result);
            }
        }
    }

    pub fn do_jmp<B: Bus>(&mut self, bus: &mut B, mode: AddressingMode) {
        let addr = self.decode_addressing_mode::<false, _>(bus, mode);

        match mode {
            AddressingMode::Absolute | AddressingMode::AbsoluteIndirect => {
                self.program_counter = addr.offset;
            }
            AddressingMode::AbsoluteLong | AddressingMode::AbsoluteIndirectLong => {
                self.program_counter = addr.offset;
                self.pbr = addr.bank;
            }
            AddressingMode::AbsoluteIndirectX => {
                self.program_counter.set_low_byte(bus.read_and_tick(addr));
                self.program_counter
                    .set_high_byte(bus.read_and_tick(addr.wrapping_offset_add(1)));
            }
            _ => unreachable!(),
        }
    }
}
