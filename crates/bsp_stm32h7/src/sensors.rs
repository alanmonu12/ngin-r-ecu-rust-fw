use ecu_traits::engine_io::RotationSensor;
use embedded_hal::digital::v2::InputPin;
use crate::hal::gpio::{ExtiPin, Edge}; // <--- Importamos Edge
use crate::hal::device::{SYSCFG, EXTI}; // <--- Importamos los periféricos necesarios

#[derive(Debug)]
pub enum SensorError {
    ReadError,
}


/// Driver generico para sensores de tipo HALL/Opticos
/// P: El pin fisico (debe soportar Input y EXTI)
pub struct Stm32h7HallSensor<P> {
    pin: P,
}

impl<P> Stm32h7HallSensor<P>
where
    P: InputPin + ExtiPin
{
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Configura el hardware para disparar interrupciones
    /// Este método actúa como puente hacia el Pin interno.
    pub fn enable_interrupt(&mut self, syscfg: &mut SYSCFG, exti: &mut EXTI) {
        // 1. Rutear el GPIO hacia el controlador de interrupciones
        self.pin.make_interrupt_source(syscfg);
        
        // 2. Configurar disparo en Flanco de Subida (Rising)
        // (Cuando el diente del engranaje pasa)
        self.pin.trigger_on_edge(exti, Edge::Rising);
        
        // 3. Desenmascarar (Habilitar) la interrupción
        self.pin.enable_interrupt(exti);
    }
}

impl<P> RotationSensor for Stm32h7HallSensor<P>
where
    P: InputPin + ExtiPin
{
    type Error = SensorError;

    fn get_state(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_high().map_err(|_| SensorError::ReadError)
    }

    fn clear_sensor_flag(&mut self) {
        self.pin.clear_interrupt_pending_bit();
    }
}
