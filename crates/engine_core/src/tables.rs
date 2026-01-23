// use libm::{floorf, ceilf};

/// Se genera una estrucutra generica que representa una tabla 3D
/// X: Eje horizontal (ej. RPM)
/// Y: Eje vertical (ej. MAP/TPS)
/// Z: Datos (ej. VE % o Grados de Avance)
/// N: Número de columnas (X)
/// M: Número de filas (Y)
/// Una tabla 3D genérica (Superficie).
///
/// # Ejemplo
///
/// ```
/// use engine_core::tables::Table3D;
///
/// // Crear una tabla simple
/// let x = [0.0, 10.0];
/// let y = [0.0, 10.0];
/// let data = [[0.0, 100.0], [0.0, 100.0]];
/// let tabla = Table3D::new(x, y, data);
///
/// // Verificar interpolación
/// assert_eq!(tabla.interpolate(5.0, 0.0), 50.0);
/// ```
#[derive(Debug, Clone)]
pub struct Table3D<const N: usize, const M: usize> {
    pub x_axis: [f32; N], // Breakpoints RPM
    pub y_axis: [f32; M], // Breakpoints Carga
    pub data: [[f32; N]; M], // Matriz de datos [Fila][Columna]
}

impl<const N: usize, const M: usize> Table3D<N,M> {

    pub fn new(x_axis: [f32; N], y_axis: [f32; M], data: [[f32; N]; M]) -> Self {

        //TODO: es necesario implentar la lligca para validar que la tabla esta ordenada
        Self {x_axis, y_axis, data}
    } 

    /// Interpolacion bilineal
    pub fn interpolate(&self, x_val: f32, y_val: f32) -> f32 {
        
        // encontramos los indices para X
        let (x0_idx, x1_idx, x_factor) = self.find_axis_indices(&self.x_axis, x_val);

        // encontramos los indices para Y
        let (y0_idx, y1_idx, y_factor) = self.find_axis_indices(&self.y_axis, y_val);

        // Se debe obtener los vecinos de la celda en la que nos encontramos (x,y)
        let q11 = self.data[y0_idx][x0_idx];
        let q21 = self.data[y0_idx][x1_idx];
        let q12 = self.data[y1_idx][x0_idx];
        let q22 = self.data[y1_idx][x1_idx];

        // Aqui se hace la interpolacion
        let r1 = q11 * (1.0 - x_factor) + q21 * x_factor;
        let r2 = q12 * (1.0 - x_factor) + q22 * x_factor;
        
        let final_val = r1 * (1.0 - y_factor) + r2 * y_factor;
        
        final_val
    }

    /// Funcion para buscar los indices de las celdas
    /// Retorna: (indice_bajo, indice_alto, factor_de_peso)
    fn find_axis_indices(&self, axis: &[f32], value:f32) -> (usize, usize, f32) {

        // Validamos no salir de la tabla, si no, usamos el ultimo valor
        if value <= axis[0] { return (0,0,0.0);}
        if value >= axis[axis.len() - 1] {return (axis.len() - 1, axis.len() - 1, 0.0);}

        // busqueda lineal
        // TODO: evaluar busqueda bianria
        let mut idx = 0;
        for i in 0..axis.len()-1 {
            if value >= axis[i] && value < axis[i+1] {
                idx = i;
                break;
            }
        }
        
        let x0 = axis[idx];
        let x1 = axis[idx + 1];
        
        // Factor: ¿Qué tan cerca estamos de x1? (0.0 = en x0, 1.0 = en x1)
        let factor: f32 = (value - x0) / (x1 - x0);
        
        (idx, idx + 1, factor)
    }
}