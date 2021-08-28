use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
mod mat_4;
mod webgl;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame'");
}

#[wasm_bindgen(start)]
#[allow(unused_assignments)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    //-----Get context
    let document = window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let height: f32 = 500.;
    let width: f32 = 500.;

    canvas.set_height(height as u32);
    canvas.set_width(width as u32);

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    //-----Compile and link program
    let program = webgl::link_program(
        &gl,
        include_str!("shader/vertex.vert"),
        include_str!("shader/fragment.frag"),
    )
    .unwrap();

    //-----Create vertex buffer object
    let buffer_manager = webgl::BufferObject::new();
    let mut att_location: [u32; 3] = [0; 3];
    att_location[0] = gl.get_attrib_location(&program, "position") as u32;
    att_location[1] = gl.get_attrib_location(&program, "normal") as u32;
    att_location[2] = gl.get_attrib_location(&program, "color") as u32;

    let att_stride: [i32; 3] = [3, 3, 4];

    let position = [
        0., 1., 0., 1., 0., 0., -1., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0., -1.,
    ];

    let normal = [
        0., 1., 0., 1., 0., 0., -1., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0., -1.,
    ];

    let color = [
        1., 0., 0., 1., 0., 1., 0., 1., 0., 0., 1., 1., 1., 1., 0., 1., 0., 1., 1., 1., 1., 1., 1.,
        1.,
    ];

    let index = [
        0, 2, 4, 0, 4, 1, 0, 1, 5, 0, 5, 2, 3, 1, 4, 3, 2, 5, 3, 5, 1, 3, 4, 2,
    ];

    //crate and set vbo
    let position_vbo = buffer_manager.create_vbo(&gl, &position);
    let normal_vbo = buffer_manager.create_vbo(&gl, &normal);
    let color_vbo = buffer_manager.create_vbo(&gl, &color);
    buffer_manager.set_attribute(
        &gl,
        &[position_vbo, normal_vbo, color_vbo],
        &att_location,
        &att_stride,
    );

    //crate and set ibo
    let ibo = buffer_manager.create_ibo(&gl, &index);
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));

    //Model, view and projection transformation
    let uni_location: [WebGlUniformLocation; 5] = [
        gl.get_uniform_location(&program, "mvpMatrix").unwrap(),
        gl.get_uniform_location(&program, "invMatrix").unwrap(),
        gl.get_uniform_location(&program, "lightDirection").unwrap(),
        gl.get_uniform_location(&program, "eyeDirection").unwrap(),
        gl.get_uniform_location(&program, "ambientColor").unwrap(),
    ];

    //WebGL setting
    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::CULL_FACE);
    gl.depth_func(GL::LEQUAL);

    //matrix initialize
    let mut m_matrix = mat_4::Matrix::identity();
    let mut v_matrix = mat_4::Matrix::identity();
    let mut p_matrix = mat_4::Matrix::identity();
    let mut mvp_matrix = mat_4::Matrix::identity();
    let mut tmp_matrix = mat_4::Matrix::identity();
    let mut inv_matrix = mat_4::Matrix::identity();

    //view and projection transformation matrix
    v_matrix.look_at(&[0., 6., 6.], &[0., 0., 0.], &[0., 1., 0.]);
    p_matrix.perspective(width / height, 45., 0.1, 100.);
    tmp_matrix.substitution(&p_matrix).multiply(&v_matrix);

    let light_direction: [f32; 3] = [0., 1., 0.];
    let ambient_color: [f32; 4] = [0.1, 0.1, 0.1, 1.];
    let eye_direction: [f32; 3] = [0., 6., 6.,];

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut i: f32 = 0.;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i >= 360. {
            i = 0.;
        }
        i += 1.;
        let rad = i * std::f32::consts::PI / 180.;

        //-----Webgl initialize
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

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


        //draw model
        gl.draw_elements_with_i32(GL::TRIANGLES, index.len() as i32, GL::UNSIGNED_SHORT, 0);

        //Context redrawn
        gl.flush();

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
