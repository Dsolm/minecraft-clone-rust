pub struct Mundo {
    bloques: Vec<u8>
}

pub const MIDA: usize = 64;

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

    pub fn to_vertex(&self) -> Vec<f32> {
        let mut vertices = vec![];

        for z in 0..MIDA as u8 {
            for y in 0..MIDA as u8 {
                for x in 0..MIDA as u8 {
                    if self.get(x,y,z) == 0 {
                        continue;
                    }
                    
                    for i in 0..36 {
                        vertices.push(VERTICES_CUADRADO[i*3] + x as f32);
                        vertices.push(VERTICES_CUADRADO[i*3 + 1] + y as f32);
                        vertices.push(VERTICES_CUADRADO[i*3 + 2] + z as f32);
                    }
                }
            }
        }

        vertices
    }

}