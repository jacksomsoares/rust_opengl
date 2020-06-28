use gl::types::*;

pub struct Shader
{
    // the program ID
    pub id: GLuint,
}

/* fn convert_to_c_string(string: &str) -> *const gl::types::GLchar
{
    return std::ffi::CString::new(string).expect("Error converting String to c_string").as_ptr();
    //return std::ffi::CStr::from_bytes_with_nul(string.as_bytes()).unwrap().as_ptr();
} */

impl Shader
{
    pub fn new() -> Shader
    {
        Shader {
            id: 0,
        }
    }

    // use/activate the shader
    pub fn use_shader(&self)
    {
        unsafe {gl::UseProgram(self.id);}
    }

    // constructor reads and builds the shader
    pub fn create_program(&mut self, vertex_path: &str, fragment_path: &str)
    {
        println!("Loading vertex shader in path: {}", vertex_path);
        println!("Loading fragment shader in path: {}", fragment_path);

        use std::fs::File;
        use std::io::prelude::*;
        use std::ptr;

        // 1. retrieve the vertex/fragment source code from filePath
        let mut v_shader_file = File::open(vertex_path).expect("ERROR::SHADER::FILE_NOT_SUCCESFULLY_READ");
        let mut f_shader_file = File::open(fragment_path).expect("ERROR::SHADER::FILE_NOT_SUCCESFULLY_READ");

        let mut vertex_code = String::new();
        let mut fragment_code = String::new();

        v_shader_file.read_to_string(&mut vertex_code).unwrap();
        f_shader_file.read_to_string(&mut fragment_code).unwrap();

        let v_shader_code = std::ffi::CString::new(vertex_code).unwrap();
        let f_shader_code = std::ffi::CString::new(fragment_code).unwrap();

        // 2. compile shaders
        let vertex: u32;
        let fragment: u32;

        // vertex shader
        unsafe {
            vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
        }
        self.check_compile_errors(vertex, "VERTEX");

        // fragment Shader
        unsafe {
            fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
        }
        self.check_compile_errors(fragment, "FRAGMENT");

        // shader Program
        unsafe {
            self.id = gl::CreateProgram();
            gl::AttachShader(self.id, vertex);
            gl::AttachShader(self.id, fragment);
            gl::LinkProgram(self.id);
        }
        self.check_compile_errors(self.id, "PROGRAM");
        
        // delete the shaders as they're linked into our program now and no longer necessary
        unsafe {
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }

    }

    // utility uniform functions
    pub fn set_bool(&self, name: &str, value: bool)
    {
        let c_string = super::utils::new_c_string(name);
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation( self.id, c_string.as_ptr() ),
                value as i32
            );
        }
    }

    pub fn set_int(&self, name: &str, value: GLint)
    {
        unsafe {
            let location = gl::GetUniformLocation( self.id, super::utils::new_c_string(name).as_ptr() );
            if location == -1 {
                println!("Error on get Uniform Location");
            }
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32)
    {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation( self.id, super::utils::new_c_string(name).as_ptr() ),
                value
            );
        }
    }

    pub fn set_mat4(&self, name: &str, value: &nalgebra_glm::TMat4<f32>)
    {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation( self.id, super::utils::new_c_string(name).as_ptr() ),
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(value).as_ptr()
            );
        }
    }

    fn check_compile_errors(&self, shader: u32, type_: &str)
    {
        use std::ffi::CStr;
        use std::ptr;

        let mut success: i32 = 0;
        let mut info_log: [gl::types::GLchar; 1024] = [0; 1024];

        if type_ != "PROGRAM"
        {
            unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success); }
            if success == 0
            {
                unsafe { gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr()); }
                let string_msg: &str;
                unsafe { string_msg = CStr::from_ptr(info_log.as_ptr()).to_str().unwrap(); }
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n -- --------------------------------------------------- -- ", type_, string_msg);
            }
        }
        else {
            unsafe { gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success); }
            if success == 0
            {
                unsafe { gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr()); }
                let string_msg: &str;
                unsafe { string_msg = CStr::from_ptr(info_log.as_ptr()).to_str().unwrap(); }

                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n -- --------------------------------------------------- -- ", type_, string_msg);
            }
        }
    }
}