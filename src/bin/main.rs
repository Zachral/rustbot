#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;

use esp_hal::mcpwm::timer::PwmWorkingMode;
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
use esp_hal::time::Rate;
use joystick::{AdcError, joystick_controller_init, map_joystick_raw_to_degree};
use servos::{ServoType, servo_init};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const DEADZONE: u16 = 50;
#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]

struct PreviousReadings {
    left_x: u16,
    left_y: u16,
    right_x: u16,
    right_y: u16,
}
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.4.0
    // generator parameters: -o esp32 -o esp32-wroom-32 -o embassy -o unstable-hal -o log -o esp-backtrace -o vscode

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let clock_cfg = PeripheralClockConfig::with_frequency(Rate::from_mhz(32)).unwrap();
    let mut pwm0 = McPwm::new(peripherals.MCPWM0, clock_cfg);
    let mut pwm1 = McPwm::new(peripherals.MCPWM1, clock_cfg);
    //Servo setup
    let mut servo_waist = servo_init(
        pwm0.operator0,
        &pwm0.timer0,
        peripherals.GPIO26,
        ServoType::Mg996r,
    );

    let mut servo_shoulder = servo_init(
        pwm0.operator1,
        &pwm0.timer0,
        peripherals.GPIO27,
        ServoType::Mg996r,
    );

    let mut servo_elbow = servo_init(
        pwm0.operator2,
        &pwm0.timer0,
        peripherals.GPIO25,
        ServoType::Mg996r,
    );

    let mut servo_wrist_pitch = servo_init(
        pwm1.operator0,
        &pwm1.timer0,
        peripherals.GPIO22,
        ServoType::Ts90a,
    );
    //joystick

    let mut joysticks = joystick_controller_init(
        peripherals.ADC1,
        peripherals.GPIO34,
        peripherals.GPIO35,
        peripherals.GPIO32,
        peripherals.GPIO33,
    );

    let mut print_vals = true;

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    let _spawner = spawner;
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(19_999, PwmWorkingMode::Increase, Rate::from_hz(50))
        .unwrap();

    pwm0.timer0.start(timer_clock_cfg);
    pwm1.timer0.start(timer_clock_cfg);

    let first_read = loop {
        match joysticks.read() {
            Ok(readings) => break readings,
            Err(_) => {
                println!("Initial joystick ADC read failed; retrying");
                Timer::after(Duration::from_millis(100)).await;
            }
        };
    };

    let mut previous_readings = PreviousReadings {
        left_x: first_read.left_x,
        left_y: first_read.left_y,
        right_x: first_read.right_x,
        right_y: first_read.right_y,
    };

    loop {

        if let Ok(joystick_readings) = joysticks.read() {
            // samples.left_x, samples.left_y, samples.right_x, samples.right_y

            if previous_readings
                .right_x
                .abs_diff(joystick_readings.right_x)
                > DEADZONE
            {
                servo_shoulder.set_angle(map_joystick_raw_to_degree(joystick_readings.right_x));
                previous_readings.right_x = joystick_readings.right_x;
                print_vals = true;
            }

            if previous_readings
                .right_y
                .abs_diff(joystick_readings.right_y)
                > DEADZONE
            {
                servo_waist.set_angle(map_joystick_raw_to_degree(joystick_readings.right_y));
                previous_readings.right_y = joystick_readings.right_y;
                print_vals = true;
            }

            if previous_readings.left_y.abs_diff(joystick_readings.left_y) > DEADZONE {
                servo_elbow.set_angle(map_joystick_raw_to_degree(joystick_readings.left_y));
                previous_readings.left_y = joystick_readings.left_y;
                print_vals = true;
            }

            if previous_readings.left_x.abs_diff(joystick_readings.left_x) > DEADZONE {
                servo_wrist_pitch.set_angle(map_joystick_raw_to_degree(joystick_readings.left_x));
                previous_readings.left_x = joystick_readings.left_x;
                print_vals = true;
            }

            if print_vals {
                print_vals = false;

                println!(
                    "left_x: {:?}
                    left_y: {:?}
                    right_x: {:?}
                    right_y: {:?}",
                    joystick_readings.left_x,
                    joystick_readings.left_y,
                    joystick_readings.right_x, 
                    joystick_readings.right_y
                );
            }
        }

        Timer::after(Duration::from_millis(50)).await;
    }
}
