use ecu_traits::engine_io::IgnitionCoil;
use embedded_hal::digital::v2::OutputPin;


#[derive(Debug)]
pub enum IgnitionError {
    ElectricalFailure,
}

/// Driver generico para represtar una bobina
/// P: pin fisico
pub struct Stm32h7Coil<P> {
    pin: P,
}

impl <P> Stm32h7Coil<P>
where
    P: OutputPin
{
    pub fn new(pin: P) -> Self {
        let mut driver = Self { pin };
        // Por seguridad el coil debe arrancar apagado
        let _ = driver.pin.set_low();
        driver
    }
}

impl <P> IgnitionCoil for Stm32h7Coil<P>
where
    P: OutputPin
{
    type Error = IgnitionError;

    fn start_dwell(&mut self) -> Result<(), Self::Error> {
        // High = Cargando (Para bobinas "Smart" comunes tipo LS/VAG)
        self.pin.set_high().map_err(|_| IgnitionError::ElectricalFailure)
    }

    fn coil_fire(&mut self) -> Result<(), Self::Error> {
        // High = Cargando (Para bobinas "Smart" comunes tipo LS/VAG)
        self.pin.set_low().map_err(|_| IgnitionError::ElectricalFailure)
    }
}
