#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DecoderEvent {
    None,
    ToothProcessed,    // Procesamos un diente normal
    SyncGained,        // ¡Acabamos de encontrar la posición! (Ej. Hueco detectado)
    SyncLost,          // Algo salió mal (ruido), perdimos la cuenta
    Noise,
}

/// Las caracterisitcas que cualuiqer decoder que se implemente debe cumplir
pub trait TriggerDecoder {
    /// Se llama cada que se detecta el trigger
    /// timestamp_us: el Tiempo exacto del evento
    fn on_edge(&mut self, timestamp_us: u32) -> DecoderEvent;

    /// Regresa el angulo en ese momento del ciguenal (0 a 720)
    fn get_angle(&self) -> f32;

    /// Regresa la velocidad del motor en rpm
    fn get_rpm(&self) -> u16;

    /// estado de la sincronizacion
    fn is_synced(&self) -> bool;

    fn get_instant_rpm(&self) -> u16;

    /// Se llama periódicamente (ej. en el main loop) para ver si
    /// el motor sigue vivo.
    /// Retorna `true` si se detectó un STALL (el motor se acaba de parar).
    fn check_stall(&mut self, current_time_us: u32) -> bool;

}