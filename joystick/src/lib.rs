#![no_std]

#[cfg(feature = "esp32")]
use esp_hal::{
    Blocking, analog::adc::{Adc, AdcChannel, AdcConfig, AdcPin, Attenuation}, gpio::{AnalogPin, Input, InputConfig, Pull}, peripherals::ADC1,
};
use rustbot_common::{MAX_ANGLE_DEGREE, MIN_ANGLE_DEGREE};

#[derive(Debug, Clone, Copy)]
pub struct AdcError;

const MAX_RAW_INPUT: i32 = 4095;

#[cfg(feature = "esp32")]
pub struct JoystickReadings {
    pub left_x: u16,
    pub left_y: u16,
    pub right_x: u16,
    pub right_y: u16,
}

#[cfg(feature = "esp32")]
pub struct JoystickController<'d, LX, LY, RX, RY> {
    adc: Adc<'d, ADC1<'d>, Blocking>,
    left_x: AdcPin<LX, ADC1<'d>>,
    left_y: AdcPin<LY, ADC1<'d>>,
    right_x: AdcPin<RX, ADC1<'d>>,
    right_y: AdcPin<RY, ADC1<'d>>,
    left_joystick_btn: Input<'d>,
    right_joystick_btn: Input<'d>,
}

#[cfg(feature = "esp32")]
impl<'d, LX, LY, RX, RY> JoystickController<'d, LX, LY, RX, RY>
where
    LX: AdcChannel,
    LY: AdcChannel,
    RX: AdcChannel,
    RY: AdcChannel,
{
    pub fn read(&mut self) -> Result<JoystickReadings, AdcError> {
        Ok(JoystickReadings {
            left_x: nb::block!(self.adc.read_oneshot(&mut self.left_x)).map_err(|_| AdcError)?,
            left_y: nb::block!(self.adc.read_oneshot(&mut self.left_y)).map_err(|_| AdcError)?,
            right_x: nb::block!(self.adc.read_oneshot(&mut self.right_x)).map_err(|_| AdcError)?,
            right_y: nb::block!(self.adc.read_oneshot(&mut self.right_y)).map_err(|_| AdcError)?,
        })
    }

    pub fn left_btn_pressed(&self) -> bool{
        self.left_joystick_btn.is_low()
    }

    pub fn right_btn_pressed(&self) -> bool{
        self.right_joystick_btn.is_low()
    }
}

#[cfg(feature = "esp32")]
pub fn joystick_controller_init<'d, LX, LY, RX, RY, LB, RB>(
    adc1: ADC1<'d>,
    left_x_pin: LX,
    left_y_pin: LY,
    right_x_pin: RX,
    right_y_pin: RY,
    left_btn: LB,
    right_btn: RB,
) -> JoystickController <'d, LX, LY, RX, RY>
where
    LX: AdcChannel + AnalogPin,
    LY: AdcChannel + AnalogPin,
    RX: AdcChannel + AnalogPin,
    RY: AdcChannel + AnalogPin,
    LB: esp_hal::gpio::InputPin + 'd,
    RB: esp_hal::gpio::InputPin + 'd
{
    let mut config = AdcConfig::<ADC1>::new();
    let left_x = config.enable_pin(left_x_pin, Attenuation::_11dB);
    let left_y = config.enable_pin(left_y_pin, Attenuation::_11dB);
    let right_x = config.enable_pin(right_x_pin, Attenuation::_11dB);
    let right_y = config.enable_pin(right_y_pin, Attenuation::_11dB);
    let left_joystick_btn = Input::new(left_btn, InputConfig::default().with_pull(Pull::Up)); 
    let right_joystick_btn = Input::new(right_btn, InputConfig::default().with_pull(Pull::Up)); 
    let adc = Adc::new(adc1, config);

    JoystickController {
        adc,
        left_x,
        left_y,
        right_x,
        right_y,
        left_joystick_btn,
        right_joystick_btn
    }
}

pub fn map_joystick_raw_to_degree(input: u16) -> i32 {
    let mapped = (input.min(MAX_RAW_INPUT as u16) as i32) * MAX_ANGLE_DEGREE / MAX_RAW_INPUT;
    let degree = mapped.clamp(MIN_ANGLE_DEGREE, MAX_ANGLE_DEGREE);
    degree
}

#[cfg(test)]
mod tests {
    use super::map_joystick_raw_to_degree;

    #[test]
    fn maps_joystick_to_endpoints_and_midpoints() {
        assert_eq!(map_joystick_raw_to_degree(0), 0);
        let degree = map_joystick_raw_to_degree(2050);
        assert!((85..=95).contains(&degree), "degree was {degree}");
        assert_eq!(map_joystick_raw_to_degree(4095), 180);
    }

    #[test]
    fn clamps_input_to_supported_range() {
        assert_eq!(map_joystick_raw_to_degree(4096), 180);
    }
}
