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
    rpm: u16,
}

impl MissingToothDecoder {
    pub fn new(total: u8, missing: u8) -> Self {
        Self {
            teeth_total: total,
            teeth_missing: missing,
            current_tooth_idx: 0,
            synced: false,
            last_timestamp: 0,
            last_delta: 0,
            rpm: 0,
        }
    }
}

impl TriggerDecoder for MissingToothDecoder {
    fn on_edge(&mut self, timestamp_us: u32) -> DecoderEvent {
        // 1. Calcular cuánto tiempo pasó desde el último diente (dt)
        // Manejamos el desbordamiento del reloj (u32 overflow)
        let delta = timestamp_us.wrapping_sub(self.last_timestamp);
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
            // Formula aproximada diente a diente
             let degrees_per_tooth = 360.0 / (self.teeth_total as f32);
             let us_per_degree = delta as f32 / degrees_per_tooth;
             // ... lógica de conversión a RPM ...
             // Para simplificar este ejemplo, no llenaré la matemática completa de RPM aquí
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
        self.rpm
    }

    fn is_synced(&self) -> bool {
        self.synced
    }
}