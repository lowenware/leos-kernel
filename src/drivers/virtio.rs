//
// pcie.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use core::ptr;

pub const MMIO_MAGIC_VALUE: u32 = 0x74726976; // ASCII "virt"
pub const MMIO_VERSION: u32 = 0x2;

#[repr(C)]
struct MMIORegisters {
    magic_value: u32,               // 0x000
    version: u32,                   // 0x004
    device_id: u32,                 // 0x004
    vendor_id: u32,                 // 0x008
    host_features: u32,             // 0x010
    host_features_selector: u32,    // 0x014
    _reserve_2: [u32; 2],
    guest_features: u32,            // 0x020
    guest_features_selector: u32,   // 0x024
    _reserve_3: [u32; 2],
    queue_selector: u32,            // 0x030
    queue_number: u32,              // 0x034
    queue_align: u32,               // 0x038
    queue_pfn: u32,                 // 0x03c
    _reserve_4: [u32; 4],
    queue_notify: u32,              // 0x050
    _reserve_5: [u32; 3],
    interrupt_ack: u32,             // 0x060
    _reserve_6: [u32; 3],
    status: u32,                    // 0x000
    _reserve_7: [u32; 35],
    config: u32,                    // 0x000
}

pub struct MMIO {
    base: *mut MMIORegisters,
}

impl MMIO {

    pub const fn new(mmio_base: usize) -> Self {
        Self { base: mmio_base as *mut MMIORegisters }
    }

    pub fn log(&mut self) {
        let mmio: &mut MMIORegisters;

        unsafe {
            mmio = &mut *(self.base);
        }

        if mmio.device_id == 0 {
            return
        }

        log_state!(
            " ---------- Device @ 0x{:p}\n \
                \t magic: {:08x}\n \
                \t version: {:08x}\n \
                \t device_id: {:08x}\n \
                \t vendor_id: {:08x}\n \
            "
            , self.base
            , mmio.magic_value
            , mmio.version
            , mmio.device_id
            , mmio.vendor_id
        );
    }
}

// + *  0x000 MagicValue       Magic value "virt" (0x74726976 LE)
// + *  0x004 DeviceID         Virtio device ID
// + *  0x008 VendorID         Virtio vendor ID
// + *
// + *  0x010 HostFeatures     Features supported by the host
// + *  0x014 HostFeaturesSel  Set of host features to access via HostFeatures
// + *  0x020 GuestFeatures    Features activated by the guest
// + *  0x024 GuestFeaturesSel Set of activated features to set via GuestFeatures
// + *
// + *  0x030 QueueSel         Queue selector
// + *  0x034 QueueNum         Queue size for the currently selected queue
// + *  0x038 QueueAlign       Used Ring alignment for the current queue
// + *  0x03c QueuePFN         PFN for the currently selected queue
// +
// + *  0x050 QueueNotify      Queue notifier
// + *  0x060 InterruptACK     Interrupt acknowledge register
// + *  0x070 Status           Device status register
// + *
// + *  0x100+                 Device-specific configuration space


