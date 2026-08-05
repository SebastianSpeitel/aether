use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Poll;

use aether_core::capability::HasDriver;
use aether_core::driver::{Driver, ReadDriver, WriteDriver};
use aether_core::kernel::Kernel;
use aether_core::task::Task;
use aether_platform::driver::PinDriver;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

// Static mock hardware register states
pub static MOCK_BUTTON_STATE: AtomicBool = AtomicBool::new(false);
pub static MOCK_LED_STATE: AtomicBool = AtomicBool::new(false);

// -----------------------------------------------------------------------------
// 1. Zero-Sized Mock embedded-hal InputPin and OutputPin
// -----------------------------------------------------------------------------

#[derive(Default, Clone, Copy)]
pub struct MockButtonPin;

impl ErrorType for MockButtonPin {
    type Error = core::convert::Infallible;
}

impl InputPin for MockButtonPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(MOCK_BUTTON_STATE.load(Ordering::Relaxed))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!MOCK_BUTTON_STATE.load(Ordering::Relaxed))
    }
}

#[derive(Default, Clone, Copy)]
pub struct MockLedPin;

impl ErrorType for MockLedPin {
    type Error = core::convert::Infallible;
}

impl OutputPin for MockLedPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        MOCK_LED_STATE.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        MOCK_LED_STATE.store(true, Ordering::Relaxed);
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// 2. Task listening to Button (ReadDriver) and toggling LED (WriteDriver)
// -----------------------------------------------------------------------------

pub struct ButtonLedTask<LedDrv: Driver, BtnDrv: Driver> {
    led_handle: LedDrv::Handle,
    btn_handle: BtnDrv::Handle,
    step: usize,
    last_btn_state: u8,
}

impl<LedDrv: Driver, BtnDrv: Driver> ButtonLedTask<LedDrv, BtnDrv> {
    pub const fn new(led_handle: LedDrv::Handle, btn_handle: BtnDrv::Handle) -> Self {
        Self {
            led_handle,
            btn_handle,
            step: 0,
            last_btn_state: 0,
        }
    }
}

impl<K, LedDrv, BtnDrv> Task<K> for ButtonLedTask<LedDrv, BtnDrv>
where
    K: Kernel + HasDriver<LedDrv> + HasDriver<BtnDrv>,
    LedDrv: WriteDriver,
    BtnDrv: ReadDriver,
{
    type Output = ();

    fn poll(&mut self, kernel: &K) -> Poll<Self::Output> {
        let led_driver = HasDriver::<LedDrv>::get_driver(kernel);
        let btn_driver = HasDriver::<BtnDrv>::get_driver(kernel);

        let mut btn_buf = [0u8; 1];
        btn_driver.read(&self.btn_handle, &mut btn_buf).unwrap();

        let pressed = btn_buf[0] != 0;

        if pressed != (self.last_btn_state != 0) {
            if pressed {
                println!("  [Task] Button Pressed! Writing HIGH (1) to LED PinDriver...");
                led_driver.write(&self.led_handle, &[1]).unwrap();
            } else {
                println!("  [Task] Button Released! Writing LOW (0) to LED PinDriver...");
                led_driver.write(&self.led_handle, &[0]).unwrap();
            }
            self.last_btn_state = u8::from(pressed);
        } else {
            println!(
                "  [Task] Polled button state: {}",
                if pressed {
                    "HIGH (Pressed)"
                } else {
                    "LOW (Idle)"
                }
            );
        }

        self.step += 1;
        if self.step >= 10 {
            Poll::Ready(())
        } else {
            kernel.r#yield()
        }
    }
}

// -----------------------------------------------------------------------------
// 3. Embedded Kernel Context (0 Bytes of RAM!)
// -----------------------------------------------------------------------------

pub struct EmbeddedKernel {
    pub led_driver: PinDriver<MockLedPin>,
    pub btn_driver: PinDriver<MockButtonPin>,
}

impl Kernel for EmbeddedKernel {}

impl HasDriver<PinDriver<MockLedPin>> for EmbeddedKernel {
    type DriverRef<'b>
        = &'b PinDriver<MockLedPin>
    where
        Self: 'b;
    fn get_driver(&self) -> &PinDriver<MockLedPin> {
        &self.led_driver
    }
}

impl HasDriver<PinDriver<MockButtonPin>> for EmbeddedKernel {
    type DriverRef<'b>
        = &'b PinDriver<MockButtonPin>
    where
        Self: 'b;
    fn get_driver(&self) -> &PinDriver<MockButtonPin> {
        &self.btn_driver
    }
}

fn main() {
    println!("=== aether Embedded PinDriver ZST Example ===");

    // Verify 0-byte memory footprints!
    assert_eq!(core::mem::size_of::<MockButtonPin>(), 0);
    assert_eq!(core::mem::size_of::<MockLedPin>(), 0);
    assert_eq!(core::mem::size_of::<PinDriver<MockButtonPin>>(), 0);
    assert_eq!(core::mem::size_of::<PinDriver<MockLedPin>>(), 0);
    assert_eq!(core::mem::size_of::<EmbeddedKernel>(), 0);
    println!(
        "  [Memory Audit] Size of PinDriver<MockButtonPin>: {} bytes",
        core::mem::size_of::<PinDriver<MockButtonPin>>()
    );
    println!(
        "  [Memory Audit] Size of EmbeddedKernel: {} bytes",
        core::mem::size_of::<EmbeddedKernel>()
    );

    // Wrap zero-sized embedded-hal pins into aether PinDrivers
    let btn_driver = PinDriver::new(MockButtonPin);
    let led_driver = PinDriver::new(MockLedPin);

    // Open handles via Driver capability
    btn_driver.open(()).unwrap();
    led_driver.open(()).unwrap();

    let kernel = EmbeddedKernel {
        led_driver,
        btn_driver,
    };

    let mut task: ButtonLedTask<PinDriver<MockLedPin>, PinDriver<MockButtonPin>> =
        ButtonLedTask::new((), ());

    println!("\n--- Poll 1: Initial State (Button Idle) ---");
    let _ = task.poll(&kernel);
    println!(
        "  -> Physical LED hardware state: {}",
        if MOCK_LED_STATE.load(Ordering::Relaxed) {
            "ON"
        } else {
            "OFF"
        }
    );

    println!("\n--- Poll 2: User Presses Button ---");
    MOCK_BUTTON_STATE.store(true, Ordering::Relaxed);
    let _ = task.poll(&kernel);
    println!(
        "  -> Physical LED hardware state: {}",
        if MOCK_LED_STATE.load(Ordering::Relaxed) {
            "ON"
        } else {
            "OFF"
        }
    );

    println!("\n--- Poll 3: User Releases Button ---");
    MOCK_BUTTON_STATE.store(false, Ordering::Relaxed);
    let _ = task.poll(&kernel);
    println!(
        "  -> Physical LED hardware state: {}",
        if MOCK_LED_STATE.load(Ordering::Relaxed) {
            "ON"
        } else {
            "OFF"
        }
    );

    println!("\n=== Example Finished Successfully ===");
}
