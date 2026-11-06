#![no_std]

use esp_hal::{
    Blocking,
    analog::adc::{Adc, AdcChannel, AdcConfig, AdcPin, Attenuation},
    gpio::AnalogPin,
    peripherals::ADC2,
};
use num_traits::float::FloatCore;

#[derive(Debug, Clone, Copy)]
pub struct AdcError;

const MIN_RAW_INPUT: i32 = 0;
const MAX_RAW_INPUT: i32 = 4096;
const MIN_DEGREE: i32 = 0;
const MAX_DEGREE: i32 = 180;

pub struct Joystick<'d, VRX, VRY> {
    adc: Adc<'d, ADC2<'d>, Blocking>,
    vrx: AdcPin<VRX, ADC2<'d>>,
    vry: AdcPin<VRY, ADC2<'d>>,
}

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

fn map_joystick_raw_to_degree(input: i32) -> i32 {
    let degree = ((input as f32 - MIN_RAW_INPUT as f32)
        / (MAX_RAW_INPUT as f32 - MIN_RAW_INPUT as f32))
        * (MAX_DEGREE as f32 - MIN_DEGREE as f32)
        + MIN_DEGREE as f32;

    degree.round() as i32
}
