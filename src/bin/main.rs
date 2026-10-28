#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::main;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::delay::Delay;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::mcpwm::operator::PwmPinConfig;
use esp_hal::mcpwm::timer::PwmWorkingMode;
use esp_hal::mcpwm::{PeripheralClockConfig,McPwm}; 
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::xtensa_lx::timer::delay;
use esp_println::println;
use log::info;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]


#[main]
fn main() -> ! {
    // generator version: 1.4.0
    // generator parameters: -o esp32 -o esp32-wroom-32 -o embassy -o unstable-hal -o log -o esp-backtrace -o vscode

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let clock_cfg = PeripheralClockConfig::with_frequency(Rate::from_mhz(32)).unwrap();
    let delay = Delay::new();

    //Servo setup
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);
    mcpwm.operator0.set_timer(&mcpwm.timer0);

    let mut pwm_pin = mcpwm
        .operator0
        .with_pin_a(peripherals.GPIO33,PwmPinConfig::UP_ACTIVE_HIGH); 
       
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(19_999, PwmWorkingMode::Increase, Rate::from_hz(50))
        .unwrap();

    mcpwm.timer0.start(timer_clock_cfg);



    loop {
        pwm_pin.set_timestamp(1000);
        esp_println::println!("1000");
        delay.delay_millis(1500);

        pwm_pin.set_timestamp(2000);
        esp_println::println!("2000");
        delay.delay_millis(1500);

        pwm_pin.set_timestamp(2500);
        esp_println::println!("2500");
        delay.delay_millis(1500);
    }

}
