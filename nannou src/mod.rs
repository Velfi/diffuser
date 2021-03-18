mod physarum;

use physarum::physarum;

use crate::{
    constants::{
        DEFAULT_AGENT_COUNT, DEFAULT_AGENT_DEP_T, DEFAULT_AGENT_FL, DEFAULT_AGENT_FR,
        DEFAULT_AGENT_RA, DEFAULT_AGENT_SO, DEFAULT_AGENT_SS, DEFAULT_DECAY_FACTOR,
        DEFAULT_ITERATION_COUNT, DEFAULT_RESOLUTION_H, DEFAULT_RESOLUTION_W,
    },
    matrix::Matrix2D,
};
use nannou::prelude::*;

pub struct Model {
    pub _window: window::Id,
    pub decay_factor: f32,
    pub mouse_xy: Vector2<f32>,
    pub rng: rand::rngs::ThreadRng,
    pub window_rect: Rect<f32>,
    pub base_matrix: Matrix2D,
    pub modifier_matrix: Matrix2D,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [-0.5, -0.25],
    },
    Vertex {
        position: [0.0, 0.5],
    },
    Vertex {
        position: [0.25, -0.1],
    },
];

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

impl Model {
    pub fn new(app: &nannou::App) -> Self {
        let window_rect = Rect::from_w_h(DEFAULT_RESOLUTION_W as f32, DEFAULT_RESOLUTION_W as f32);
        let _window = app
            .new_window()
            .size(DEFAULT_RESOLUTION_W, DEFAULT_RESOLUTION_H)
            .view(view)
            .mouse_moved(update::mouse_moved)
            .mouse_pressed(update::mouse_pressed)
            .mouse_released(update::mouse_released)
            // .key_pressed(update::key_pressed)
            // .resized(update::resized)
            .build()
            .unwrap();

        let device = window.swap_chain_device();
        let format = Frame::TEXTURE_FORMAT;
        let sample_count = window.msaa_samples();

        // Load shader modules.
        let vs_mod = wgpu::shader_from_spirv_bytes(device, include_bytes!("shaders/vert.spv"));
        let fs_mod = wgpu::shader_from_spirv_bytes(device, include_bytes!("shaders/frag.spv"));

        // Create the vertex buffer.
        let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
        let usage = wgpu::BufferUsage::VERTEX;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices_bytes,
            usage,
        });

        // Create the render pipeline.
        let bind_group_layout = wgpu::BindGroupLayoutBuilder::new().build(device);
        let bind_group = wgpu::BindGroupBuilder::new().build(device, &bind_group_layout);
        let pipeline_layout =
            wgpu::create_pipeline_layout(device, None, &[&bind_group_layout], &[]);
        let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
            .fragment_shader(&fs_mod)
            .color_format(format)
            .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float2])
            .sample_count(sample_count)
            .build(device);

        let rng = rand::thread_rng();
        let base_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);
        let modifier_matrix = Matrix2D::new(window_rect.h() as usize, window_rect.w() as usize);

        let model = Self {
            _window,
            base_matrix,
            bind_group,
            decay_factor: DEFAULT_DECAY_FACTOR,
            modifier_matrix,
            mouse_xy: Vector2::new(0.0, 0.0),
            render_pipeline,
            rng,
            vertex_buffer,
            window_rect,
        };

        model
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    let height = model.window_rect.h();

    model.base_matrix.iter_mut().for_each(|(x, y, value)| {
        let (x, y) = (x as f32, y as f32);
    });
}

pub fn view(app: &App, _model: &Model, frame: Frame) {
    // Using this we will encode commands that will be submitted to the GPU.
    let mut encoder = frame.command_encoder();

    // The render pass can be thought of a single large command consisting of sub commands. Here we
    // begin a render pass that outputs to the frame's texture. Then we add sub-commands for
    // setting the bind group, render pipeline, vertex buffers and then finally drawing.
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(frame.texture_view(), |color| color)
        .begin(&mut encoder);
    render_pass.set_bind_group(0, &model.bind_group, &[]);
    render_pass.set_pipeline(&model.render_pipeline);
    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));

    // We want to draw the whole range of vertices, and we're only drawing one instance of them.
    let vertex_range = 0..VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);

    // Now we're done! The commands we added will be submitted after `view` completes.

    // let draw = app.draw();
    // draw.to_frame(app, &frame).unwrap();
}

fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}
