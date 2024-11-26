use std::io::{self, BufRead};

use cgmath::Matrix;
use sdl2::sys::SDL_GL_SetAttribute;

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  out vec3 fragment_pos;
  uniform mat4 MVP;
  void main() {
    fragment_pos = pos;
    gl_Position = MVP * vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;
  in vec3 fragment_pos;
  void main() {
    final_color = vec4(fragment_pos.x, fragment_pos.y, 0.8, 1.0);
  }
"#;

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Camera {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 6.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: window_width / window_height,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return proj * view;
    }
}

type Vertex = [f32; 3];
const VERTICES: [Vertex; 6] = [
    [-0.5, -0.5, 0.0],
    [0.5, -0.5, 0.0],
    [0.0, 0.5, 0.0],
    [0.5, 0.5, 0.0],
    [0.5, -0.5, 0.0],
    [0.0, 0.5, 0.0],
];

// Returns: (vertices, indices)
fn read_obj_model(file: &str) -> (Vec<Vertex>, Vec<u16>) {
    let file = std::fs::File::open(file).expect("File could not be open. Closing program.");
    let lines = io::BufReader::new(file).lines();

    let mut vertices : Vec<Vertex> = Vec::new();
    let mut indices : Vec<u16> = Vec::new();

    for line in lines {
        let line = line.expect("Error while reading file.");
        if !line.is_empty() {
            let mut words = line.split_whitespace();
            match words.next() {
                Some("v") => {
                    vertices.push([
                        words.next().unwrap().parse().unwrap(),
                        words.next().unwrap().parse().unwrap(),
                        words.next().unwrap().parse().unwrap()
                    ]);
                },
                Some("f") => {
                    indices.push(words.next().unwrap().parse().unwrap());
                    indices.push(words.next().unwrap().parse().unwrap());
                    indices.push(words.next().unwrap().parse().unwrap());
                },
                _ => {
                    panic!("Invalid file");
                }

            }
        }
    }

    (vertices, indices)
}

fn main() {
    let (vertices, indices) = read_obj_model("teapot.obj");
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    unsafe {
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK, 1); // XXX: 1 es PROFILE_CORE
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 3);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 3);
    }

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        // gl::DepthFunc(gl::LESS);  
    }

    unsafe {
        let mut VAO = 0;
        gl::GenVertexArrays(1, &mut VAO);
        gl::BindVertexArray(VAO);
    }

    unsafe {
        let mut VBO = 0;
        gl::GenBuffers(1, &mut VBO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<Vertex>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
    }

    let shader_program: u32;
    let uniform_mvp: i32;
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::UseProgram(shader_program);
        uniform_mvp = gl::GetUniformLocation(shader_program, "MVP\0".as_bytes().as_ptr().cast());
        if uniform_mvp < 0 {
            println!("EL SHADER NO TIENE LA VARIABLE UNIFORME MVP");
        }
    }

    let camera = Camera::new(900 as f32, 700 as f32);
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(shader_program);
            gl::UniformMatrix4fv(uniform_mvp, 1, gl::FALSE, camera.build_view_projection_matrix().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
        }

        window.gl_swap_window();
    }
}
