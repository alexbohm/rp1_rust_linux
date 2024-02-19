// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::{
    c_str, define_pci_id_table, device::RawDevice, driver, io_mem::IoMem, pci, prelude::*,
};

module! {
    type: RustOutOfTree,
    name: "rust_out_of_tree",
    author: "Rust for Linux Contributors",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

const RP1_SYSINFO_BASE: usize = 0x000000;

const RP1_IO_BANK0_BASE: usize = 0xd0000;
const RP1_SYSRIO0_BASE: usize = 0xe0000;
const RP1_PADS_BANK0_BASE: usize = 0xf0000;

const SYSINFO_CHIP_ID_OFFSET: usize = 0x00000000;
const SYSINFO_PLATFORM_OFFSET: usize = 0x00000004;

const PCI_VENDOR_ID_RPI: u32 = 0x1de4;
const PCI_DEVICE_ID_RP1_C0: u32 = 0x0001;

const RP1_C0_CHIP_ID: u32 = 0x20001927;

struct RP1Device;

impl pci::Driver for RP1Device {
    define_pci_id_table! {(),
        [(pci::DeviceId::new(PCI_VENDOR_ID_RPI, PCI_DEVICE_ID_RP1_C0), None)]
    }

    fn probe(
        dev: &mut pci::Device,
        _id: core::prelude::v1::Option<&Self::IdInfo>,
    ) -> Result<Self::Data> {
        pr_info!("Probing rp1: {}", dev.name());

        // Take resource 1 to access peripherials on BAR1. See section 2.3.1 of the RP1 datasheet.
        let res = dev.take_resource(1).ok_or_else(|| {
            pr_warn!("Failed to take resource.");
            ENXIO
        })?;

        pr_info!("Resource: {:?}", res);

        let bar = unsafe { IoMem::<4194304>::try_new(res) }?;

        // Read and verify the rp1 chip id.
        {
            let chip_id: u32 = bar.readl(RP1_SYSINFO_BASE + SYSINFO_CHIP_ID_OFFSET);
            let platform: u32 = bar.readl(RP1_SYSINFO_BASE + SYSINFO_PLATFORM_OFFSET);

            pr_info!("chip_id: {chip_id:#x} platform: {platform:#x}");

            if chip_id != RP1_C0_CHIP_ID {
                return Err(EINVAL);
            }
        }

        // Read the status and ctrl registers for GPIO26.
        {
            let status: u32 = bar.readl(RP1_IO_BANK0_BASE + 0x0d0);
            let ctrl: u32 = bar.readl(RP1_IO_BANK0_BASE + 0x0d4);

            pr_info!("GPIO26 status: {status:b} ctrl: {ctrl:b}");
        }
        // Turn on GPIO26 by setting a bit in the SYSRIO0 RIO_OUT register.
        {
            const RP1_SET_OFFSET: usize = 0x2000;
            const RP1_CLR_OFFSET: usize = 0x3000;

            const PIN_NUM: u32 = 26;
            let value = true;

            // The registers have atomic access aliases at certain offsets.
            let offset = RP1_SYSRIO0_BASE
                + if value {
                    RP1_SET_OFFSET
                } else {
                    RP1_CLR_OFFSET
                };

            bar.writel(1 << PIN_NUM, offset);
        }

        pr_info!("Success");
        Ok(())
    }

    fn remove(_data: &Self::Data) {
        pr_info!("Removing rp1");

        // todo!();
    }
}

struct RustOutOfTree {
    registration: Pin<Box<driver::Registration<pci::Adapter<RP1Device>>>>,
}

impl kernel::Module for RustOutOfTree {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let registration = driver::Registration::new_pinned(c_str!("alex-rp1"), module)?;

        Ok(RustOutOfTree { registration })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
