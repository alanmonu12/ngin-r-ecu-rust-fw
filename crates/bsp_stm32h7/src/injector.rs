use ecu_traits::engine_io::Injector;
use embedded_hal::digital::v2::OutputPin;
//use stm32h7xx_hal as hal;
//use hal::device::TIM2; // Usaremos TIM2 como ejemplo, pero puede ser genérico
//use hal::timer::{Event, Counter};
//use hal::gpio::{Output, PushPull, Pin};
//use hal::prelude::*;
//use hal::delay::Delay;


// Definimos un error básico
#[derive(Debug)]
pub enum InjectorError {
    ElectricalFailure,
}

/// Driver de Inyector para STM32H7 solo la logica para dejar 
/// control de tiempos a la aplicacion
pub struct Stm32h7Injector<P> {
    // El pin físico (ej. PA1)
    pin: P,
    // Un contador de hardware que nos da la base de tiempo
    //timer: Delay, // 1 MHz = 1 tick por microsegundo
}

impl<P> Stm32h7Injector<P>
where
    P:OutputPin,
{
    pub fn new(pin: P) -> Self {
        let mut driver = Self { pin };
        let _ = driver.pin.set_low(); // Asegurar inyector cerrado al inicio
        driver
    }
}

impl<P> Injector for Stm32h7Injector<P> 
where 
    P: OutputPin
{
    type Error = InjectorError;

    fn open(&mut self) -> Result<(), Self::Error> {
        // 1. Abrimos el inyector (GPIO HIGH)
        let _ = self.pin.set_high();
        Ok(())
    }

    fn close(&mut self) -> Result<(), Self::Error> {
        let _ = self.pin.set_low();
        Ok(())
        //self.timer.cancel().ok();
    }
}