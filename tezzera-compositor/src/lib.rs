//! wgpu GPU compositor for TEZZERA (D072–D075).
//!
//! `GpuPresenter` takes the RGBA pixel buffer produced by `SkiaCanvas` each
//! frame, uploads it to a GPU texture, and blits it to the wgpu surface via a
//! minimal fullscreen-quad shader. This replaces the softbuffer memcpy path in
//! `tezzera-platform`, keeping GPU code entirely out of the widget/render
//! crates.
//!
//! # Integration
//! ```ignore
//! let presenter = GpuPresenter::new(&window, width, height);
//! // in frame loop:
//! presenter.present(canvas.pixels(), canvas.width(), canvas.height());
//! ```

use wgpu::util::DeviceExt;

/// GPU compositor state. One instance per window, lives as long as the window.
///
/// Created via [`GpuPresenter::new`]. Returns `None` if wgpu fails to find a
/// compatible GPU adapter; callers should fall back to the softbuffer path.
pub struct GpuPresenter {
    surface:            wgpu::Surface<'static>,
    device:             wgpu::Device,
    queue:              wgpu::Queue,
    config:             wgpu::SurfaceConfiguration,
    pipeline:           wgpu::RenderPipeline,
    bind_group_layout:  wgpu::BindGroupLayout,
    sampler:            wgpu::Sampler,
    width:              u32,
    height:             u32,
}

impl GpuPresenter {
    /// Initialise the GPU presenter for the given window handle.
    ///
    /// Blocks the calling thread using `pollster` while wgpu negotiates with
    /// the GPU driver. Returns `None` if no suitable adapter is found or device
    /// creation fails (safe fallback path — caller uses softbuffer instead).
    ///
    /// # Safety
    /// `window` must outlive this `GpuPresenter`. The `'static` bound on
    /// `wgpu::Surface` is satisfied by holding the window `Arc` alongside the
    /// presenter (caller's responsibility).
    pub fn new<W>(window: W, width: u32, height: u32) -> Option<Self>
    where
        W: wgpu::rwh::HasWindowHandle
            + wgpu::rwh::HasDisplayHandle
            + Send
            + Sync
            + 'static,
    {
        pollster::block_on(Self::new_async(window, width, height))
    }

    async fn new_async<W>(window: W, width: u32, height: u32) -> Option<Self>
    where
        W: wgpu::rwh::HasWindowHandle
            + wgpu::rwh::HasDisplayHandle
            + Send
            + Sync
            + 'static,
    {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window).ok()?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        log::info!(
            "wgpu: {} backend, adapter = {}",
            adapter.get_info().backend,
            adapter.get_info().name,
        );

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label:             Some("tezzera-compositor"),
                required_features: wgpu::Features::empty(),
                required_limits:   wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints:      Default::default(),
                ..Default::default()
            }, None)
            .await
            .ok()?;

        let caps   = surface.get_capabilities(&adapter);
        let format = caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage:        wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width:        width.max(1),
            height:       height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode:   caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label:  Some("compositor"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("frame-texture-bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding:    0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type:    wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled:   false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding:    1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("compositor-pl"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("compositor-rp"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module:      &shader,
                entry_point: Some("vs_main"),
                buffers:     &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module:      &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend:      Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive:    wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample:  wgpu::MultisampleState::default(),
            multiview:    None,
            cache:        None,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter:     wgpu::FilterMode::Nearest,
            min_filter:     wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Some(Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            bind_group_layout,
            sampler,
            width,
            height,
        })
    }

    /// Resize the wgpu surface to match the new physical window dimensions.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 { return; }
        self.width  = width;
        self.height = height;
        self.config.width  = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Upload `pixels` (RGBA8, row-major, `pixel_width × pixel_height`) to the
    /// GPU and blit it to the surface via the fullscreen-quad pipeline.
    ///
    /// `pixels` must be exactly `pixel_width * pixel_height * 4` bytes. This is
    /// called once per frame after tiny-skia has finished drawing.
    pub fn present(&mut self, pixels: &[u8], pixel_width: u32, pixel_height: u32) {
        if pixel_width == 0 || pixel_height == 0 { return; }

        let Ok(output) = self.surface.get_current_texture() else { return; };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Upload the CPU pixel buffer as a 2-D RGBA8 texture.
        let texture = self.device.create_texture_with_data(
            &self.queue,
            &wgpu::TextureDescriptor {
                label:           Some("frame"),
                size:            wgpu::Extent3d { width: pixel_width, height: pixel_height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count:    1,
                dimension:       wgpu::TextureDimension::D2,
                format:          wgpu::TextureFormat::Rgba8Unorm,
                usage:           wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats:    &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            pixels,
        );
        let tex_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("frame-bg"),
            layout:  &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&tex_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("compositor-enc"),
        });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("compositor-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view:           &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes:         None,
                occlusion_query_set:      None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.draw(0..6, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    /// Physical size of the configured surface.
    pub fn surface_size(&self) -> (u32, u32) { (self.width, self.height) }
}
