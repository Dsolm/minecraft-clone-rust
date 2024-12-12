use cgmath::{Matrix, Matrix4};

use crate::{camera::Camera, trozo::{self, Trozo}};

pub struct Mundo {
    trozos: Vec<trozo::Trozo>
}

pub const MIDA_MUNDO: usize = 8;
pub const MIDA: usize = MIDA_MUNDO * trozo::MIDA;

impl Mundo {
    pub fn new() -> Mundo {
        let mut trozos = Vec::with_capacity(MIDA_MUNDO * MIDA_MUNDO * MIDA_MUNDO);
        for _ in 0..MIDA_MUNDO * MIDA_MUNDO * MIDA_MUNDO {
            trozos.push(Trozo::new());
        }
        Mundo {
            trozos,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
        let trozo_x = x / trozo::MIDA;
        let trozo_y = y / trozo::MIDA;
        let trozo_z = z / trozo::MIDA;

        let x = x % trozo::MIDA;
        let y = y % trozo::MIDA;
        let z = z % trozo::MIDA;

        self.trozos[trozo_x + MIDA_MUNDO * MIDA_MUNDO * trozo_y + MIDA_MUNDO * trozo_z].get(x,y,z)
    }

    pub fn get_mut_chunk_by_idx(&mut self, idx_x: usize, idx_y: usize, idx_z: usize) -> &mut Trozo {
        &mut self.trozos[idx_x + MIDA_MUNDO * MIDA_MUNDO * idx_y + MIDA_MUNDO * idx_z]
    }

    #[allow(dead_code)]
    pub fn get_chunk_by_idx(&self, idx_x: usize, idx_y: usize, idx_z: usize) -> &Trozo {
        &self.trozos[idx_x + MIDA_MUNDO * MIDA_MUNDO * idx_y + MIDA_MUNDO * idx_z]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: u8) {
        let trozo_x = x / trozo::MIDA;
        let trozo_y = y / trozo::MIDA;
        let trozo_z = z / trozo::MIDA;

        let x = x % trozo::MIDA;
        let y = y % trozo::MIDA;
        let z = z % trozo::MIDA;

        self.trozos[trozo_x + MIDA_MUNDO * trozo_z + MIDA_MUNDO * MIDA_MUNDO * trozo_y].set(x,y,z, val)
    }

    fn trozo_to_vertex(&self, chunk: (usize, usize, usize)) -> Vec<f32> {
        let mut vertices = vec![];
        vertices.reserve(trozo::MIDA.pow(3)*6*36);

        for z in chunk.2*trozo::MIDA..chunk.2*trozo::MIDA+trozo::MIDA {
            for y in chunk.1*trozo::MIDA..chunk.1*trozo::MIDA+trozo::MIDA {
                for x in chunk.0*trozo::MIDA..chunk.0*trozo::MIDA+trozo::MIDA {
                    if self.get(x, y, z) == 0 {
                        continue;
                    }
                    if self.touches_air(x, y, z) {
                        for i in 0..36 {
                            vertices.push(trozo::VERTICES_CUADRADO[i * 6] + x as f32 + 10.0);
                            vertices.push(trozo::VERTICES_CUADRADO[i * 6 + 1] + y as f32 + 10.0);
                            vertices.push(trozo::VERTICES_CUADRADO[i * 6 + 2] + z as f32 + 10.0);

                            let tipo = self.get(x, y, z);

                            const MIDA_TEXTURA_0_A_1: f32 = 32.0 / 256.0;
                            let principio_x = ((tipo - 1) * 32) as f32 / 256.0;
                            let principio_y = 0.0;

                            let uv_x =
                                principio_x + MIDA_TEXTURA_0_A_1 * trozo::VERTICES_CUADRADO[i * 6 + 3];
                            let uv_y =
                                principio_y + MIDA_TEXTURA_0_A_1 * trozo::VERTICES_CUADRADO[i * 6 + 4];

                            vertices.push(uv_x);
                            vertices.push(uv_y);

                            vertices.push(trozo::VERTICES_CUADRADO[i * 6 + 5]);
                        }
                    }
                }
            }
        }
        vertices
    }

    pub fn genera_mallas_de_trozo(&mut self, chunk: (usize, usize, usize)) {
        let verts = self.trozo_to_vertex(chunk);
        let chunk = self.get_mut_chunk_by_idx(chunk.0, chunk.1, chunk.2);

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

            if let Some(viejo_vbo) = chunk.vbo {
                unsafe {
                    gl::DeleteBuffers(1, &viejo_vbo);
                }
            }

            chunk.vbo = Some(vbo);
            chunk.n_vertices = verts.len() / 6;
        }
    }

    pub fn genera_todas_las_mallas(&mut self) {
        for y in 0..MIDA_MUNDO  {
            for z in 0..MIDA_MUNDO  {
                for x in 0..MIDA_MUNDO  {
                    self.genera_mallas_de_trozo((x,y,z));
                }
            }
        }
    }

    pub fn touches_air(&self, x: usize, y: usize, z: usize) -> bool {
        (x < MIDA - 1 && self.get(x + 1, y, z) == 0)
            || (x > 0 && self.get(x - 1, y, z) == 0)
            || (y < MIDA - 1 && self.get(x, y + 1, z) == 0)
            || (y > 0 && self.get(x, y - 1, z) == 0)
            || (z < MIDA - 1 && self.get(x, y, z + 1) == 0)
            || (z > 0 && self.get(x, y, z - 1) == 0)
    }


    pub fn dibuja(&self, camera: &Camera, shader_program: u32) {
        for y in 0..MIDA_MUNDO  {
            for z in 0..MIDA_MUNDO  {
                for x in 0..MIDA_MUNDO  {
                    self.trozos[x + MIDA_MUNDO * z + MIDA_MUNDO * MIDA_MUNDO * y].dibuja();
                    
                }
            }
        }
    }
}