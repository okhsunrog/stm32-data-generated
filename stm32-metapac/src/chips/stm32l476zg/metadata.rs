include!("../metadata_0641.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32L476ZG",
    family: "STM32L4",
    line: "STM32L4x6",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 524288,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8080000,
            size: 524288,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 98304,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 0x10000000,
            size: 0,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
