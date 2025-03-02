use rusb::{ DeviceDescriptor, DeviceHandle, GlobalContext};

#[derive(Copy, Clone, Debug)]
struct Dongle {
    vid: u16,
    pid: u16,
    name: &'static str
}

impl Dongle {
    fn is_an_instance(&self, device_descriptor: &DeviceDescriptor) -> bool {
        device_descriptor.vendor_id() == self.vid && device_descriptor.product_id() == self.pid
    }
}

impl TryFrom<DeviceDescriptor> for Dongle {
    type Error = String;

    fn try_from(device_descriptor: DeviceDescriptor) -> Result<Self, Self::Error> {
        let dongle = KNOWN_DEVICES.iter().find(|known_dongle| known_dongle.is_an_instance(&device_descriptor)).copied();
        dongle.ok_or(format!("Device with vendor id {} and product id {} is not supported!", device_descriptor.vendor_id(), device_descriptor.product_id()))
    }
}

static KNOWN_DEVICES: &[Dongle] = &[
    Dongle { vid: 0x0bda, pid: 0x2832, name: "Generic RTL2832U" },
	Dongle { vid: 0x0bda, pid: 0x2838, name: "Generic RTL2832U OEM" },
	//{ 0x0413, 0x6680, "DigitalNow Quad DVB-T PCI-E card" },
	//{ 0x0413, 0x6f0f, "Leadtek WinFast DTV Dongle mini D" },
	//{ 0x0458, 0x707f, "Genius TVGo DVB-T03 USB dongle (Ver. B)" },
	//{ 0x0ccd, 0x00a9, "Terratec Cinergy T Stick Black (rev 1)" },
	//{ 0x0ccd, 0x00b3, "Terratec NOXON DAB/DAB+ USB dongle (rev 1)" },
	//{ 0x0ccd, 0x00b4, "Terratec Deutschlandradio DAB Stick" },
	//{ 0x0ccd, 0x00b5, "Terratec NOXON DAB Stick - Radio Energy" },
	//{ 0x0ccd, 0x00b7, "Terratec Media Broadcast DAB Stick" },
	//{ 0x0ccd, 0x00b8, "Terratec BR DAB Stick" },
	//{ 0x0ccd, 0x00b9, "Terratec WDR DAB Stick" },
	//{ 0x0ccd, 0x00c0, "Terratec MuellerVerlag DAB Stick" },
	//{ 0x0ccd, 0x00c6, "Terratec Fraunhofer DAB Stick" },
	//{ 0x0ccd, 0x00d3, "Terratec Cinergy T Stick RC (Rev.3)" },
	//{ 0x0ccd, 0x00d7, "Terratec T Stick PLUS" },
	//{ 0x0ccd, 0x00e0, "Terratec NOXON DAB/DAB+ USB dongle (rev 2)" },
	Dongle { vid: 0x1209, pid: 0x2832, name: "Generic RTL2832U" },
	//{ 0x1554, 0x5020, "PixelView PV-DT235U(RN)" },
	//{ 0x15f4, 0x0131, "Astrometa DVB-T/DVB-T2" },
	//{ 0x15f4, 0x0133, "HanfTek DAB+FM+DVB-T" },
	//{ 0x185b, 0x0620, "Compro Videomate U620F"},
	//{ 0x185b, 0x0650, "Compro Videomate U650F"},
	//{ 0x185b, 0x0680, "Compro Videomate U680F"},
	//{ 0x1b80, 0xd393, "GIGABYTE GT-U7300" },
	//{ 0x1b80, 0xd394, "DIKOM USB-DVBT HD" },
	//{ 0x1b80, 0xd395, "Peak 102569AGPK" },
	//{ 0x1b80, 0xd397, "KWorld KW-UB450-T USB DVB-T Pico TV" },
	//{ 0x1b80, 0xd398, "Zaapa ZT-MINDVBZP" },
	//{ 0x1b80, 0xd39d, "SVEON STV20 DVB-T USB & FM" },
	//{ 0x1b80, 0xd3a4, "Twintech UT-40" },
	//{ 0x1b80, 0xd3a8, "ASUS U3100MINI_PLUS_V2" },
	//{ 0x1b80, 0xd3af, "SVEON STV27 DVB-T USB & FM" },
	//{ 0x1b80, 0xd3b0, "SVEON STV21 DVB-T USB & FM" },
	//{ 0x1d19, 0x1101, "Dexatek DK DVB-T Dongle (Logilink VG0002A)" },
	//{ 0x1d19, 0x1102, "Dexatek DK DVB-T Dongle (MSI DigiVox mini II V3.0)" },
	//{ 0x1d19, 0x1103, "Dexatek Technology Ltd. DK 5217 DVB-T Dongle" },
	//{ 0x1d19, 0x1104, "MSI DigiVox Micro HD" },
	//{ 0x1f4d, 0xa803, "Sweex DVB-T USB" },
	//{ 0x1f4d, 0xb803, "GTek T803" },
	//{ 0x1f4d, 0xc803, "Lifeview LV5TDeluxe" },
	//{ 0x1f4d, 0xd286, "MyGica TD312" },
	//{ 0x1f4d, 0xd803, "PROlectrix DV107669" },
];

#[derive(Debug)]
struct Device {
    device_handle: DeviceHandle<GlobalContext>,
    dongle_type: Dongle
}

fn open_device() -> Option<Device> {
    let devices = rusb::devices().unwrap();
    let found_device = devices.iter().find(
        |device| KNOWN_DEVICES.iter().any(
            |known_dongle| known_dongle.is_an_instance(&device.device_descriptor().unwrap())
        )
    );
    if let Some(device) = found_device {
		let device_descriptor = device.device_descriptor().unwrap();
		let dongle = Dongle::try_from(device_descriptor).unwrap();
        println!("Found {} dongle, trying to open", dongle.name);
        let device_handle_result = device.open();
        if let Err(rusb::Error::Access) = device_handle_result {
            panic!("Please fix the device permissions, e.g. \
			by installing the udev rules file rtl-sdr.rules\n");
        }
        let device_handle = device_handle_result.unwrap();
        if device_handle.kernel_driver_active(0).unwrap() {
            panic!("\nKernel driver is active, or device is \
				claimed by second instance of librtlsdr. \
				\nIn the first case, please either detach \
				 or blacklist the kernel module\n \
				(dvb_usb_rtl28xxu), or enable automatic \
				 detaching at compile time.\n\n");
        }
        device_handle.claim_interface(0).unwrap();
        println!("Device opened and interface claimed");
        // TODO: to be continued, finished at librtlsdr.c:3232
        Some(Device { device_handle: device_handle, dongle_type: dongle })
    } else {
        None
    }
}

fn main() {
    println!("{:?}", open_device());
}
