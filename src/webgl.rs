use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub fn get_webgl_context(height: u32, width: u32) -> Result<WebGlRenderingContext, String> {
    //Get WebGLContext
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or_else(|| String::from("canvas doesn't exist :("))?;
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    canvas.set_height(height);
    canvas.set_width(width);

    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")
        .unwrap()
        .ok_or_else(|| String::from("webgl is not supported in this browser :("))?
        .dyn_into()
        .unwrap();

    //Initialize WebGLContext
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
    gl.clear_depth(1.);

    Ok(gl)
}

pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();

    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        gl.use_program(Some(&program));
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unable to get shader info log")))
    }
}

#[allow(dead_code)]
pub fn create_vbo_array(gl: &GL, data: &[f32]) -> Result<WebGlBuffer, String> {
    let vbo = gl.create_buffer().ok_or("Failed to create buffer :(")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
    unsafe {
        let f32_array = js_sys::Float32Array::view(data);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::STATIC_DRAW)
    }
    gl.bind_buffer(GL::ARRAY_BUFFER, None);

    Ok(vbo)
}

pub fn create_vbo_vector(gl: &GL, data: &Vec<f32>) -> Result<WebGlBuffer, String> {
    let vbo = gl.create_buffer().ok_or("Failed to create buffer :(")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
    unsafe {
        let f32_array = js_sys::Float32Array::view(&(*data));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &f32_array, GL::STATIC_DRAW)
    }
    gl.bind_buffer(GL::ARRAY_BUFFER, None);

    Ok(vbo)
}

#[allow(dead_code)]
pub fn create_ibo_array(gl: &GL, data: &[u16]) -> Result<WebGlBuffer, String> {
    let ibo = gl.create_buffer().ok_or("Failed to create buffer :(")?;

    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));
    unsafe {
        let ui16_array = js_sys::Uint16Array::view(data);
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &ui16_array,
            GL::STATIC_DRAW,
        );
    }
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);

    Ok(ibo)
}

pub fn create_ibo_vector(gl: &GL, data: &Vec<u16>) -> Result<WebGlBuffer, String> {
    let ibo = gl.create_buffer().ok_or("Failed to create buffer")?;

    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));
    unsafe {
        let ui16_array = js_sys::Uint16Array::view(&(*data));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &ui16_array,
            GL::STATIC_DRAW,
        );
    }
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);

    Ok(ibo)
}

pub fn set_attribute(gl: &GL, vbo: &[WebGlBuffer], att_location: &[u32], att_stride: &[i32]) {
    for i in 0..vbo.len() {
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo[i]));
        gl.enable_vertex_attrib_array(att_location[i]);
        gl.vertex_attrib_pointer_with_i32(att_location[i], att_stride[i], GL::FLOAT, false, 0, 0);
    }
}

