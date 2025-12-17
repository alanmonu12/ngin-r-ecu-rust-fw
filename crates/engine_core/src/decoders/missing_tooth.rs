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
            rpm_instant: 0,  // Cruda (para lógica interna)
            rpm_filtered: 0, // Suavizada (para tablas)
            noise_filter_ratio: 0.25,
            max_tooth_time_us: 500_000,
            filter_alpha: 85,
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
    fn on_edge(&mut self, timestamp_us: u32) -> DecoderEvent {

        // Chequeo de Timeout / Stall
        // Calculamos cuánto tiempo pasó REALMENTE desde la última vez que vimos un diente
        let time_since_last = timestamp_us.wrapping_sub(self.last_timestamp);

        // Si ha pasado más de medio segundo, asumimos que el motor se paró
        // y este es un nuevo arranque.
        if !self.first_edge && time_since_last > self.max_tooth_time_us {
            self.reset();
            // Y ahora nos comportamos como si fuera el first_edge
        }

        //  Lógica de Primer Diente (o después de un Reset)
        if self.first_edge {
            self.first_edge = false;
            self.last_timestamp = timestamp_us;
            return DecoderEvent::None;
        }

        // 1. Calcular cuánto tiempo pasó desde el último diente (dt)
        // Manejamos el desbordamiento del reloj (u32 overflow)
        let delta = timestamp_us.wrapping_sub(self.last_timestamp);

        if self.last_delta > 0 {
            let min_valid_delta = (self.last_delta as f32 * self.noise_filter_ratio) as u32;
            if delta < min_valid_delta {
                // Es ruido (Spark noise), lo ignoramos completamente.
                // NO actualizamos last_timestamp, hacemos de cuenta que no pasó.
                return DecoderEvent::Noise; 
            }
        }

        self.last_timestamp = timestamp_us;

        // Filtro básico de ruido: si el pulso es absurdamente rápido, ignorar
        if delta < 5 { return DecoderEvent::None; }

        let mut event = DecoderEvent::ToothProcessed;

        // 2. Detección del Hueco (Gap Detection)
        // Si el tiempo actual es > 1.5 veces el anterior, es un hueco.
        // Usamos math entera: (current * 2) > (last * 3) equivale a current > last * 1.5
        if self.last_delta > 0 && (delta * 2) > (self.last_delta * 3) {
            // ¡HUECO ENCONTRADO!
            // En una rueda 60-2, el hueco significa que estamos llegando al diente 0.
            self.current_tooth_idx = 0;
            
            if !self.synced {
                self.synced = true;
                event = DecoderEvent::SyncGained;
            }
        } else {
            // Diente normal
            self.current_tooth_idx += 1;
            
            // Protección: Si contamos más dientes de los que existen, perdimos sincro
            let real_teeth = self.teeth_total - self.teeth_missing;
            if self.current_tooth_idx >= real_teeth {
                self.synced = false;
                self.current_tooth_idx = 0;
                event = DecoderEvent::SyncLost;
            }
        }

        // 3. Cálculo de RPM (Simplificado)
        // RPM = (1 / tiempo_vuelta_minutos)
        // Por ahora, calculamos RPM instantáneas basadas en el último diente
        // Ojo: En el hueco el RPM parecerá caer, hay que filtrar eso en el futuro.
        if delta > 0 {
            // 1. Calcular Instantánea
            let factor: u32 = 60_000_000 / (self.teeth_total as u32);
            let raw_rpm = (factor / delta) as u16;
            self.rpm_instant = raw_rpm;

            // 2. Calcular Filtrada (EMA)
            // Usamos math entera para velocidad: (Old * alpha + New * (100-alpha)) / 100
            let alpha = self.filter_alpha;
            let inv_alpha = 100 - alpha;
            
            let smooth = (
                (self.rpm_filtered as u32 * alpha) + 
                (raw_rpm as u32 * inv_alpha)
            ) / 100;

            self.rpm_filtered = smooth as u16;
            
        }

        self.last_delta = delta;
        event
    }

    fn get_angle(&self) -> f32 {
        // Angulo simple: Diente * GradosPorDiente
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