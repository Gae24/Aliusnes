use super::emu::Emu;

fn brk(emu: &mut Emu, mode: &AddressingMode) {

}

fn ora(emu: &mut Emu, mode: &AddressingMode) {
    let addr = emu.cpu.get_operand_address(mode);
    let data = emu.cpu.mem_read(addr);
}