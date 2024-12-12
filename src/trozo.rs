pub struct Trozo {
    bloques: Vec<u8>,
    pub n_vertices: usize,
    pub vbo: Option<u32>,
}

pub const MIDA: usize = 64;

#[rustfmt::skip]
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

impl Trozo {
    pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
        self.bloques[MIDA * MIDA * y + MIDA * z + x] // TODO: Changed
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: u8) {
        self.bloques[MIDA * MIDA * y + MIDA * z + x] = val; // TODO: Changed
    }

    pub fn new() -> Trozo {
        Trozo {
            bloques: vec![0; MIDA * MIDA * MIDA],
            vbo: None,
            n_vertices: 0,
        }
    }



    pub fn dibuja(&self) {
        unsafe {
            if let Some(vbo) = self.vbo {
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (4 * 6) as _, std::ptr::null());
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, (4 * 6) as _, 12 as *const _);
                gl::EnableVertexAttribArray(1);

                gl::VertexAttribPointer(2, 1, gl::FLOAT, gl::FALSE, (4 * 6) as _, 20 as *const _);
                gl::EnableVertexAttribArray(2);
                gl::DrawArrays(gl::TRIANGLES, 0, self.n_vertices as _);
            } 
        }
    }
}