#![no_std]

// Re-exportamos el HAL para que los ejemplos lo puedan usar
pub use stm32h7xx_hal as hal;
pub use ecu_traits;

pub mod injector;
pub mod pinout; // <--- Nuevo módulo

use hal::prelude::*;
use hal::gpio::GpioExt;
//use injector::Stm32h7Injector;
use pinout::{map_hardware, RawPorts};
//use embedded_hal::digital::OutputPin;
use hal::delay::Delay; // Importar Delay

// --- LA ESTRUCTURA DEL HARDWARE ---
// Esta struct representa tu ECU física completa.
pub struct Board {
    // Usamos los tipos abstractos definidos en pinout
    pub inyector_1: pinout::Inj1Driver,
    pub inyector_2: pinout::Inj2Driver,
    pub delay: Delay, // <--- NUEVO: La board incluye su propio reloj de espera
}

impl Board {
    /// Función que toma los periféricos "crudos" del micro y devuelve la Board configurada
    pub fn init() -> Self {
        // 1. Tomar periféricos crudos
        let dp = hal::pac::Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap(); // Necesario para el SYST (Delay)
        
        // 2. Configurar Relojes (RCC) - Centralizado aquí
        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();
        let rcc = dp.RCC.constrain();
        let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

        // 3. Dividir los GPIOs (Split)
        let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
        let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

        // 4. Empaquetamos los recursos crudos
        let raw_ports = RawPorts {
            gpioa,
            gpioe,
        };

        // 5. LLAMAMOS AL MAPEO (Aquí ocurre la abstracción)
        let hardware = map_hardware(raw_ports);

        let sys_delay = Delay::new(cp.SYST, ccdr.clocks);

        // 6. Retornar la estructura empaquetada
        Board {
            inyector_1: hardware.inj1,
            inyector_2: hardware.inj2,
            delay: sys_delay, // <--- Lo guardamos
        }
    }
}