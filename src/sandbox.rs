pub fn test_matrix_calculations() {

    
    /* let mut trans = nalgebra_glm::make_mat4(&[1.0 as f32; 16]);
    trans = nalgebra_glm::rotate(&trans, 1.5708 as f32, &nalgebra_glm::vec3(0.0, 0.0, 1.0));
    trans = nalgebra_glm::scale(&trans, &nalgebra_glm::vec3(0.5, 0.5, 0.5)); */

    let test_mat4: nalgebra_glm::Mat4 = nalgebra_glm::diagonal4x4( &nalgebra_glm::make_vec4( &[1.0 as f32; 4] ) );

    let test_mat4_2 = nalgebra_glm::diagonal4x4( &nalgebra_glm::vec4(1.0 as f32, 1.0 as f32, 1.0 as f32, 1.0 as f32) );

    let rot = nalgebra_glm::rotation(90_f32, &nalgebra_glm::vec3(0.0 as f32, 0.0 as f32, 1.0 as f32));
    let scale = nalgebra_glm::scale(&rot, &nalgebra_glm::vec3(0.5 as f32, 0.5 as f32, 0.5 as f32));

    println!("{}", test_mat4);
    println!("{}", test_mat4_2);
    println!("Rotatio mat: {}", rot);
    println!("Scale: {}", scale);

    let mut trans = nalgebra_glm::make_mat4(&[1.0 as f32; 16]);

    println!("{}", trans);

    trans = nalgebra_glm::rotate(&trans, 90_f32, &nalgebra_glm::vec3(0.0, 0.0, 1.0));

    println!("{}", trans);

    // printMatrix(nalgebra_glm::value_ptr(&trans), 4);
}


fn print_matrix(matrix: &[f32], break_number: u32) {

    let mut count = 0;
    for i in matrix {
        if count == break_number {
            count = 0;
            println!();
        }        
        print!("{} ", i);

        count+=1;
    }

}