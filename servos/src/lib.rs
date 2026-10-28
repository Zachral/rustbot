#![no_std]

use esp_hal::{
    gpio::interconnect::PeripheralOutput,
    mcpwm::{
        operator::{Operator, PwmPin, PwmPinConfig},
        timer::Timer,
        PwmPeripheral,
    },
};

pub struct Servo<'d, PWM, const OP: u8>
where
    PWM: PwmPeripheral,
{
    pwm_pin: PwmPin<'d, PWM, OP, true>,
}

impl<PWM, const OP: u8> Servo<'_, PWM, OP>
where
    PWM: PwmPeripheral,
{
    pub fn set_pulse_width(&mut self, microseconds: u16) {
        self.pwm_pin.set_timestamp(microseconds);
    }
}

pub fn servo_init<'d, PWM, const OP: u8, const TIM: u8, PIN>(
    mut operator: Operator<'d, OP, PWM>,
    timer: &Timer<TIM, PWM>,
    pin: PIN,
) -> Servo<'d, PWM, OP>
where
    PWM: PwmPeripheral + 'd,
    PIN: PeripheralOutput<'d>,
{
    operator.set_timer(timer);

    Servo {
        pwm_pin: operator.with_pin_a(pin, PwmPinConfig::UP_ACTIVE_HIGH),
    }
}
