pub fn new_c_string(string: &str) -> std::ffi::CString {
    std::ffi::CString::new(string)
        .unwrap_or_else(|_| panic!("Error converting {} to C_String", string))
        //.expect(format!("Error converting {} to C_String", string)
}

fn convert_to_c_string(string: &str) -> *const gl::types::GLchar
{
    let c_string = std::ffi::CString::new(string).expect("Error converting String to c_string");
    //return std::ffi::CStr::from_bytes_with_nul(string.as_bytes()).unwrap().as_ptr();
    c_string.as_ptr()
}

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * std::f32::consts::PI/180f32
}