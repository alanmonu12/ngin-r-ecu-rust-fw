// crates/bsp/src/pinout.rs
use crate::hal::gpio;
use crate::injector::Stm32h7Injector; // Importamos el driver genérico

// --- DEFINICIONES FÍSICAS (El "define" de Rust) ---
// --- 1. DEFINICIÓN DE RECURSOS (EL "HARDWARE") ---
// Para no pasar 10 argumentos, agrupamos todos los puertos crudos del micro aquí.
pub struct RawPorts {
    pub gpioa: gpio::gpioa::Parts,
    pub gpioe: gpio::gpioe::Parts,
    // Agrega más puertos (GPIOB, GPIOC) si tu PCB los usa
}

// --- 2. DEFINICIÓN DE TIPOS (LOS "ALIAS") ---
// Si cambias el PCB, modificas estas líneas:
pub type Inj1Pin = gpio::PE3<gpio::Output<gpio::PushPull>>;
pub type Inj2Pin = gpio::PE4<gpio::Output<gpio::PushPull>>;
// Ejemplo: Si cambiáramos a PA5, solo editaríamos arriba: gpio::PA5...

// Definimos el driver ya configurado
pub type Inj1Driver = Stm32h7Injector<Inj1Pin>;
pub type Inj2Driver = Stm32h7Injector<Inj2Pin>;

// Estructura que devuelve los pines ya convertidos en drivers
pub struct ConfiguredHardware {
    pub inj1: Inj1Driver,
    pub inj2: Inj2Driver,
}

// --- 3. EL MAPEO (LA "CONEXIÓN") ---
// Aquí es donde ocurre la magia. Esta función toma los puertos crudos
// y devuelve los objetos listos.
pub fn map_hardware(ports: RawPorts) -> ConfiguredHardware {
    // A) Extraemos los puertos que necesitamos
    let gpioe = ports.gpioe;
    let gpioa = ports.gpioa; // Lo tenemos disponible por si cambiamos de opinión

    // B) Inicializamos los pines FÍSICOS
    // SI CAMBIAS EL PCB, SOLO CAMBIAS ESTAS LÍNEAS AQUÍ:
    let pin_fisico_1 = gpioe.pe3.into_push_pull_output();
    let pin_fisico_2 = gpioe.pe4.into_push_pull_output();
    
    // C) Creamos los drivers
    ConfiguredHardware {
        inj1: Stm32h7Injector::new(pin_fisico_1),
        inj2: Stm32h7Injector::new(pin_fisico_2),
    }
}