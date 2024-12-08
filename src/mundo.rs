pub struct Mundo {
    bloques: Vec<u8>,
}

pub const MIDA: usize = 1024;
pub const MIDA_U16: u16 = MIDA as u16;

pub const VERTICES_CUADRADO: [f32; 36*6] = [
    // ESTE
    -0.5, -0.5, -0.5,  0.0, 0.0, 0.8,
    0.5, -0.5, -0.5,   1.0, 0.0, 0.8,
    0.5,  0.5, -0.5,   1.0, 1.0, 0.8,
    0.5,  0.5, -0.5,   1.0, 1.0, 0.8,
    -0.5,  0.5, -0.5,  0.0, 1.0, 0.8,
    -0.5, -0.5, -0.5,  0.0, 0.0, 0.8,

    // OESTE
    0.5,  0.5,  0.5,  1.0, 1.0, 0.8,
    0.5, -0.5,  0.5,  1.0, 0.0, 0.8,
    -0.5, -0.5,  0.5,  0.0, 0.0, 0.8,
    -0.5, -0.5,  0.5,  0.0, 0.0, 0.8,
    -0.5,  0.5,  0.5,  0.0, 1.0, 0.8,
    0.5,  0.5,  0.5,  1.0, 1.0,  0.8,

    // SUR
    -0.5, -0.5, -0.5,  0.0, 1.0, 0.5,
    -0.5,  0.5, -0.5,  1.0, 1.0, 0.5,
    -0.5,  0.5,  0.5,  1.0, 0.0, 0.5,  
    -0.5,  0.5,  0.5,  1.0, 0.0, 0.5,
    -0.5, -0.5,  0.5,  0.0, 0.0, 0.5,
    -0.5, -0.5, -0.5,  0.0, 1.0, 0.5,


    // NORTE
    0.5,  0.5,  0.5,  1.0, 0.0, 0.5,  
    0.5,  0.5, -0.5,  1.0, 1.0, 0.5,
    0.5, -0.5, -0.5,  0.0, 1.0, 0.5,
    0.5, -0.5, -0.5,  0.0, 1.0, 0.5,
    0.5, -0.5,  0.5,  0.0, 0.0, 0.5,
    0.5,  0.5,  0.5,  1.0, 0.0, 0.5,

    // BOTTOM
    0.5, -0.5,  0.5,  1.0, 0.0,  0.3,
    0.5, -0.5, -0.5,  1.0, 1.0,  0.3,
    -0.5, -0.5, -0.5,  0.0, 1.0, 0.3,  
    -0.5, -0.5, -0.5,  0.0, 1.0, 0.3,
    -0.5, -0.5,  0.5,  0.0, 0.0, 0.3,
    0.5, -0.5,  0.5,  1.0, 0.0,  0.3,

    // TOP
    -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 0.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,
];

impl Mundo {
    pub fn get(&self, x: u16, y: u16, z: u16) -> u8 {
        self.bloques[MIDA * MIDA * z as usize + MIDA * y as usize + x as usize]
    }

    pub fn set(&mut self, x: u16, y: u16, z: u16, val: u8) {
        self.bloques[MIDA * MIDA * z as usize + MIDA * y as usize + x as usize] = val;
    }

    pub fn new() -> Mundo {
        Mundo { bloques: vec![0; MIDA * MIDA * MIDA] }
    }

    fn is_air(&self, x: u16, y: u16, z: u16) -> bool {
        self.get(x, y, z) == 0
    }

    fn touches_air(&self, x: u16, y: u16, z: u16) -> bool {
        (x < MIDA_U16 - 1 && self.is_air(x + 1, y, z))
            || (x > 0 && self.is_air(x - 1, y, z))
            || (y < MIDA_U16 - 1 && self.is_air(x, y + 1, z))
            || (y > 0 && self.is_air(x, y - 1, z))
            || (z < MIDA_U16 - 1 && self.is_air(x, y, z + 1))
            || (z > 0 && self.is_air(x, y, z - 1))
    }

    pub fn to_vertex(&self) -> Vec<f32> {
        let mut vertices = vec![];

        for z in 0..MIDA as u16 {
            for y in 0..MIDA as u16 {
                for x in 0..MIDA as u16 {
                    if self.get(x, y, z) == 0 {
                        continue;
                    }
                    if self.touches_air(x, y, z) {
                        for i in 0..36 {
                            vertices.push(VERTICES_CUADRADO[i * 6] + x as f32 + 10.0);
                            vertices.push(VERTICES_CUADRADO[i * 6 + 1] + y as f32 + 10.0);
                            vertices.push(VERTICES_CUADRADO[i * 6 + 2] + z as f32 + 10.0);

                            let tipo = self.get(x, y, z);

                            const MIDA_TEXTURA_0_A_1: f32 = 32.0 / 256.0;
                            let principio_x = ((tipo - 1) * 32) as f32 / 256.0;
                            let principio_y = 0.0;

                            let uv_x =
                                principio_x + MIDA_TEXTURA_0_A_1 * VERTICES_CUADRADO[i * 6 + 3];
                            let uv_y =
                                principio_y + MIDA_TEXTURA_0_A_1 * VERTICES_CUADRADO[i * 6 + 4];

                            vertices.push(uv_x);
                            vertices.push(uv_y);

                            vertices.push(VERTICES_CUADRADO[i * 6 + 5]);
                        }
                    }
                }
            }
        }
        println!(
            "Emiting: {} vertexs {} bytes in total",
            vertices.len(),
            vertices.len() * size_of::<f32>()
        );
        vertices
    }
}
