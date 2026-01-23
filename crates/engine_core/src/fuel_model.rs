// use libm::{powf, expf}; // Importamos funciones matemáticas si fueran necesarias

// Constante de los gases ideales para el aire (J / kg*K)
const R_SPECIFIC_AIR: f32 = 287.05;

/// Representa el modelo físico de combustible (Speed Density)
#[derive(Debug, Clone)]
pub struct SpeedDensity {
    /// Volumen de un solo cilindro en Litros (ej. 0.5L para un motor 2.0L de 4 cil)
    cylinder_volume_l: f32,
    
    /// Flujo del inyector en gramos por segundo (g/s)
    /// Nota: Usamos g/s porque es más fácil para la física. 
    /// (1 cc/min gasolina ~ 0.0123 g/s)
    injector_flow_gps: f32,
}

impl SpeedDensity {
    /// Constructor
    /// displacement_cc: Cilindrada total del motor
    /// cylinders: Número de cilindros
    /// injector_cc_min: Flujo del inyector en cc/min
    pub fn new(displacement_cc: f32, cylinders: u8, injector_cc_min: f32) -> Self {
        let vol_per_cyl_cc = displacement_cc / (cylinders as f32);
        let vol_per_cyl_l = vol_per_cyl_cc / 1000.0;

        // Densidad aproximada gasolina: 0.74 g/cc
        // (cc/min * 0.74) / 60 = g/s
        let flow_gps = (injector_cc_min * 0.74) /  60.0;

        Self {
            cylinder_volume_l: vol_per_cyl_l,
            injector_flow_gps: flow_gps,
        }
    }

    /// Calcula la masa de aire (gramos) que entra al cilindro
    /// map_kpa: Presión absoluta del múltiple (kPa)
    /// iat_c: Temperatura del aire de admisión (°C)
    /// ve_percent: Eficiencia Volumétrica de la tabla (0.0 a 100.0)
    pub fn calculate_air_mass(&self, map_kpa: f32, iat_c: f32, ve_percent: f32) -> f32 {
        // Se convierte temperatua a Kelvin
        let temp_k = iat_c + 273.15;

        // Presion a pascales
        let pressure_pa = map_kpa * 1000.0;

        // Ley de gases ideales Densidad = P / (R * T)
        let air_density = pressure_pa / (R_SPECIFIC_AIR * temp_k);

        // Se calcula la masa teorica total Masa Teórica = Volumen Cilindro * Densidad
        let theoretical_mass_g = self.cylinder_volume_l * air_density;

        // La masa real depende de la eficiencia volumetrica
        let real_mass_g = theoretical_mass_g * (ve_percent / 100.0);

        real_mass_g
    }

    /// Calcula el ancho de pulso REQUERIDO (Effective Pulse Width)
    /// afr_target: Relación Aire/Combustible deseada (ej. 14.7 para estoico, 12.5 potencia)
    pub fn calculate_pulse_width_us(
        &self, 
        air_mass_g: f32, 
        afr_target: f32
    ) -> u32 {
        if afr_target <= 0.0 { return 0; }

        // 1. Calcular masa de combustible necesaria
        let fuel_mass_g = air_mass_g / afr_target;

        // 2. Calcular tiempo necesario (segundos) = MasaRequerida / FlujoInyector
        let time_sec = fuel_mass_g / self.injector_flow_gps;

        // 3. Convertir a microsegundos
        (time_sec * 1_000_000.0) as u32
    }



}