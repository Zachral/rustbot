#![no_std]

use esp_hal::{
    Blocking,
    analog::adc::{Adc, AdcChannel, AdcConfig, AdcPin, Attenuation},
    gpio::AnalogPin,
    peripherals::ADC2,
};


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
    pub fn read_x(&mut self) -> nb::Result<u16, ()>{
        self.adc.read_oneshot(&mut self.vrx)
    }

    pub fn read_y(&mut self) -> nb::Result<u16, ()> {
        self.adc.read_oneshot(&mut self.vry)
    }
}

pub fn joystick_init<'d, VRX, VRY>(
    adc2: ADC2<'d>, 
    vrx_pin: VRX,
    vry_pin: VRY
) -> Joystick <'d, VRX, VRY>
where 
    VRX: AdcChannel + AnalogPin,
    VRY: AdcChannel + AnalogPin,

{
    let mut config = AdcConfig::<ADC2>::new();
    let vrx = config.enable_pin(vrx_pin, Attenuation::_11dB); 
    let vry = config.enable_pin(vry_pin, Attenuation::_11dB); 
    let adc = Adc::new(adc2, config);

    Joystick{adc, vrx, vry}
}
