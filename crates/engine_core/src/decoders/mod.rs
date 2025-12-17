// 1. Declaramos que existe el archivo missing_tooth.rs
pub mod missing_tooth;

// 2. (Opcional pero recomendado) Re-exportamos para acortar el import
// Esto permite usar: engine_core::decoders::MissingToothDecoder
// En lugar de:       engine_core::decoders::missing_tooth::MissingToothDecoder
pub use missing_tooth::MissingToothDecoder;