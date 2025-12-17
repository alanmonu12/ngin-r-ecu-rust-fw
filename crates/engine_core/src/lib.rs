#![no_std]

// Permitimos std solo para correr tests unitarios en PC
#[cfg(test)]
extern crate std;

pub mod tables; // <--- Aquí vivirá la matemática
pub mod fuel_model;
pub mod decoder;

pub mod decoders;