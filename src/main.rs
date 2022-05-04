mod compute;
mod graphics;

pub fn main() {
    let shader_descriptor = wgpu::include_spirv!(env!("shaders.spv"));

    if true {
        // triangle example
        graphics::start(shader_descriptor);
    } else {
        // compute shader example
        compute::start(shader_descriptor);
    }
}
