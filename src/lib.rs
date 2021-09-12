use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::WebGlRenderingContext as GL;
mod mat_4;
mod shapes;
mod webgl;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame'");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let height: f32 = 500.;
    let width: f32 = 500.;

    //-----Get context
    let gl = webgl::get_webgl_context(height as u32, width as u32).unwrap();

    //-----Compile and link program
    let program = webgl::link_program(
        &gl,
        include_str!("shader/vertex.vert"),
        include_str!("shader/fragment.frag"),
    )
    .unwrap();

    //Create vertex buffer object
    let att_location: [u32; 3] = [
        gl.get_attrib_location(&program, "position") as u32,
        gl.get_attrib_location(&program, "normal") as u32,
        gl.get_attrib_location(&program, "color") as u32,
    ];

    let att_stride: [i32; 3] = [3, 3, 4];
    let torus_data = shapes::torus(32, 32, 1.0, 2.0);

    let position = torus_data.0;
    let normal = torus_data.1;
    let color = torus_data.2;
    let index = torus_data.3;

    //Crate and set vbo
    let position_vbo = webgl::create_vbo_vector(&gl, &position).unwrap();
    let normal_vbo = webgl::create_vbo_vector(&gl, &normal).unwrap();
    let color_vbo = webgl::create_vbo_vector(&gl, &color).unwrap();
    webgl::set_attribute(
        &gl,
        &[position_vbo, normal_vbo, color_vbo],
        &att_location,
        &att_stride,
    );

    //Crate and set ibo
    let ibo = webgl::create_ibo_vector(&gl, &index).unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));

    //Model, view and projection transformation
    let uni_location = [
        gl.get_uniform_location(&program, "mvpMatrix").unwrap(),
        gl.get_uniform_location(&program, "invMatrix").unwrap(),
        gl.get_uniform_location(&program, "lightDirection").unwrap(),
        gl.get_uniform_location(&program, "eyeDirection").unwrap(),
        gl.get_uniform_location(&program, "ambientColor").unwrap(),
    ];

    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::CULL_FACE);
    gl.depth_func(GL::LEQUAL);

    let mut m_matrix = mat_4::Matrix::new();
    let mut v_matrix = mat_4::Matrix::new();
    let mut p_matrix = mat_4::Matrix::new();
    let mut mvp_matrix = mat_4::Matrix::new();
    let mut tmp_matrix = mat_4::Matrix::new();
    let mut inv_matrix = mat_4::Matrix::new();

    let eye_direction = [0., 0., 15.];
    v_matrix.look_at(&eye_direction, &[0., 0., 0.], &[0., 1., 0.]);
    p_matrix.perspective(width / height, 45., 0.1, 100.);
    tmp_matrix.substitution(&p_matrix).multiply(&v_matrix);

    let light_direction = [-0.5, 0.5, 0.5];
    let ambient_color = [0.1, 0.1, 0.1, 1.0];


    //call once per animation frame
    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();
    let mut i: f32 = 0.;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i >= 360. {
            i = 0.;
        }
        i += 1.;

        //Webgl initialize
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let rad = i * std::f32::consts::PI / 180.;

        //Draw by element
        m_matrix
            .set_identity()
            .rotate_around_y(rad)
            .rotate_around_z(rad);
        mvp_matrix.substitution(&tmp_matrix).multiply(&m_matrix);

        inv_matrix.substitution(&m_matrix).inverse().unwrap();

        gl.uniform_matrix4fv_with_f32_array(Some(&uni_location[0]), false, &mvp_matrix.get_value());
        gl.uniform_matrix4fv_with_f32_array(Some(&uni_location[1]), false, &inv_matrix.get_value());
        gl.uniform3fv_with_f32_array(Some(&uni_location[2]), &light_direction);
        gl.uniform3fv_with_f32_array(Some(&uni_location[3]), &eye_direction);
        gl.uniform4fv_with_f32_array(Some(&uni_location[4]), &ambient_color);
        gl.draw_elements_with_i32(GL::TRIANGLES, index.len() as i32, GL::UNSIGNED_SHORT, 0);

        //Context redrawn
        gl.flush();

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
