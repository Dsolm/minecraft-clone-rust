pub struct Mundo {
    bloques: Vec<u8>
}

pub const MIDA: usize = 255;
pub const MIDA_U8: u8 = MIDA as u8;

pub const VERTICES_CUADRADO: [f32; 36*3] = [
	-1.0,-1.0,-1.0, // triangle 1 : begin
	-1.0,-1.0, 1.0,
	-1.0, 1.0, 1.0, // triangle 1 : end
	1.0, 1.0,-1.0, // triangle 2 : begin
	-1.0,-1.0,-1.0,
	-1.0, 1.0,-1.0, // triangle 2 : end
	1.0,-1.0, 1.0,
	-1.0,-1.0,-1.0,
	1.0,-1.0,-1.0,

	1.0, 1.0,-1.0,
	1.0,-1.0,-1.0,
	-1.0,-1.0,-1.0,

	-1.0,-1.0,-1.0,
	-1.0, 1.0, 1.0,
	-1.0, 1.0,-1.0,

	1.0,-1.0, 1.0,
	-1.0,-1.0, 1.0,
	-1.0,-1.0,-1.0,

	-1.0, 1.0, 1.0,
	-1.0,-1.0, 1.0,
	1.0,-1.0, 1.0,

	1.0, 1.0, 1.0,
	1.0,-1.0,-1.0,
	1.0, 1.0,-1.0,

	1.0,-1.0,-1.0,
	1.0, 1.0, 1.0,
	1.0,-1.0, 1.0,

	1.0, 1.0, 1.0,
	1.0, 1.0,-1.0,
	-1.0, 1.0,-1.0,

	1.0, 1.0, 1.0,
	-1.0, 1.0,-1.0,
	-1.0, 1.0, 1.0,

	1.0, 1.0, 1.0,
	-1.0, 1.0, 1.0,
	1.0,-1.0, 1.0
];

impl Mundo {
    pub fn get(&self, x: u8, y: u8, z: u8) -> u8 {
        self.bloques[MIDA * MIDA * z as usize + MIDA * y as usize + x as usize]
    }

    pub fn set(&mut self, x: u8, y: u8, z: u8, val: u8) {
        self.bloques[MIDA * MIDA * z as usize + MIDA * y as usize + x as usize] = val;
    }

    pub fn new() -> Mundo {
        let mut vector = Vec::new();
        vector.resize(MIDA * MIDA * MIDA, 0);

        Mundo {
            bloques: vector,
        }
    }


    fn is_air(&self, x: u8, y:u8, z:u8) -> bool {
        self.get(x, y, z) == 0
    }

    fn touches_air(&self, x: u8, y: u8, z:u8) -> bool {

        (x < MIDA_U8-1 && self.is_air(x+1,y,z)) || 
            (x > 0 && self.is_air(x-1,y,z)) || 
            (y < MIDA_U8-1 && self.is_air(x,y+1,z)) || 
            (y > 0 && self.is_air(x,y-1,z)) || 
            (z < MIDA_U8-1 && self.is_air(x,y,z+1)) || 
            (z > 0 && self.is_air(x,y,z-1))
    }

    pub fn to_vertex(&self) -> Vec<f32> {
        let mut vertices = vec![];

        for z in 0..MIDA as u8 {
            for y in 0..MIDA as u8 {
                for x in 0..MIDA as u8 {
                    if self.get(x,y,z) == 0 {
                        continue;
                    }
                    if self.touches_air(x,y,z) {
                        for i in 0..36 {
                            vertices.push(VERTICES_CUADRADO[i*3] + x as f32);
                            vertices.push(VERTICES_CUADRADO[i*3 + 1] + y as f32);
                            vertices.push(VERTICES_CUADRADO[i*3 + 2] + z as f32);
                        }
                    }
                }
            }
        }
        vertices
    }

}