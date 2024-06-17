use super::{
    addressing::{Address, AddressingMode},
    functions::do_push,
    opcodes::{OpCode, OPCODES},
    regsize::RegSize,
};
use crate::{
    bus::{dma::Dma, Bus},
    utils::int_traits::ManipulateU16,
};

pub enum Vectors {
    Cop,
    Brk,
    Abort,
    Nmi,
    Irq,
    EmuCop,
    EmuAbort,
    EmuNmi,
    EmuReset,
    EmuBrk,
}

impl Vectors {
    pub const fn get_addr(&self) -> u16 {
        match self {
            Vectors::Cop => 0xFFE4,
            Vectors::Brk => 0xFFE6,
            Vectors::Abort => 0xFFE8,
            Vectors::Nmi => 0xFFEA,
            Vectors::Irq => 0xFFEE,
            Vectors::EmuCop => 0xFFF4,
            Vectors::EmuAbort => 0xFFF8,
            Vectors::EmuNmi => 0xFFFA,
            Vectors::EmuReset => 0xFFFC,
            Vectors::EmuBrk => 0xFFFE,
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

fn format_status(status: &Status) -> String {
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
    pub cycles: u32,
}

impl Cpu {
    pub fn new() -> Self {
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
            cycles: 0,
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

    pub fn emulation_mode(&self) -> bool {
        self.emulation_mode
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

    pub fn reset(&mut self, bus: &mut Bus) {
        self.stopped = false;
        self.waiting_interrupt = false;
        self.set_emulation_mode(true);
        self.dpr = 0;
        self.dbr = 0;
        self.stack_pointer = 0x1FF;
        self.status.set_decimal(false);
        self.status.set_irq_disable(true);
        self.pbr = 0;
        self.program_counter = self.read_bank0(bus, Vectors::EmuReset.get_addr());
    }

    pub fn step(&mut self, bus: &mut Bus) -> u32 {
        self.cycles = 0;

        if self.stopped {
            return 0;
        }
        if self.waiting_interrupt {
            if bus.requested_nmi() {
                self.waiting_interrupt = false;
                self.handle_interrupt(bus, Vectors::Nmi);
            } else if !self.status.irq_disable() && bus.requested_irq() {
                self.waiting_interrupt = false;
                self.handle_interrupt(bus, Vectors::Irq);
            } else {
                return 0;
            }
        }

        let op = self.get_imm::<u8>(bus);

        // DMA will take place in the middle of the next instruction, just after its opcode is read from memory.
        // todo a better way that takes account of syncing components
        if bus.dma.enable_channels > 0 {
            self.cycles += Dma::do_dma(bus);
        }

        let opcode = OPCODES[op as usize];
        // log::trace!(
        //     "Instr {} A:{:#06x} X:{:#06x} Y:{:#06x}, PC:{:#06x}, SP:{:#06x}, P:{:#04x} {}",
        //     opcode.mnemonic,
        //     self.accumulator,
        //     self.index_x,
        //     self.index_y,
        //     (self.program_counter - 1),
        //     self.stack_pointer,
        //     self.status.0,
        //     format_status(&self.status)
        // );

        let instr = opcode.function;
        instr(self, bus, opcode.mode);

        self.cycles
    }

    pub fn peek_opcode(&self, bus: &mut Bus) -> OpCode {
        let op = bus.read::<false>(Address::new(self.program_counter, self.pbr)) as usize;
        OPCODES[op]
    }

    pub fn handle_interrupt(&mut self, bus: &mut Bus, interrupt: Vectors) {
        if !self.emulation_mode {
            do_push(self, bus, self.pbr);
        }
        do_push(self, bus, self.program_counter);
        do_push(self, bus, self.status.0);
        self.status.set_decimal(false);
        self.status.set_irq_disable(true);
        self.pbr = 0;
        self.program_counter = self.read_bank0(bus, interrupt.get_addr());
    }

    pub fn read_16(&mut self, bus: &mut Bus, addr: Address) -> u16 {
        bus.read_and_tick(addr) as u16 | (bus.read_and_tick(addr.wrapping_add(1)) as u16) << 8
    }

    pub fn write_16(&mut self, bus: &mut Bus, addr: Address, data: u16) {
        bus.write_and_tick(addr, data.low_byte());
        bus.write_and_tick(addr.wrapping_add(1), data.high_byte());
    }

    pub fn do_write<T: RegSize>(&mut self, bus: &mut Bus, mode: &AddressingMode, val: T) {
        match mode {
            AddressingMode::Direct
            | AddressingMode::DirectX
            | AddressingMode::DirectY
            | AddressingMode::StackRelative => {
                let (_, page) = self.read_from_direct_page::<T>(bus, mode);
                match T::IS_U16 {
                    true => self.write_bank0(bus, page, val.as_u16()),
                    false => bus.write_and_tick(page.into(), val.as_u8()),
                }
            }
            _ => {
                let addr = self.decode_addressing_mode::<true>(bus, *mode);
                match T::IS_U16 {
                    true => self.write_16(bus, addr, val.as_u16()),
                    false => bus.write_and_tick(addr, val.as_u8()),
                }
            }
        }
    }

    pub fn do_rmw<T: RegSize>(
        &mut self,
        bus: &mut Bus,
        mode: &AddressingMode,
        f: fn(&mut Cpu, T) -> T,
    ) {
        match mode {
            AddressingMode::Direct
            | AddressingMode::DirectX
            | AddressingMode::DirectY
            | AddressingMode::StackRelative => {
                let (data, page) = self.read_from_direct_page::<T>(bus, mode);
                let result = f(self, data);
                match T::IS_U16 {
                    true => self.write_bank0(bus, page, result.as_u16()),
                    false => bus.write_and_tick(page.into(), result.as_u8()),
                }
            }
            _ => {
                let addr = self.decode_addressing_mode::<true>(bus, *mode);
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
