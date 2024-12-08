use std::str::from_utf8;

use cgmath::Matrix;
use noise::{NoiseFn, Simplex};
use sdl2::{
    keyboard::{KeyboardState, Scancode},
    sys::{random, SDL_Delay, SDL_GL_SetAttribute, SDL_GLprofile},
};

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  layout (location = 1) in vec2 uv;
  layout (location = 2) in float light;
  out vec2 tex_coord;
  out float frag_light;
  uniform mat4 MVP;
  void main() {
    tex_coord = uv;
    frag_light = light;
    gl_Position = MVP * vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"
  #version 330 core
  out vec4 final_color;
  in vec2 tex_coord;

  in float frag_light;
  uniform sampler2D tex;

  void main() {
     final_color = texture(tex, tex_coord) * frag_light;
  }
"#;

mod camera;
use camera::Camera;
mod mundo;
use mundo::{Mundo, MIDA};

fn main() {
    let mut mundo = Mundo::new();

    let perlin: noise::Fbm<Simplex> = noise::Fbm::new(1);
    for z in 0..mundo::MIDA as u16 {
        for x in 0..mundo::MIDA as u16 {
            let val = perlin.get([0.0001 * x as f64, 0.0001 * z as f64])
                * 500.0
                * perlin.get([0.005 * x as f64, 0.005 * z as f64]);

            let altura = ((2.0 * val) - 2.0) as u16;

            let val = match altura {
                0 => 2,
                40.. => 3,
                25..40 => 6,
                1..5 => 4,
                5..20 => {
                    if unsafe { random() % 1000 } < 30
                        && x > 5
                        && x < MIDA as u16
                        && z > 5
                        && z < MIDA as u16 - 5
                    {
                        mundo.set(x, altura, z, 1);
                        mundo.set(x, altura + 1, z, 1);
                        mundo.set(x, altura + 2, z, 1);
                        for x in x - 1..=x + 1 {
                            for z in z - 1..=z + 1 {
                                for y in altura + 3..=6 + altura {
                                    mundo.set(x, y, z, 5);
                                }
                            }
                        }
                    }
                    5
                }
                _ => 1,
            };
            mundo.set(x, altura, z, val);
            for y in 0..altura {
                mundo.set(x, y, z, val);
            }
        }
    }
    mundo.set(0, 100, 0, 1);
    mundo.set(2, 100, 0, 2);
    mundo.set(0, 100, 2, 3);
    mundo.set(2, 100, 2, 4);

    let verts: Vec<f32> = mundo.to_vertex();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .build()
        .unwrap();

    unsafe {
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_RED_SIZE, 8);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_GREEN_SIZE, 8);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_BLUE_SIZE, 8);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_ALPHA_SIZE, 8);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 3);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 2);
        SDL_GL_SetAttribute(
            sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_FLAGS,
            (sdl2::sys::SDL_GLcontextFlag::SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG as u32) as i32,
        );
        SDL_GL_SetAttribute(
            sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK,
            (SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_CORE as u32) as i32,
        ); // XXX: 1 es PROFILE_CORE
           // SDL_GL_SetSwapInterval(1);
        sdl2::sys::SDL_SetRelativeMouseMode(sdl2::sys::SDL_bool::SDL_TRUE);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_DEPTH_SIZE, 24); // TODO: ???
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_DOUBLEBUFFER, 1);
    }

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.0, 0.71, 0.71, 1.0);
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE); // TODO AÑADIR
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);
    }

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

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

        gl::VertexAttribPointer(2, 1, gl::FLOAT, gl::FALSE, (4 * 6) as _, 20 as *const _); // TODO: Muy mal
        gl::EnableVertexAttribArray(2);
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
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut info_log: [u8; 2000] = [0; 2000];
            let mut length = 0;

            gl::GetShaderInfoLog(vertex_shader, 2000, &mut length, info_log.as_mut_ptr() as _);

            let slice = std::slice::from_raw_parts(info_log.as_ptr(), length as usize);
            println!(
                "Vertex shader compilation error: {}",
                from_utf8(slice).unwrap()
            );
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut info_log: [u8; 2000] = [0; 2000];
            let mut length = 0;

            gl::GetShaderInfoLog(
                fragment_shader,
                2000,
                &mut length,
                info_log.as_mut_ptr() as _,
            );

            let slice = std::slice::from_raw_parts(info_log.as_ptr(), length as usize);
            println!(
                "Fragment shader compilation error: {}",
                from_utf8(slice).unwrap()
            );
        }

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut info_log: [u8; 2000] = [0; 2000];
            let mut length = 0;

            gl::GetProgramInfoLog(
                shader_program,
                2000,
                &mut length,
                info_log.as_mut_ptr() as _,
            );

            let slice = std::slice::from_raw_parts(info_log.as_ptr(), length as usize);
            println!("Shader linking error: {}", from_utf8(slice).unwrap());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::UseProgram(shader_program);

        location = gl::GetUniformLocation(shader_program, c"MVP".as_ptr() as _);
        if location < 0 {
            panic!("ERROR: NO SE HA ENCONTRADO MVP EN EL SHADER!");
        }
    }

    let load_result = stb_image::image::load("res/atlas.png");
    let mut texture: u32 = 0;
    match load_result {
        stb_image::image::LoadResult::Error(error) => {
            panic!("Ha habido un error: {}", error);
        }
        stb_image::image::LoadResult::ImageF32(_) => {
            panic!("No soportamos este tipo de imágen");
        }
        stb_image::image::LoadResult::ImageU8(image) => unsafe {
            // glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
            // glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
            // glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
            // glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                image.width as i32,
                image.height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                image.data.as_ptr().cast(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); // TODO: ????

            gl::GenerateMipmap(gl::TEXTURE_2D);
        },
    }

    let mut camera = Camera::new(900_f32, 700_f32);
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mousestate: _,
                    x: _,
                    y: _,
                    xrel,
                    yrel,
                } => {
                    camera.rotate(xrel as f32 / -900.0, yrel as f32 / -700.0);
                }
                _ => {}
            }
        }

        let keyboard = KeyboardState::new(&event_pump);

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
            gl::BindVertexArray(vao);
            gl::UseProgram(shader_program);

            let vp = camera.build_view_projection_matrix();
            gl::UniformMatrix4fv(location, 1, gl::FALSE, vp.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, (verts.len() / 6) as _); // TODO: Muy mal
                                                                      // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                                                                      // gl::DrawElements(gl::TRIANGLES, index.len() as i32, gl::UNSIGNED_SHORT, 0 as _);
        }
        window.gl_swap_window();

        unsafe {
            SDL_Delay(5);
        }
    }
    unsafe {
        gl::DeleteProgram(shader_program);
        gl::DeleteTextures(1, &texture);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
