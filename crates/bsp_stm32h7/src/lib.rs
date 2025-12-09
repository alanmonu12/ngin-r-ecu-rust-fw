#![no_std]


// Re-exportamos el HAL para que los ejemplos lo puedan usar
pub use stm32h7xx_hal as hal;
pub use ecu_traits;

pub mod injector;