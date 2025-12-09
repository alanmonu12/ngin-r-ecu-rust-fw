// crates/bsp/src/pinout.rs
use crate::hal::gpio;
use crate::injector::Stm32h7Injector; // Importamos el driver genérico
use crate::ignition::Stm32h7Coil; // Importamos el driver genérico
use crate::sensors::Stm32h7HallSensor;

// --- DEFINICIONES FÍSICAS (El "define" de Rust) ---
// --- 1. DEFINICIÓN DE RECURSOS (EL "HARDWARE") ---
// Para no pasar 10 argumentos, agrupamos todos los puertos crudos del micro aquí.
pub struct RawPorts {
    pub gpioa: gpio::gpioa::Parts,
    pub gpioe: gpio::gpioe::Parts,
    // Agrega más puertos (GPIOB, GPIOC) si es necesario
}

// --- 2. LA MACRO MAESTRA ---
macro_rules! define_hardware_map {
    (
        outputs: {
            $($OutAlias:ident : $out_port:ident . $out_pin:ident as $OutPinType:ident),* $(,)?
        },
        inputs: {
            $($InAlias:ident : $in_port:ident . $in_pin:ident as $InPinType:ident),* $(,)?
        }
    ) => {
        // 1. Generar Tipos de SALIDA
        $(
            pub type $OutAlias = gpio::$OutPinType<gpio::Output<gpio::PushPull>>;
        )*

        // 2. Generar Tipos de ENTRADA (CORREGIDO)
        // En H7 HAL, 'Input' no lleva genéricos. El estado PullUp se configura
        // pero el tipo sigue siendo solo 'Input'.
        $(
            pub type $InAlias = gpio::$InPinType<gpio::Input>;
        )*

        // 3. Función extractora
        // Agregamos comas (,) al final de los grupos para forzar que sean tuplas
        // incluso si solo hay 1 elemento.
        fn extract_pins(ports: RawPorts) -> ( ($($OutAlias),*), ($($InAlias,)*) ) {
            (
                // Inicializar Salidas
                (
                    $(
                        ports.$out_port.$out_pin.into_push_pull_output()
                    ),*
                ),
                // Inicializar Entradas
                (
                    $(
                        ports.$in_port.$in_pin.into_pull_up_input()
                    ),* , // <--- Coma mágica para evitar warning de paréntesis
                )
            )
        }
    };
}

// --- 3. TU TABLA DE CONFIGURACIÓN (AQUÍ ES DONDE EDITAS) ---
// ¡Esta es la única fuente de verdad
// Si cambias el PCB, solo cambias la línea correspondiente aquí.
define_hardware_map!(
    outputs: {
        // Alias      : Puerto . Pin   as Tipo
        Inj1Pin       : gpioe  . pe2   as PE2,
        Inj2Pin       : gpioe  . pe3   as PE3,
        Inj3Pin       : gpioe  . pe4   as PE4,
        Inj4Pin       : gpioe  . pe5   as PE5,

        Ign1Pin       : gpioa  . pa0   as PA0,
        Ign2Pin       : gpioa  . pa1   as PA1,
        Ign3Pin       : gpioa  . pa2   as PA2,
        Ign4Pin       : gpioa  . pa3   as PA3,
    },
    inputs: {
        // Alias      : Puerto . Pin   as Tipo
        CkpPin        : gpioa  . pa4   as PA4,
    }
);

// --- 2. DEFINICIÓN DE TIPOS (LOS "ALIAS") ---

// Definimos el driver ya configurado
pub type Inj1Driver = Stm32h7Injector<Inj1Pin>;
pub type Inj2Driver = Stm32h7Injector<Inj2Pin>;
pub type Inj3Driver = Stm32h7Injector<Inj3Pin>;
pub type Inj4Driver = Stm32h7Injector<Inj4Pin>;

pub type Ign1Driver = Stm32h7Coil<Ign1Pin>;
pub type Ign2Driver = Stm32h7Coil<Ign2Pin>;
pub type Ign3Driver = Stm32h7Coil<Ign3Pin>;
pub type Ign4Driver = Stm32h7Coil<Ign4Pin>;

pub type CkpDriver = Stm32h7HallSensor<CkpPin>;

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

    pub ckp: CkpDriver,
}

// --- 3. EL MAPEO (LA "CONEXIÓN") ---
pub fn map_hardware(ports: RawPorts) -> ConfiguredHardware {

    let (
        (p_inj1,
        p_inj2,
        p_inj3,
        p_inj4,

        p_ign1,
        p_ign2,
        p_ign3,
        p_ign4),
        (p_ckp,)
    ) = extract_pins(ports);

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

        ckp: Stm32h7HallSensor::new(p_ckp),
    }
}