use cgmath::Matrix;
use noise::{NoiseFn, Perlin};
use sdl2::{keyboard::{KeyboardState, Scancode}, sys::{SDL_GL_SetAttribute, SDL_GL_SetSwapInterval}};

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  out vec3 fragment_pos;
  uniform mat4 MVP;
  void main() {
    fragment_pos = pos;
    gl_Position = MVP * vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"
  #version 330 core
  out vec4 final_color;
  in vec3 fragment_pos;
  void main() {
     vec3 color = abs(cos(fragment_pos));
     final_color = vec4(color,1.0);
  }
"#;

mod camera;
use camera::Camera;
mod mundo;
use mundo::Mundo;

fn main() {
    let mut mundo = Mundo::new();

    let perlin = Perlin::new(1);

    for z in 0..mundo::MIDA as u8 {
        for x in 0..mundo::MIDA as u8 {
            let val = perlin.get([0.005*x as f64,0.005*z as f64]) * 100.0 + perlin.get([0.05*x as f64,0.05*z as f64]) * 10.0;
            let altura = val as u8;
            mundo.set(x,altura,z,1);
            for y in 0..altura {
                mundo.set(x,y,z,1);
            }
        }
    }

    let verts: Vec<f32> = mundo.to_vertex();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut window_builder = video_subsystem
        .window("Game", 900, 700);
    let flags = window_builder.opengl().resizable().window_flags();
    let window = window_builder.build().unwrap();

    unsafe {
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK, 1); // XXX: 1 es PROFILE_CORE
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 3);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 3);
        SDL_GL_SetSwapInterval(0);
        sdl2::sys::SDL_SetRelativeMouseMode(sdl2::sys::SDL_bool::SDL_TRUE);
    }

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        gl::Enable(gl::DEPTH_TEST);
    }

    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (verts.len() * 4) as isize,
            verts.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (4 * 3) as _,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // gl::GenBuffers(1, &mut ebo);
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        // gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (index.len() * 2) as isize, index.as_ptr().cast(), gl::STATIC_DRAW);

    }

    let shader_program: u32;
    let location;
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

        location = gl::GetUniformLocation(shader_program, "MVP\0".as_ptr() as _);
        if location < 0 {
            panic!("ERROR: NO SE HA ENCONTRADO MVP EN EL SHADER!");
        }
    }

    let mut camera = Camera::new(900_f32, 700_f32);
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                    camera.rotate(xrel as f32 / -900.0, yrel as f32 / -700.0);
                },
                _ => {}
            }
        }

        let keyboard = KeyboardState::new(&event_pump);

        // let start = Instant::now();
        if keyboard.is_scancode_pressed(Scancode::T) {
            camera.rotate(0.01, 0.0);
        }
        if keyboard.is_scancode_pressed(Scancode::G) {
            camera.rotate(-0.01, 0.0);
        }

        if keyboard.is_scancode_pressed(Scancode::E) {
            camera.eye.y += 0.1;
        }
        if keyboard.is_scancode_pressed(Scancode::Q) {
            camera.eye.y -= 0.1;
        }

        if keyboard.is_scancode_pressed(Scancode::W) {
            camera.mover(camera::Direction::Front);
        }
        if keyboard.is_scancode_pressed(Scancode::S) {
            camera.mover(camera::Direction::Back);
        }

        if keyboard.is_scancode_pressed(Scancode::A) {
            camera.mover(camera::Direction::Left);
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            camera.mover(camera::Direction::Right);
        }


        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(shader_program);

            let vp = camera.build_view_projection_matrix();
            gl::UniformMatrix4fv(location, 1, gl::FALSE, vp.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, (verts.len() / 3) as _);
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            // gl::DrawElements(gl::TRIANGLES, index.len() as i32, gl::UNSIGNED_SHORT, 0 as _);
        }
        window.gl_swap_window();

        // let duration = start.elapsed();
        // println!("El frame ha durado {}ms", duration.as_millis());

    }
}
