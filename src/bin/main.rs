#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::result::Result::Ok;
use esp_hal::analog::adc::AdcConfig;
use esp_hal::main; 
use esp_hal::delay::Delay;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use embassy_executor::Spawner;
use esp_hal::timer::timg::TimerGroup;
use embassy_time::{Duration, Timer};
use esp_println::{self as _, println};

use esp_hal::mcpwm::timer::{PeriodUpdatingMethod, PwmWorkingMode};
use esp_hal::mcpwm::{PeripheralClockConfig,McPwm}; 
use esp_hal::time::Rate;
use joystick:: joystick_init; 
use servos::servo_init;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]


#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.4.0
    // generator parameters: -o esp32 -o esp32-wroom-32 -o embassy -o unstable-hal -o log -o esp-backtrace -o vscode

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let clock_cfg = PeripheralClockConfig::with_frequency(Rate::from_mhz(32)).unwrap();
    let delay = Delay::new();
    let mut pwm0 = McPwm::new(peripherals.MCPWM0, clock_cfg);
    //Servo setup
    let mut servo_1 = servo_init(pwm0.operator0, &pwm0.timer0, peripherals.GPIO33);
    
    //joystick
   
    let mut joystick = joystick_init(peripherals.ADC2, peripherals.GPIO13, peripherals.GPIO14); 
    let mut prev_vrx: u16 = 0;
    let mut prev_vry: u16 = 0;
    let mut print_vals = true;   
    let mut read_value_x: u16 = 0;
    let mut read_value_y: u16 = 0;  
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    

    let _ = spawner;
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(19_999, PwmWorkingMode::Increase, Rate::from_hz(50))
        .unwrap();

    pwm0.timer0.start(timer_clock_cfg);



    loop {
        
        read_value_x = nb::block!(joystick.read_x()).unwrap(); 
        read_value_y = nb::block!(joystick.read_y()).unwrap(); 

        if read_value_x.abs_diff(prev_vrx) > 100 {
            prev_vrx = read_value_x; 
            print_vals = true;
        }

        if read_value_y.abs_diff(prev_vry) > 100 {
            prev_vry = read_value_y;
            print_vals = true;
        }

        if print_vals {
            print_vals = false;

            println!("X: {} Y: {}\r\n", read_value_x, read_value_y);
        }

        Timer::after(Duration::from_millis(50)).await;
    }

}
