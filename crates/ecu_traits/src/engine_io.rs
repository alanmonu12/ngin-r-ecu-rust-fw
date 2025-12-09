// Interface genérica para cualquier inyector
pub trait Injector {
    type Error;

    /// Abre el inyector por una duración específica en microsegundos.
    /// Esta función debe ser NO bloqueante (el hardware hace el trabajo).
    fn open(&mut self) -> Result<(), Self::Error>;
    
    /// (Opcional) Cierra de emergencia
    fn close(&mut self) -> Result<(), Self::Error>;
}

// Interface para la bobina de encendido
// Maneja el tiempo de carga (dwell) y el disparo
pub trait IgnitionCoil {
    type Error;

    /// Funcion para comenzar la carga de la bobina
    /// Se pone el PIN en HIGH para iniciar la carga
    fn start_dwell(&mut self) -> Result<(), Self::Error>;

    /// Se apaga la corriente para generar la chispa
    /// Se pone el PIN en LOW
    fn coil_fire(&mut self) -> Result<(), Self::Error>;
}

/// Interface para sensores de poscion (HAL/VR despues de acondicionamiento)
/// Se usan principalmente para CKP y CMP
pub trait RotationSensor {
    type Error;

    /// Leer estado actual
    fn get_state(&mut self) -> Result<bool, Self::Error>;

    /// Limpiar bandera de la interrupcion pendiente de HW
    fn clear_sensor_flag(&mut self);
}