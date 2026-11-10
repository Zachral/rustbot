#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::time;

use embassy_executor::{SpawnError, Spawner};
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::peripherals::Peripherals;
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;

use esp_hal::mcpwm::timer::{PeriodUpdatingMethod, PwmWorkingMode};
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
use esp_hal::time::Rate;
use joystick::{joystick_init, map_joystick_raw_to_degree};
use servos::{ServoType, servo_init};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const DEADZONE: u16 = 50;
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
    let mut servo_waist = servo_init(
        pwm0.operator0,
        &pwm0.timer0,
        peripherals.GPIO33,
        ServoType::Mg996r,
    );

    let mut servo_shoulder = servo_init(
        pwm0.operator1,
        &pwm0.timer0,
        peripherals.GPIO32,
        ServoType::Mg996r,
    );

    //joystick

    let mut joystick = joystick_init(peripherals.ADC2, peripherals.GPIO14, peripherals.GPIO13);
    let mut prev_vrx = joystick.read_x().unwrap_or(0);
    let mut prev_vry = joystick.read_y().unwrap_or(0);
    let mut print_vals = true;

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    let _spawner = spawner;
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(19_999, PwmWorkingMode::Increase, Rate::from_hz(50))
        .unwrap();

    pwm0.timer0.start(timer_clock_cfg);

    loop {
        if let Ok(x) = joystick.read_x() {
            if x.abs_diff(prev_vrx) > DEADZONE {
                prev_vrx = x;
                print_vals = true;
                servo_waist.set_angle(map_joystick_raw_to_degree(x));
            }
        }

        if let Ok(y) = joystick.read_y() {
            if y.abs_diff(prev_vry) > DEADZONE {
                prev_vry = y;
                print_vals = true;
                servo_shoulder.set_angle(map_joystick_raw_to_degree(y));
            }
        }

        if print_vals {
            print_vals = false;

            println!("X: {:?} Y: {:?}", prev_vrx, prev_vry);
        }

        Timer::after(Duration::from_millis(50)).await;
    }
}
