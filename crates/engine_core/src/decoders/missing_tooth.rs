/// Decoder genérico para ruedas con dientes faltantes (ej. 60-2, 36-1)
use crate::decoder::{TriggerDecoder, DecoderEvent};

pub struct MissingToothDecoder {
    // Configuración
    teeth_total: u8,     // Ej. 60
    teeth_missing: u8,   // Ej. 2
    
    // Estado
    current_tooth_idx: u8,
    synced: bool,
    last_timestamp: u32,
    last_delta: u32,     // Tiempo que duró el diente anterior
    rpm_instant: u16,  // Cruda (para lógica interna)
    rpm_filtered: u16, // Suavizada (para tablas)
    first_edge: bool,
    noise_filter_ratio: f32,
    max_tooth_time_us: u32,

    // Factor de suavizado (0 a 100).
    // 80 significa: El nuevo valor pesa 20%, el historial pesa 80%.
    filter_alpha: u32,
    stall_timeout_us: u32,
}

impl MissingToothDecoder {
    pub fn new(total: u8, missing: u8) -> Self {
        Self {
            teeth_total: total,
            teeth_missing: missing,
            current_tooth_idx: 0,
            synced: false,
            first_edge: true,
            last_timestamp: 0,
            last_delta: 0,
            rpm_instant: 0,
            rpm_filtered: 0, 
            noise_filter_ratio: 0.25,
            max_tooth_time_us: 500_000,
            filter_alpha: 20,
            stall_timeout_us: 500_000,
        }
    }

    // Método auxiliar para limpiar estado (público por si el firmware quiere forzarlo)
    pub fn reset(&mut self) {
        self.first_edge = true;
        self.synced = false;
        self.current_tooth_idx = 0;
        self.last_delta = 0;
        self.rpm_filtered = 0;
        self.rpm_instant = 0;
    }
}

impl TriggerDecoder for MissingToothDecoder {

    /// Procesa una interrupción de flanco (diente) del sensor de posición.
    ///
    /// Esta función implementa la máquina de estados principal del decodificador:
    /// 1. **Filtrado:** Descarta pulsos espurios (ruido) basados en el `last_delta`.
    /// 2. **Sincronización:** Detecta el hueco (missing tooth) comparando la duración
    ///    del pulso actual vs el anterior (Ratio 1.5x).
    /// 3. **Estimación:** Calcula RPM instantáneas y suavizadas (EMA).
    ///
    /// # Argumentos
    ///
    /// * `timestamp_us` - Tiempo monótono en microsegundos del evento actual.
    ///
    /// # Retorno
    ///
    /// Retorna un `DecoderEvent` que indica si se encontró un diente, se logró
    /// sincronía (Gap encontrado) o si hubo ruido.
    fn on_edge(&mut self, timestamp_us: u32) -> DecoderEvent {

        let time_since_last = timestamp_us.wrapping_sub(self.last_timestamp);

        if !self.first_edge && time_since_last > self.max_tooth_time_us {
            self.reset();
        }

        if self.first_edge {
            self.first_edge = false;
            self.last_timestamp = timestamp_us;
            return DecoderEvent::None;
        }

        let delta = timestamp_us.wrapping_sub(self.last_timestamp);

        if self.last_delta > 0 {
            let min_valid_delta = (self.last_delta as f32 * self.noise_filter_ratio) as u32;
            if delta < min_valid_delta {
                return DecoderEvent::Noise; 
            }
        }

        self.last_timestamp = timestamp_us;

        if delta < 5 { return DecoderEvent::Noise; }

        let mut event = DecoderEvent::ToothProcessed;

        if self.last_delta > 0 && (delta * 2) > (self.last_delta * 3) {
            self.current_tooth_idx = 0;
            
            if !self.synced {
                self.synced = true;
                event = DecoderEvent::SyncGained;
            }

        } else {
            self.current_tooth_idx += 1;

            // Solo calculamos si tenemos sync y NO es el primer diente tras el hueco
            // El diente 1 es el que causa esos picos extraños que ves en la foto.
            if delta > 0 && self.current_tooth_idx > 1 {
                
                let factor: u32 = 60_000_000 / (self.teeth_total as u32);
                let raw_rpm = (factor / delta) as u16;
                
                // Solo ahora actualizamos la instantánea
                self.rpm_instant = raw_rpm;

                // Aplicar el filtro a la filtrada
                let alpha = self.filter_alpha;
                let inv_alpha = 100 - alpha;
                let smooth = ((raw_rpm as u32 * alpha) + (self.rpm_filtered as u32 * inv_alpha)) / 100;
                self.rpm_filtered = smooth as u16;
            }
            
            let real_teeth = self.teeth_total - self.teeth_missing;
            if self.current_tooth_idx >= real_teeth {
                self.synced = false;
                self.current_tooth_idx = 0;
                event = DecoderEvent::SyncLost;
            }
        }

        self.last_delta = delta;
        event
    }

    fn check_stall(&mut self, current_time_us: u32) -> bool {
        // Cuidar el wrapping (desbordamiento) del reloj
        let delta = current_time_us.wrapping_sub(self.last_timestamp);

        if delta > self.stall_timeout_us {
            // ¡STALL DETECTADO!
            if self.rpm_filtered > 0 {
                self.rpm_filtered = 0;
                self.synced = false;

                return true;
            }
        }
        false
    }

    fn get_angle(&self) -> f32 {
        let deg_per_tooth = 360.0 / (self.teeth_total as f32);
        (self.current_tooth_idx as f32) * deg_per_tooth
    }

    fn get_rpm(&self) -> u16 {
        self.rpm_filtered
    }

    fn get_instant_rpm(&self) -> u16 {
        self.rpm_instant
    }

    fn is_synced(&self) -> bool {
        self.synced
    }
}