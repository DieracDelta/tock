//! Board file for SiFive HiFive1 RISC-V development platform.
//!
//! - <https://www.sifive.com/products/hifive1/>
//!
//! This board is no longer being produced. However, many were made so it may
//! be useful for testing Tock with.
//!
//! The primary drawback is the original HiFive1 board did not support User
//! mode, so this board cannot run Tock applications.

#![no_std]
#![no_main]
#![feature(asm)]

extern crate riscvregs;

use capsules::virtual_uart::{MuxUart, UartDevice};
use kernel::capabilities;
use kernel::hil;
use kernel::Platform;
use kernel::{create_capability, debug, static_init};

pub mod io;


// State for loading and holding applications.

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 1;

// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::procs::FaultResponse = kernel::procs::FaultResponse::Panic;

// RAM to be shared by all application processes.
#[link_section = ".app_memory"]
static mut APP_MEMORY: [u8; 8192] = [0; 8192];

// Actual memory for holding the active process structures.
static mut PROCESSES: [Option<&'static kernel::procs::ProcessType>; NUM_PROCS] =
    [None];

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x1000] = [0; 0x1000];

/// A structure representing this platform that holds references to all
/// capsules for this platform. However, since this board does not support
/// userspace this can just be empty.
struct HiFive1 {
    modes: u32,
    //console: &'static capsules::console::Console<'static>,
    //gpio: &'static capsules::gpio::GPIO<'static, sifive::gpio::GpioPin>,
    //led: &'static capsules::led::LED<'static, sifive::gpio::GpioPin>,
}

/// Mapping of integer syscalls to objects that implement syscalls.
impl Platform for HiFive1 {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&kernel::Driver>) -> R,
    {
        match driver_num {
            //capsules::console::DRIVER_NUM => f(Some(self.console)),
            //capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            //capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            //capsules::led::DRIVER_NUM => f(Some(self.led)),
            //capsules::button::DRIVER_NUM => f(Some(self.button)),

            _ => f(None),
        }
    }
}

/// Reset Handler.
///
/// This function is called from the arch crate after some very basic RISC-V
/// setup.
#[no_mangle]
pub unsafe fn reset_handler() {
    // Basic setup of the platform.
    rv32i::init_memory();
    rv32i::configure_trap_handler(rv32i::PermissionMode::Machine);

    let process_mgmt_cap = create_capability!(capabilities::ProcessManagementCapability);
    let main_loop_cap = create_capability!(capabilities::MainLoopCapability);
    let memory_allocation_cap = create_capability!(capabilities::MemoryAllocationCapability);

    e310x::watchdog::WATCHDOG.disable();
    e310x::rtc::RTC.disable();
    e310x::pwm::PWM0.disable();
    e310x::pwm::PWM1.disable();
    e310x::pwm::PWM2.disable();

    e310x::prci::PRCI.set_clock_frequency(sifive::prci::ClockFrequency::Freq18Mhz);

    let main_loop_cap = create_capability!(capabilities::MainLoopCapability);

    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));

    // Configure kernel debug gpios as early as possible
    kernel::debug::assign_gpios(
        Some(&e310x::gpio::PORT[22]), // Red
        None,
        None,
    );

    let chip = static_init!(e310x::chip::E310x, e310x::chip::E310x::new());

    // disable interrupts globally
    riscvregs::register::mstatus::clear_mie();
    // disable specific types of interrupts
    riscvregs::register::mie::clear_msoft();
    riscvregs::register::mie::clear_mtimer();
    riscvregs::register::mie::clear_mext();
    // Need to enable all interrupts for Tock Kernel
    chip.enable_plic_interrupts();
    // enable interrupts globally
    riscvregs::register::mie::set_msoft();
    //we don't use timer interrupts anywhere; not needed
    riscvregs::register::mie::set_mtimer();
    // this should be uncommented and masked; unclear why board hangs
    //riscvregs::register::mie::set_mext();
    riscvregs::register::mstatus::set_mie();

    // Create a shared UART channel for the console and for kernel debug.
    let uart_mux = static_init!(
        MuxUart<'static>,
        MuxUart::new(
            &e310x::uart::UART0,
            &mut capsules::virtual_uart::RX_BUF,
            115200
        )
    );

    uart_mux.initialize();

    hil::uart::Transmit::set_transmit_client(&e310x::uart::UART0, uart_mux);
    hil::uart::Receive::set_receive_client(&e310x::uart::UART0, uart_mux);

    // Initialize some GPIOs which are useful for debugging.
    hil::gpio::Pin::make_output(&e310x::gpio::PORT[22]);
    hil::gpio::Pin::set(&e310x::gpio::PORT[22]);

    hil::gpio::Pin::make_output(&e310x::gpio::PORT[19]);
    hil::gpio::Pin::set(&e310x::gpio::PORT[19]);

    hil::gpio::Pin::make_output(&e310x::gpio::PORT[21]);
    hil::gpio::Pin::clear(&e310x::gpio::PORT[21]);

    let hifive1 = HiFive1 {
        modes: rv32i::PermissionMode::Machine as u32,
    };

    // Create virtual device for kernel debug.
    let debugger_uart = static_init!(UartDevice, UartDevice::new(uart_mux, false));
    debugger_uart.setup();
    let debugger = static_init!(
        kernel::debug::DebugWriter,
        kernel::debug::DebugWriter::new(
            debugger_uart,
            &mut kernel::debug::OUTPUT_BUF,
            &mut kernel::debug::INTERNAL_BUF,
        )
    );
    hil::uart::Transmit::set_transmit_client(debugger_uart, debugger);

    let debug_wrapper = static_init!(
        kernel::debug::DebugWriterWrapper,
        kernel::debug::DebugWriterWrapper::new(debugger)
    );
    kernel::debug::set_debug_writer_wrapper(debug_wrapper);

    e310x::uart::UART0.initialize_gpio_pins(&e310x::gpio::PORT[17], &e310x::gpio::PORT[16]);

    debug!("HiFive1 initialization complete. Entering main loop\n");
    debug!("hello world 1");
    debug!("hello world 2");
    debug!("hello world 3");
    //debug!(
        //"the value in MIE is {:x}\n",
        //riscvregs::register::mie::read().bits()
    //);
    //debug!(
        //"the value in Mstatus is {:x}\n",
        //riscvregs::register::mstatus::read().bits()
    //);

    extern "C" {
        /// Beginning of the ROM region containing app images.
        ///
        /// This symbol is defined in the linker script.
        static _sapps: u8;
    }

    kernel::procs::load_processes(
        board_kernel,
        chip,
        &_sapps as *const u8,
        &mut APP_MEMORY,
        &mut PROCESSES,
        FAULT_RESPONSE,
        &process_mgmt_cap
    );



    board_kernel.kernel_loop(&hifive1, chip, None, &main_loop_cap);
}
