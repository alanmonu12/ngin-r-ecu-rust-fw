// crates/bsp/src/pinout.rs
use crate::hal::gpio;
use crate::injector::Stm32h7Injector; // Importamos el driver genérico
use crate::ignition::Stm32h7Coil; // Importamos el driver genérico

// --- DEFINICIONES FÍSICAS (El "define" de Rust) ---
// --- 1. DEFINICIÓN DE RECURSOS (EL "HARDWARE") ---
// Para no pasar 10 argumentos, agrupamos todos los puertos crudos del micro aquí.
pub struct RawPorts {
    pub gpioa: gpio::gpioa::Parts,
    pub gpioe: gpio::gpioe::Parts,
    // Agrega más puertos (GPIOB, GPIOC) si es necesario
}

macro_rules! define_hardware_map {
    (
        $(
            // Formato: Alias : Puerto . Pin as TipoFisico
            $Alias:ident : $port_field:ident . $pin_field:ident as $PinType:ident
        ),* $(,)?
    ) => {
        // A) Generación de Tipos
        $(
            pub type $Alias = gpio::$PinType<gpio::Output<gpio::PushPull>>;
        )*

        // B) Función interna para extraer y configurar
        // Devuelve una tupla con todos los pines inicializados en orden
        fn extract_pins(ports: RawPorts) -> ( $($Alias),* ) {
            (
                $(
                    ports.$port_field.$pin_field.into_push_pull_output()
                ),*
            )
        }
    };
}

// --- 3. TU TABLA DE CONFIGURACIÓN (AQUÍ ES DONDE EDITAS) ---
// ¡Esta es la única fuente de verdad
// Si cambias el PCB, solo cambias la línea correspondiente aquí.
define_hardware_map!(
    // Alias      : Puerto . Pin   as Tipo
    Inj1Pin       : gpioe  . pe2   as PE2,
    Inj2Pin       : gpioe  . pe3   as PE3,
    Inj3Pin       : gpioe  . pe4   as PE4,
    Inj4Pin       : gpioe  . pe5   as PE5,

    Ign1Pin       : gpioa  . pa0   as PA0,
    Ign2Pin       : gpioa  . pa1   as PA1,
    Ign3Pin       : gpioa  . pa2   as PA2,
    Ign4Pin       : gpioa  . pa3   as PA3
);

// --- 2. DEFINICIÓN DE TIPOS (LOS "ALIAS") ---
// Si cambias el PCB, modificas estas líneas:
//pub type Inj1Pin = gpio::PE3<gpio::Output<gpio::PushPull>>;
//pub type Inj2Pin = gpio::PE4<gpio::Output<gpio::PushPull>>;

//pub type Ign1Pin = gpio::PA2<gpio::Output<gpio::PushPull>>;
//pub type Ign2Pin = gpio::PA3<gpio::Output<gpio::PushPull>>;

// Ejemplo: Si cambiáramos a PA5, solo editaríamos arriba: gpio::PA5...

// Definimos el driver ya configurado
pub type Inj1Driver = Stm32h7Injector<Inj1Pin>;
pub type Inj2Driver = Stm32h7Injector<Inj2Pin>;
pub type Inj3Driver = Stm32h7Injector<Inj3Pin>;
pub type Inj4Driver = Stm32h7Injector<Inj4Pin>;

pub type Ign1Driver = Stm32h7Coil<Ign1Pin>;
pub type Ign2Driver = Stm32h7Coil<Ign2Pin>;
pub type Ign3Driver = Stm32h7Coil<Ign3Pin>;
pub type Ign4Driver = Stm32h7Coil<Ign4Pin>;

// Estructura que devuelve los pines ya convertidos en drivers
pub struct ConfiguredHardware {
    pub inj1: Inj1Driver,
    pub inj2: Inj2Driver,
    pub inj3: Inj3Driver,
    pub inj4: Inj4Driver,
    
    pub ing1: Ign1Driver,
    pub ing2: Ign2Driver,
    pub ing3: Ign3Driver,
    pub ing4: Ign4Driver,
}

// --- 3. EL MAPEO (LA "CONEXIÓN") ---
pub fn map_hardware(ports: RawPorts) -> ConfiguredHardware {

    let (
        p_inj1,
        p_inj2,
        p_inj3,
        p_inj4,

        p_ign1,
        p_ign2,
        p_ign3,
        p_ign4,
    ) = extract_pins(ports);

    // A) Extraemos los puertos que necesitamos
    //let gpioe = ports.gpioe;
    //let gpioa = ports.gpioa; // Lo tenemos disponible por si cambiamos de opinión

    // B) Inicializamos los pines FÍSICOS
    // SI CAMBIAS EL PCB, SOLO CAMBIAS ESTAS LÍNEAS AQUÍ:
    //let pin_fisico_1 = gpioe.pe3.into_push_pull_output();
    //let pin_fisico_2 = gpioe.pe4.into_push_pull_output();

    //let pin_fisico_3 = gpioa.pa2.into_push_pull_output();
    //let pin_fisico_4 = gpioa.pa3.into_push_pull_output();
    
    // C) Creamos los drivers
    ConfiguredHardware {
        inj1: Stm32h7Injector::new(p_inj1),
        inj2: Stm32h7Injector::new(p_inj2),
        inj3: Stm32h7Injector::new(p_inj3),
        inj4: Stm32h7Injector::new(p_inj4),

        ing1: Stm32h7Coil::new(p_ign1),
        ing2: Stm32h7Coil::new(p_ign2),
        ing3: Stm32h7Coil::new(p_ign3),
        ing4: Stm32h7Coil::new(p_ign4),
    }
}