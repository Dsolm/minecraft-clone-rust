use cgmath::{Matrix, Matrix4};

use crate::{camera::Camera, trozo::{self, Trozo}};

pub struct Mundo {
    trozos: Vec<trozo::Trozo>
}

pub const MIDA_MUNDO: usize = 32;
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

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: u8) {
        let trozo_x = x / trozo::MIDA;
        let trozo_y = y / trozo::MIDA;
        let trozo_z = z / trozo::MIDA;

        let x = x % trozo::MIDA;
        let y = y % trozo::MIDA;
        let z = z % trozo::MIDA;

        self.trozos[trozo_x + MIDA_MUNDO * trozo_z + MIDA_MUNDO * MIDA_MUNDO * trozo_y].set(x,y,z, val)
    }

    pub fn genera_todas_las_mallas(&mut self) {
        for trozo in self.trozos.iter_mut() {
            trozo.genera_mallas();
        }
    }

    pub fn dibuja(&self, camera: &Camera, shader_program: u32) {
        let location = unsafe { gl::GetUniformLocation(shader_program, c"MVP".as_ptr() as _) };
        let vp = camera.build_view_projection_matrix();
        for y in 0..MIDA_MUNDO  {
            for z in 0..MIDA_MUNDO  {
                for x in 0..MIDA_MUNDO  {
                    let m = Matrix4::from_translation(((x*trozo::MIDA) as f32, (y*trozo::MIDA) as f32, (z*trozo::MIDA) as f32).into());
                    let mvp = vp * m;
                    unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, mvp.as_ptr()) };
                    self.trozos[x + MIDA_MUNDO * z + MIDA_MUNDO * MIDA_MUNDO * y].dibuja();
                    
                }
            }
        }
    }

}