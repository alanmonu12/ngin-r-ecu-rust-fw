#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use bsp_stm32h7::Board;
use bsp_stm32h7::hal::prelude::*;
use rtt_target::{rtt_init_print, rprintln};

// IMPORTE CRÍTICO: El trait para leer el ADC
use embedded_hal::adc::OneShot; 

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("--- TEST ANALOGICO ---");
    let mut board = Board::init();


    loop {
        // En este HAL, read devuelve un Result<u32, _> (incluso si configuraste 16 bits)
        // Usamos unwrap_or(0) para que no crashee si falla una lectura
        let tps_val: u32 = board.adc1.read(&mut board.tps).unwrap_or(0);
        let map_val: u32 = board.adc1.read(&mut board.map).unwrap_or(0);
        
        rprintln!("TPS: {} | MAP: {}", tps_val, map_val);

        board.delay.delay_ms(500u32);
    }
}