// Interface genérica para cualquier inyector
pub trait Injector {
    type Error;

    /// Abre el inyector por una duración específica en microsegundos.
    /// Esta función debe ser NO bloqueante (el hardware hace el trabajo).
    fn open(&mut self) -> Result<(), Self::Error>;
    
    /// (Opcional) Cierra de emergencia
    fn close(&mut self) -> Result<(), Self::Error>;
}