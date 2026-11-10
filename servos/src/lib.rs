#![no_std]

use rustbot_common::{MAX_ANGLE_DEGREE, MIN_ANGLE_DEGREE};

const SERVO_TS_MIN_PULSEWIDTH_US: i32 = 500;
const SERVO_TS_MAX_PULSEWIDTH_US: i32 = 2500;
const SERVO_MG_MIN_PULSEWIDTH_US: i32 = 400;
const SERVO_MG_MAX_PULSEWIDTH_US: i32 = 2500;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServoType {
    Ts90a,
    Mg996r,
}

#[cfg(feature = "esp32")]
use esp_hal::{
    gpio::interconnect::PeripheralOutput,
    mcpwm::{
        PwmPeripheral,
        operator::{Operator, PwmPin, PwmPinConfig},
        timer::Timer,
    },
};

#[cfg(feature = "esp32")]
pub struct Servo<'d, PWM, const OP: u8>
where
    PWM: PwmPeripheral,
{
    pwm_pin: PwmPin<'d, PWM, OP, true>,
    servo_type: ServoType,
}

#[cfg(feature = "esp32")]
impl<PWM, const OP: u8> Servo<'_, PWM, OP>
where
    PWM: PwmPeripheral,
{
    pub fn set_pulse_width(&mut self, microseconds: u16) {
        self.pwm_pin.set_timestamp(microseconds);
    }

    pub fn set_angle(&mut self, degrees: i32) {
        self.set_pulse_width(map_degree_to_pulsewidth(self.servo_type, degrees) as u16);
    }
}

#[cfg(feature = "esp32")]
pub fn servo_init<'d, PWM, const OP: u8, const TIM: u8, PIN>(
    mut operator: Operator<'d, OP, PWM>,
    timer: &Timer<TIM, PWM>,
    pin: PIN,
    servo_type: ServoType,
) -> Servo<'d, PWM, OP>
where
    PWM: PwmPeripheral + 'd,
    PIN: PeripheralOutput<'d>,
{
    operator.set_timer(timer);

    Servo {
        pwm_pin: operator.with_pin_a(pin, PwmPinConfig::UP_ACTIVE_HIGH),
        servo_type,
    }
}

pub fn map_degree_to_pulsewidth(servo_type: ServoType, servo_angle: i32) -> u32 {
    let clamped = servo_angle.clamp(MIN_ANGLE_DEGREE, MAX_ANGLE_DEGREE);
    let (min_pulsewidth, max_pulsewidth) = match servo_type {
        ServoType::Ts90a => (SERVO_TS_MIN_PULSEWIDTH_US, SERVO_TS_MAX_PULSEWIDTH_US),
        ServoType::Mg996r => (SERVO_MG_MIN_PULSEWIDTH_US, SERVO_MG_MAX_PULSEWIDTH_US),
    };

    ((clamped - MIN_ANGLE_DEGREE) * (max_pulsewidth - min_pulsewidth)
        / (MAX_ANGLE_DEGREE - MIN_ANGLE_DEGREE)
        + min_pulsewidth) as u32
}

#[cfg(test)]
mod tests {
    use super::{ServoType, map_degree_to_pulsewidth};

    #[test]
    fn maps_servo_endpoints_and_midpoints() {
        assert_eq!(map_degree_to_pulsewidth(ServoType::Ts90a, 0), 500);
        assert_eq!(map_degree_to_pulsewidth(ServoType::Ts90a, 90), 1500);
        assert_eq!(map_degree_to_pulsewidth(ServoType::Ts90a, 180), 2500);

        assert_eq!(map_degree_to_pulsewidth(ServoType::Mg996r, 0), 400);
        assert_eq!(map_degree_to_pulsewidth(ServoType::Mg996r, 90), 1450);
        assert_eq!(map_degree_to_pulsewidth(ServoType::Mg996r, 180), 2500);
    }

    #[test]
    fn clamps_angles_to_supported_range() {
        assert_eq!(map_degree_to_pulsewidth(ServoType::Mg996r, -1), 400);
        assert_eq!(map_degree_to_pulsewidth(ServoType::Mg996r, 181), 2500);
    }
}
