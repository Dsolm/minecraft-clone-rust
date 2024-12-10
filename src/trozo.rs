pub struct Trozo {
    bloques: Vec<u8>,
    n_vertices: usize,
    vbo: Option<u32>,
}

pub const MIDA: usize = 32;

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
        self.bloques[MIDA * MIDA * z as usize + MIDA * y as usize + x as usize]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: u8) {
        self.bloques[MIDA * MIDA * z as usize + MIDA * y as usize + x as usize] = val;
    }

    pub fn new() -> Trozo {
        Trozo {
            bloques: vec![0; MIDA * MIDA * MIDA],
            vbo: None,
            n_vertices: 0,
        }
    }

    fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
        self.get(x, y, z) == 0
    }

    fn touches_air(&self, x: usize, y: usize, z: usize) -> bool {
        (x < MIDA - 1 && self.is_air(x + 1, y, z))
            || (x > 0 && self.is_air(x - 1, y, z))
            || (y < MIDA - 1 && self.is_air(x, y + 1, z))
            || (y > 0 && self.is_air(x, y - 1, z))
            || (z < MIDA - 1 && self.is_air(x, y, z + 1))
            || (z > 0 && self.is_air(x, y, z - 1))
    }

    fn to_vertex(&self) -> Vec<f32> {
        let mut vertices = vec![];

        for z in 0..MIDA {
            for y in 0..MIDA {
                for x in 0..MIDA {
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
        // println!(
        //     "Emiting: {} vertexs {} bytes in total",
        //     vertices.len(),
        //     vertices.len() * size_of::<f32>()
        // );
        vertices
    }

    pub fn genera_mallas(&mut self) {
        let verts = self.to_vertex();
        if !verts.is_empty() {
            let mut vbo = 0;
            unsafe {
                gl::GenBuffers(1, &mut vbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (verts.len() * 4) as isize,
                    verts.as_ptr().cast(),
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (4 * 6) as _, std::ptr::null());
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, (4 * 6) as _, 12 as *const _);
                gl::EnableVertexAttribArray(1);

                gl::VertexAttribPointer(2, 1, gl::FLOAT, gl::FALSE, (4 * 6) as _, 20 as *const _);
                gl::EnableVertexAttribArray(2);
            }
            if let Some(viejo_vbo) = self.vbo {
                unsafe {
                    gl::DeleteBuffers(1, &viejo_vbo);
                }
            }

            self.vbo = Some(vbo);
            self.n_vertices = verts.len() / 6;
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