use crate::trozo::{self, Trozo};

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

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: u8) {
        let trozo_x = x / trozo::MIDA;
        let trozo_y = y / trozo::MIDA;
        let trozo_z = z / trozo::MIDA;

        let x = x % trozo::MIDA;
        let y = y % trozo::MIDA;
        let z = z % trozo::MIDA;

        self.trozos[trozo_x + MIDA_MUNDO * trozo_y + MIDA_MUNDO * MIDA_MUNDO * trozo_z].set(x,y,z, val)
    }

    pub fn genera_todas_las_mallas(&mut self) {
        for trozo in self.trozos.iter_mut() {
            trozo.genera_mallas();
        }
    }

    pub fn dibuja(&self) {
        for trozo in self.trozos.iter() {
            trozo.dibuja();
        }
    }

}