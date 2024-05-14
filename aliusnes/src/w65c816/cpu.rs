use super::{functions::do_push, opcodes::OPCODES_MAP, regsize::RegSize, vectors::Vectors};
use crate::{bus::dma::Dma, bus::Bus, utils::int_traits::ManipulateU16};

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
    emulation_mode: bool,
    pub stopped: bool,
    pub waiting_interrupt: bool,
    pub extra_cycles: u32,
}

#[derive(Debug)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Relative,
    RelativeLong,
    Direct,
    DirectX,
    DirectY,
    Indirect,
    IndirectX,
    IndirectY,
    IndirectLong,
    IndirectLongY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    AbsoluteLong,
    AbsoluteLongX,
    AbsoluteIndirect,
    AbsoluteIndirectLong,
    AbsoluteIndirectX,
    AbsoluteJMP,
    AbsoluteLongJSL,
    StackRelative,
    StackRelativeIndirectY,
    StackPEI,
    BlockMove,
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
            extra_cycles: 0,
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
            self.stack_pointer &= 0x01FF;
            self.index_y &= 0xFF;
            self.index_x &= 0xFF;
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
        self.program_counter = self.read_16(bus, 0xFFFC);
    }

    pub fn step(&mut self, bus: &mut Bus) -> u32 {
        self.extra_cycles = 0;

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
            log::warn!("Started DMA");
            self.extra_cycles += Dma::do_dma(bus);
            log::warn!("Terminated DMA");
        }

        let opcode = OPCODES_MAP
            .get(&op)
            .unwrap_or_else(|| panic!("OpCode {:x} is not recognized", op));
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
        instr(self, bus, &opcode.mode);

        self.extra_cycles + (opcode.cycles as u32)
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
        self.program_counter = self.read_16(bus, interrupt.get_interrupt_addr());
    }

    pub fn read_8(&mut self, bus: &mut Bus, addr: u32) -> u8 {
        self.extra_cycles += bus.memory_access_cycles(addr);
        bus.read::<false>(addr)
    }

    pub fn write_8(&mut self, bus: &mut Bus, addr: u32, data: u8) {
        self.extra_cycles += bus.memory_access_cycles(addr);
        bus.write::<false>(addr, data);
    }

    pub fn read_16(&mut self, bus: &mut Bus, addr: u32) -> u16 {
        self.read_8(bus, addr) as u16 | (self.read_8(bus, addr.wrapping_add(1)) as u16) << 8
    }

    pub fn write_16(&mut self, bus: &mut Bus, addr: u32, data: u16) {
        self.write_8(bus, addr, data as u8);
        self.write_8(bus, addr.wrapping_add(1), (data >> 8) as u8);
    }

    fn add_extra_cycles<const WRITE: bool>(&mut self, unindexed: u32, indexed: u32) {
        if WRITE || unindexed >> 8 != indexed >> 8 {
            self.extra_cycles += 1;
        }
    }

    fn get_imm<T: RegSize>(&mut self, bus: &mut Bus) -> T {
        if T::IS_U16 {
            self.extra_cycles += 1;
            let pbr = self.pbr as u16;
            let res = self.read_16(bus, (pbr | self.program_counter) as u32);
            self.program_counter = self.program_counter.wrapping_add(2);
            T::from_u16(res)
        } else {
            let pbr = self.pbr as u16;
            let res = self.read_8(bus, (pbr | self.program_counter) as u32);
            self.program_counter = self.program_counter.wrapping_add(1);
            T::from_u8(res)
        }
    }

    fn get_direct_addr(&mut self, bus: &mut Bus) -> u16 {
        let dpr = self.dpr;
        if dpr as u8 != 0 {
            self.extra_cycles += 1;
        }
        dpr.wrapping_add(self.get_imm::<u8>(bus) as u16)
    }

    fn get_indirect_addr(&mut self, bus: &mut Bus, addr: u16) -> u32 {
        (self.read_16(bus, addr.into()) | self.dbr as u16).into()
    }

    fn get_indirect_long_addr(&mut self, bus: &mut Bus, addr: u32) -> u32 {
        self.read_16(bus, addr) as u32 | (self.read_8(bus, addr.wrapping_add(2)) as u32) << 16
    }

    fn get_absolute_addr(&mut self, bus: &mut Bus) -> u32 {
        (self.dbr as u16 | self.get_imm::<u16>(bus)) as u32
    }

    fn get_absolute_long_addr(&mut self, bus: &mut Bus) -> u32 {
        self.get_imm::<u16>(bus) as u32 | self.get_imm::<u8>(bus) as u32
    }

    fn get_stack_relative_addr(&mut self, bus: &mut Bus) -> u16 {
        self.stack_pointer
            .wrapping_add(self.get_imm::<u8>(bus).into())
    }

    fn get_address<const WRITE: bool>(&mut self, bus: &mut Bus, mode: &AddressingMode) -> u32 {
        match mode {
            AddressingMode::Direct => self.get_direct_addr(bus) as u32,
            AddressingMode::DirectX => (self.get_direct_addr(bus) + self.index_x) as u32,
            AddressingMode::DirectY => (self.get_direct_addr(bus) + self.index_y) as u32,
            AddressingMode::Indirect => {
                let indirect = self.get_direct_addr(bus);
                self.get_indirect_addr(bus, indirect)
            }
            AddressingMode::IndirectX => {
                let indirect = self.get_direct_addr(bus).wrapping_add(self.index_x);
                self.get_indirect_addr(bus, indirect)
            }
            AddressingMode::IndirectY => {
                let indirect = self.get_direct_addr(bus);
                let unindexed = self.get_indirect_addr(bus, indirect);
                let indexed = (unindexed + self.index_y as u32) & 0xFF_FFFF;
                self.add_extra_cycles::<WRITE>(unindexed, indexed);
                indexed
            }
            AddressingMode::IndirectLong => {
                let indirect = self.get_direct_addr(bus) as u32;
                self.get_indirect_long_addr(bus, indirect)
            }
            AddressingMode::IndirectLongY => {
                let indirect = self.get_direct_addr(bus) as u32;
                (self.get_indirect_long_addr(bus, indirect) + self.index_y as u32) & 0xFF_FFFF
            }
            AddressingMode::Absolute => self.get_absolute_addr(bus),
            AddressingMode::AbsoluteX => {
                let unindexed = self.get_absolute_addr(bus);
                let indexed = (unindexed + self.index_x as u32) & 0xFF_FFFF;
                self.add_extra_cycles::<WRITE>(unindexed, indexed);
                indexed
            }
            AddressingMode::AbsoluteY => {
                let unindexed = self.get_absolute_addr(bus);
                let indexed = (unindexed + self.index_y as u32) & 0xFF_FFFF;
                self.add_extra_cycles::<WRITE>(unindexed, indexed);
                indexed
            }
            AddressingMode::AbsoluteLong => self.get_absolute_long_addr(bus),
            AddressingMode::AbsoluteLongX => {
                (self.get_absolute_long_addr(bus) + self.index_x as u32) & 0xFF_FFFF
            }
            AddressingMode::AbsoluteIndirect => self.get_imm::<u16>(bus) as u32,
            AddressingMode::AbsoluteIndirectX => {
                self.pbr as u32 | self.get_imm::<u16>(bus).wrapping_add(self.index_x) as u32
            }
            AddressingMode::StackRelative => self.get_stack_relative_addr(bus).into(),
            AddressingMode::StackRelativeIndirectY => {
                let indirect = self.get_stack_relative_addr(bus);
                (self.get_indirect_addr(bus, indirect) + self.index_y as u32) & 0xFF_FFFF
            }
            AddressingMode::StackPEI => self.get_direct_addr(bus) as u32,
            _ => unreachable!(),
        }
    }

    pub fn get_operand<T: RegSize>(&mut self, bus: &mut Bus, mode: &AddressingMode) -> T {
        match mode {
            AddressingMode::Immediate
            | AddressingMode::Implied
            | AddressingMode::Relative
            | AddressingMode::RelativeLong
            | AddressingMode::AbsoluteJMP
            | AddressingMode::AbsoluteLongJSL
            | AddressingMode::AbsoluteIndirectLong
            | AddressingMode::BlockMove => self.get_imm(bus),
            _ => {
                let addr = self.get_address::<false>(bus, mode);
                if T::IS_U16 {
                    T::from_u16(self.read_16(bus, addr))
                } else {
                    T::from_u8(self.read_8(bus, addr))
                }
            }
        }
    }

    pub fn do_write<T: RegSize>(&mut self, bus: &mut Bus, mode: &AddressingMode, val: T) {
        let addr = self.get_address::<true>(bus, mode);
        if T::IS_U16 {
            self.write_16(bus, addr, val.as_u16());
        } else {
            self.write_8(bus, addr, val.as_u8());
        }
    }

    pub fn do_rmw<T: RegSize>(
        &mut self,
        bus: &mut Bus,
        mode: &AddressingMode,
        f: fn(&mut Cpu, T) -> T,
    ) {
        let addr = self.get_address::<true>(bus, mode);
        if T::IS_U16 {
            let data = self.read_16(bus, addr);
            let result = f(self, T::from_u16(data)).as_u16();
            self.write_16(bus, addr, result);
        } else {
            let data = self.read_8(bus, addr);
            let result = f(self, T::from_u8(data)).as_u8();
            self.write_8(bus, addr, result);
        }
    }
}
