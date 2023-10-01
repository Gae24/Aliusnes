use proc_bitfield::bitfield;

bitfield! {
    pub struct Configuration(u8) {
        pub transfer_pattern: u8 @ 0..=2,
        pub address_adjust_mode: u8 @ 3..=4,
        pub h_indirect: bool @ 6,
        pub direction: bool @ 7,
    }
}

pub struct Channel {
    configuration: Configuration,
    b_addr: u8,
    a_addr_or_h_table_addr: u16,
    a_bank_or_h_table_bank: u8,
    byte_count_or_h_indirect_addr: u16,
    h_indirect_bank: u8,
    h_curr_addr: u16,
    unused: u8,
}

pub struct Dma {
    channels: [Channel; 8],
    enable_channels: u8,
    h_enable_channels: u8,
}
