#![no_std]

#[cfg(feature = "esp32")]
use esp_hal::{
    Blocking,
    analog::adc::{Adc, AdcChannel, AdcConfig, AdcPin, Attenuation},
    gpio::AnalogPin,
    peripherals::ADC2,
};
use rustbot_common::{MAX_ANGLE_DEGREE, MIN_ANGLE_DEGREE};

#[derive(Debug, Clone, Copy)]
pub struct AdcError;

const MAX_RAW_INPUT: i32 = 4095;

#[cfg(feature = "esp32")]
pub struct Joystick<'d, VRX, VRY> {
    adc: Adc<'d, ADC2<'d>, Blocking>,
    vrx: AdcPin<VRX, ADC2<'d>>,
    vry: AdcPin<VRY, ADC2<'d>>,
}
#[cfg(feature = "esp32")]
impl<'d, VRX, VRY> Joystick<'d, VRX, VRY>
where
    VRX: AdcChannel,
    VRY: AdcChannel,
{
    pub fn read_x(&mut self) -> Result<u16, AdcError> {
        nb::block!(self.adc.read_oneshot(&mut self.vrx)).map_err(|_| AdcError)
    }

    pub fn read_y(&mut self) -> Result<u16, AdcError> {
        nb::block!(self.adc.read_oneshot(&mut self.vry)).map_err(|_| AdcError)
    }

    pub fn read(&mut self) -> Result<(u16, u16), AdcError> {
        Ok((self.read_x()?, self.read_y()?))
    }
}

#[cfg(feature = "esp32")]
pub fn joystick_init<'d, VRX, VRY>(
    adc2: ADC2<'d>,
    vrx_pin: VRX,
    vry_pin: VRY,
) -> Joystick<'d, VRX, VRY>
where
    VRX: AdcChannel + AnalogPin,
    VRY: AdcChannel + AnalogPin,
{
    let mut config = AdcConfig::<ADC2>::new();
    let vrx = config.enable_pin(vrx_pin, Attenuation::_11dB);
    let vry = config.enable_pin(vry_pin, Attenuation::_11dB);
    let adc = Adc::new(adc2, config);

    Joystick { adc, vrx, vry }
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
