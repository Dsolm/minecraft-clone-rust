use cgmath::Vector3;


pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    front: cgmath::Vector3<f32>,
    right: cgmath::Vector3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    angulo_horizontal: f32,
    angulo_vertical: f32,
}

pub enum Direction {
    Left,
    Right,
    Front,
    Back,
}

impl Camera {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Camera {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 0.0, 30.0).into(),
            // have it look at the origin
            angulo_horizontal: 180.0_f32.to_radians(),
            angulo_vertical: 0.0,
            front: (0.0, 0.0, 1.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: window_width / window_height,
            fovy: 45.0,
            znear: 0.1,
            zfar: 1000.0,
            right: (1.0, 0.0, 0.0).into(),
        }
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        self.angulo_horizontal += dx;
        self.angulo_vertical += dy;
        self.front = Vector3::new(
            self.angulo_horizontal.sin() * self.angulo_vertical.cos(),
            self.angulo_vertical.sin(),
            self.angulo_horizontal.cos() * self.angulo_vertical.cos(),
        );
        self.right = self.front.cross(self.up);
    }

    const SPEED_SCALE: f32 = 1.0;
    pub fn mover(&mut self, direction: Direction) {
        match direction {
            Direction::Front => {
                self.eye += self.front * Self::SPEED_SCALE;
            }
            Direction::Back => {
                self.eye -= self.front * Self::SPEED_SCALE;
            }
            Direction::Right => {
                self.eye += self.right * Self::SPEED_SCALE;
            }
            Direction::Left => {
                self.eye -= self.right * Self::SPEED_SCALE;
            }
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.eye + self.front, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        proj * view
    }

    pub fn get_bloque_apuntado(&self) -> (usize, usize, usize) {
        let posicion = self.eye + self.front;
        (
            posicion.x as usize,
            posicion.y as usize,
            posicion.z as usize,
        )
    }
}
