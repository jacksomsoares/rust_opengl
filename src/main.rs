#![allow(dead_code, unused_variables)]

use sdl2::init;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use gl::types::*;

use std::mem;
use std::ptr;

mod shader;
mod utils;
mod sandbox;
mod vertex_shapes;
mod camera;

fn print_gl_version_and_profile(gl_attributes: &sdl2::video::gl_attr::GLAttr)
{
    use sdl2::video::GLProfile;
    let major_version = gl_attributes.context_major_version();
    let minor_version = gl_attributes.context_minor_version();

    let ctx_profile = gl_attributes.context_profile();

    println!("Major: {}\nMinor: {}", major_version, minor_version);

    match ctx_profile {
        GLProfile::Core => {
            println!("Profile is core");
        },
        GLProfile::Compatibility => println!("Profile is compatibility"),
        GLProfile::GLES => println!("Profile is GLES (Mobile/web)"),
        _ => println!("Profile is unknow")
    }
}

fn set_gl_version_and_profile(gl_attributes: &sdl2::video::gl_attr::GLAttr)
{
    use sdl2::video::GLProfile;

    gl_attributes.set_context_major_version(4);
    gl_attributes.set_context_minor_version(5);

    gl_attributes.set_context_profile(GLProfile::Core);
}

fn print_sdl_version_info()
{
    let version = sdl2::version::version();
    println!("Sdl version: {}.{}.{}", version.major, version.minor, version.patch);
}

fn framebuffer_size_callback(width: GLsizei, height: GLsizei)
{
    // make sure the viewport matches the new window dimensions; note that width and 
    // height will be significantly larger than specified on retina displays.
    unsafe { gl::Viewport(0, 0, width, height); }
}

fn mouse_callback(camera: &mut camera::Camera, xpos: i32, ypos: i32, last_x: &mut i32, last_y: &mut i32, yaw: &mut f32, pitch: &mut f32)
{
    let mut xoffset: f32 = (xpos - *last_x) as f32;
    let mut yoffset: f32 = (*last_y - ypos) as f32; // reversed since y-coordinates range from bottom to top

    *last_x = xpos;
    *last_y = ypos;

    camera.process_mouse_movement(&mut xoffset, &mut yoffset, true);
}

fn scroll_callback(camera: &mut camera::Camera ,zoom: &mut f32, xoffset: i32, yoffset: i32) {
    *zoom -= yoffset as f32;
    if *zoom < 1.0f32 {
        *zoom = 1.0f32;
    }
    if *zoom > 45.0f32 {
        *zoom = 45.0f32;
    }

    camera.process_mouse_scroll(yoffset as f32);
}

fn main()
{
    //.unwrap().to_str().unwrap();
    //unwrap().to_str().unwrap();

    let current_dir_path = std::env::current_dir().unwrap();
    let current_dir = current_dir_path.to_str().unwrap();
    println!("Current dir: {}", current_dir);

    let width: u32 = 800;
    let height: u32 = 600;

    print_sdl_version_info();
    let sdl_context = init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    //video_subsystem.gl_load_library_default().unwrap();
    let gl_attributes = video_subsystem.gl_attr();

    // print_gl_version_and_profile(&gl_attributes);
    set_gl_version_and_profile(&gl_attributes);
    // print_gl_version_and_profile(&gl_attributes);

    for i in sdl2::video::drivers() 
    {
        println!("Video driver: {}",i);
    }

    println!("Current video driver: {}", video_subsystem.current_video_driver());

    let window = video_subsystem.window("title: title", width, height)
    .position_centered()
    .opengl()
    .resizable()
    .build()
    .unwrap();

    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().capture(true);
    sdl_context.mouse().set_relative_mouse_mode(true);

    let gl_context = window.gl_create_context().unwrap();

    // video_subsystem.gl_set_swap_interval(0).unwrap();

    window.gl_make_current(&gl_context).unwrap();

    // Load OpenGL Function Pointers
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);

    unsafe { gl::Enable(gl::DEPTH_TEST); }

    //let mut canvas = window.into_canvas().build().unwrap();
    //canvas.set_draw_color(Color::RGB(0, 255, 255));
    //canvas.clear();
    //canvas.present();
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut shader = shader::Shader::new();
    shader.create_program(current_dir_path.join("assets\\3.3.shader.vs").to_str().unwrap() , current_dir_path.join("assets\\3.3.shader.fs").to_str().unwrap() );

    /* let vertices = [
        // positions         // colors
         0.5 as f32, -0.5, 0.0,  1.0, 0.0, 0.0,  // bottom right
        -0.5, -0.5, 0.0,         0.0, 1.0, 0.0,  // bottom left
         0.0,  0.5, 0.0,         0.0, 0.0, 1.0   // top 
    ]; */

    /* let vertices = [
        // positions              // colors         // texture coords
         0.5 as f32,  0.5, 0.0,   1.0, 0.0, 0.0,    1.0, 1.0, // top right
         0.5,        -0.5, 0.0,   0.0, 1.0, 0.0,    1.0, 0.0, // bottom right
        -0.5,        -0.5, 0.0,   0.0, 0.0, 1.0,    0.0, 0.0, // bottom left
        -0.5,         0.5, 0.0,   1.0, 1.0, 0.0,    0.0, 1.0  // top left 
    ]; */

    let vertices = vertex_shapes::get_cube();

    let cube_positions = [
        nalgebra_glm::vec3( 0.0f32,  0.0,  0.0),
        nalgebra_glm::vec3( 2.0f32,  5.0, -15.0),
        nalgebra_glm::vec3(-1.5f32, -2.2, -2.5),
        nalgebra_glm::vec3(-3.8f32, -2.0, -12.3),
        nalgebra_glm::vec3( 2.4f32, -0.4, -3.5),
        nalgebra_glm::vec3(-1.7f32,  3.0, -7.5),
        nalgebra_glm::vec3( 1.3f32, -2.0, -2.5),
        nalgebra_glm::vec3( 1.5f32,  2.0, -2.5),
        nalgebra_glm::vec3( 1.5f32,  0.2, -1.5),
        nalgebra_glm::vec3(-1.3f32,  1.0, -1.5)
    ];

    let indices = [
        0 as u32, 1, 3, // first triangle
        1,        2, 3  // second triangle
    ];

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    /* let mut EBO: u32 = 0; */

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        /* gl::GenBuffers(1, &mut EBO); */

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);
    
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(&vertices) as isize, vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

        /* gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, mem::size_of_val(&indices) as isize, indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW); */
    
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<f32>() as i32, ptr::null());
        gl::EnableVertexAttribArray(0);
        // color attribute
        // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * mem::size_of::<f32>() as i32, (3 * mem::size_of::<f32>() as i32) as *const std::ffi::c_void);
        // gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<f32>() as i32, (3 * mem::size_of::<f32>()) as *const std::ffi::c_void);
        gl::EnableVertexAttribArray(1);
    
        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        // gl::BindVertexArray(0);
    }

    // Load and create Texture
    let mut texture1: u32 = 0;
    let mut texture2: u32 = 0;
    {
        unsafe {
            gl::GenTextures(1, &mut texture1);
            gl::BindTexture(gl::TEXTURE_2D, texture1); 
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	// set texture wrapping to GL_REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        // load image, create texture and generate mipmaps
        let data = image::open( current_dir_path.join("assets\\container.jpg") ).expect("Failed to load image");

        let data_raw = data.into_rgb();

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                data_raw.width() as i32,
                data_raw.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data_raw.into_raw().as_ptr() as *const std::ffi::c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        // texture 2
        // ---------
        unsafe {
            gl::GenTextures(1, &mut texture2);
            gl::BindTexture(gl::TEXTURE_2D, texture2);
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        // load image, create texture and generate mipmaps
        let data_t2 = image::open( current_dir_path.join("assets\\awesomeface.png") ).expect("Failed to load image");
        let data_raw_t2 = data_t2.rotate180().into_rgba();

        // note that the awesomeface.png has transparency and thus an alpha channel, so make sure to tell OpenGL the data type is of GL_RGBA
        unsafe {
            gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    data_raw_t2.width() as i32,
                    data_raw_t2.height() as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    data_raw_t2.into_raw().as_ptr() as *const std::ffi::c_void
                );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    shader.use_shader();
    shader.set_int("texture1", 0);
    shader.set_int("texture2", 1);

    let mut camera = camera::Camera::new();
    camera.position = nalgebra_glm::vec3(0.0f32, 0.0, 3.0);

    let mut mouse_last_x: i32 = 400;
    let mut mouse_last_y: i32 = 300;
    let mut yaw = -90.0f32;
    let mut pitch = 0f32;
    let mut fov = 45.0f32;

    let timer = std::time::SystemTime::now();

    // let mut camera_pos   = nalgebra_glm::vec3(0.0f32, 0.0,  3.0);
    // let mut camera_front = nalgebra_glm::vec3(0.0f32, 0.0, -1.0);
    let camera_up = nalgebra_glm::vec3(0.0f32, 1.0,  0.0);

    let mut last_frame = 0.0f32; // Time of last frame
    let mut first_mouse = true;
    
    'running: loop {
        let dur = std::time::Duration::from_secs(1);
        let current_frame = timer.elapsed().expect("Time elapsed failed").as_secs_f32();
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;

        for event in event_pump.poll_iter() {
            let camera_speed = 3.5f32 * delta_time;

            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    // camera_pos += camera_speed * camera_front;
                    camera.process_keyboard(camera::CameraMovement::FORWARD, delta_time);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    // camera_pos -= camera_speed * camera_front;
                    camera.process_keyboard(camera::CameraMovement::BACKWARD, delta_time);
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    // camera_pos -= nalgebra_glm::normalize(&nalgebra_glm::cross(&camera_front, &camera_up)) * camera_speed;
                    camera.process_keyboard(camera::CameraMovement::LEFT, delta_time);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    // camera_pos += nalgebra_glm::normalize(&nalgebra_glm::cross(&camera_front, &camera_up)) * camera_speed;
                    camera.process_keyboard(camera::CameraMovement::RIGHT, delta_time);
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    // camera_pos += camera_up * camera_speed;
                    camera.process_keyboard(camera::CameraMovement::UP, delta_time);
                },
                Event::KeyDown { keycode: Some(Keycode::LShift), .. } | 
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    // camera_pos += (camera_up * -1.0f32) * camera_speed;
                    camera.process_keyboard(camera::CameraMovement::DOWN, delta_time);
                },
                Event::MouseMotion { x, y, ..} => {
                    if first_mouse
                    {
                        mouse_last_x = x;
                        mouse_last_y = y;
                        first_mouse = false;
                    }
                    mouse_callback(&mut camera, x, y, &mut mouse_last_x, &mut mouse_last_y, &mut yaw, &mut pitch);
                },
                Event::MouseWheel { x, y, ..} => {
                    scroll_callback(&mut camera, &mut fov, x, y);
                },
                _ => {}
            }
        }

        if gl_context.is_current()
        {
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // bind textures on corresponding texture units
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture1);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, texture2);
            }

            shader.use_shader();

            let projection = nalgebra_glm::perspective(utils::degree_to_radian( camera.zoom ), width as f32 / height as f32, 0.1f32, 100.0f32);
            shader.set_mat4("projection", &projection);

            

            let mut direction = nalgebra_glm::vec3(0.0f32, 0.0, 0.0);
            direction.x = f32::cos(utils::degree_to_radian(yaw) * f32::cos(utils::degree_to_radian(pitch)) );
            direction.y = f32::sin(utils::degree_to_radian(pitch));
            direction.z = f32::sin(utils::degree_to_radian(yaw)) * f32::cos(utils::degree_to_radian(pitch));
            // camera_front = nalgebra_glm::normalize(&direction);

            let view = camera.get_view_matrix();
            // let view = nalgebra_glm::look_at(&camera_pos, &(camera_pos + camera_front), &camera_up);
            shader.set_mat4("view", &view);

            unsafe { gl::BindVertexArray(vao); }
            for ( i, cube ) in cube_positions.iter().enumerate() {
                let mut model = nalgebra_glm::translation(&cube);
                
                let angle = 20.0f32 * (i + 1) as f32;
                let seconds = timer.elapsed().expect("Time elapsed failed").as_secs_f32() * utils::degree_to_radian(angle);
                model = nalgebra_glm::rotate(&model, seconds, &nalgebra_glm::vec3(1.0f32, 0.3, 0.5));
                shader.set_mat4("model", &model);

                unsafe {
                    // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                    gl::DrawArrays(gl::TRIANGLES, 0, 36);
                }
            }
        }

        window.gl_swap_window();
        let dur = std::time::Duration::from_millis(1000);
        // std::thread::sleep(dur);
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        /* gl::DeleteBuffers(1, &EBO); */
    }
}