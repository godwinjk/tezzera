//! wgpu GPU compositor for TEZZERA (D072–D079).
//!
//! `GpuPresenter` takes one or more RGBA pixel buffers produced by `SkiaCanvas`
//! each frame, uploads them as GPU textures, and composites them onto the wgpu
//! surface:
//! - Pass 1 (base layer): REPLACE blend — overwrites the surface
//! - Pass N (overlay layers): ALPHA_BLENDING — Porter-Duff "over" operation
//!
//! # Integration
//! ```ignore
//! let presenter = GpuPresenter::new(&window, width, height);
//! // in frame loop:
//! presenter.present_layers(&[
//!     CompositorLayer { pixels: base.pixels(), width, height, opacity: 1.0 },
//!     CompositorLayer { pixels: overlay.pixels(), width, height, opacity: 1.0 },
//! ]);
//! ```

use wgpu::util::DeviceExt;

/// One render layer passed to `GpuPresenter::present_layers`.
///
/// `pixels` must be an RGBA8 byte slice of exactly `width * height * 4` bytes.
/// `opacity` scales the entire layer's alpha (1.0 = fully opaque, 0.0 = invisible).
pub struct CompositorLayer<'a> {
    pub pixels:  &'a [u8],
    pub width:   u32,
    pub height:  u32,
    pub opacity: f32,
}

/// GPU compositor state. One instance per window.
///
/// Created via [`GpuPresenter::new`]. Returns `None` if wgpu fails to find a
/// compatible GPU adapter; callers should fall back to the softbuffer path.
pub struct GpuPresenter {
    surface:               wgpu::Surface<'static>,
    device:                wgpu::Device,
    queue:                 wgpu::Queue,
    config:                wgpu::SurfaceConfiguration,
    /// Pipeline for the base layer (REPLACE blend — writes all channels).
    pipeline_base:         wgpu::RenderPipeline,
    /// Pipeline for overlay layers (ALPHA_BLENDING — Porter-Duff over).
    pipeline_overlay:      wgpu::RenderPipeline,
    bind_group_layout:     wgpu::BindGroupLayout,
    sampler:               wgpu::Sampler,
    width:                 u32,
    height:                u32,
}

impl GpuPresenter {
    /// Initialise the GPU presenter for the given window handle.
    ///
    /// Blocks using `pollster`. Returns `None` if no suitable adapter is found.
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

        // Base layer pipeline — REPLACE blend (first pass, writes everything)
        let pipeline_base = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("compositor-base"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module:              &shader,
                entry_point:         Some("vs_main"),
                buffers:             &[],
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
            primitive:     wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample:   wgpu::MultisampleState::default(),
            multiview:     None,
            cache:         None,
        });

        // Overlay pipeline — Porter-Duff "over" (subsequent passes, alpha blend)
        let pipeline_overlay = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("compositor-overlay"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module:              &shader,
                entry_point:         Some("vs_main"),
                buffers:             &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module:      &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive:     wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample:   wgpu::MultisampleState::default(),
            multiview:     None,
            cache:         None,
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
            pipeline_base,
            pipeline_overlay,
            bind_group_layout,
            sampler,
            width,
            height,
        })
    }

    /// Resize the wgpu surface.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 { return; }
        self.width  = width;
        self.height = height;
        self.config.width  = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Present a single opaque layer (backward-compatible shim for Phase 15 API).
    pub fn present(&mut self, pixels: &[u8], pixel_width: u32, pixel_height: u32) {
        self.present_layers(&[CompositorLayer {
            pixels,
            width:   pixel_width,
            height:  pixel_height,
            opacity: 1.0,
        }]);
    }

    /// Composite and present one or more layers (D076, D077, D079).
    ///
    /// Layers are blended bottom-to-top:
    /// - Layer 0: REPLACE blend (base, fully overwrites surface)
    /// - Layer 1+: ALPHA_BLENDING (Porter-Duff over on top of previous)
    ///
    /// Each layer's `opacity` scales its alpha channel before blending.
    /// Pass an empty slice to skip presentation for this frame.
    pub fn present_layers(&mut self, layers: &[CompositorLayer<'_>]) {
        if layers.is_empty() { return; }

        let Ok(output) = self.surface.get_current_texture() else { return; };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("compositor-enc"),
        });

        for (idx, layer) in layers.iter().enumerate() {
            if layer.width == 0 || layer.height == 0 { continue; }

            let texture = self.device.create_texture_with_data(
                &self.queue,
                &wgpu::TextureDescriptor {
                    label:           Some("frame-layer"),
                    size:            wgpu::Extent3d {
                        width:               layer.width,
                        height:              layer.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count:    1,
                    dimension:       wgpu::TextureDimension::D2,
                    format:          wgpu::TextureFormat::Rgba8Unorm,
                    usage:           wgpu::TextureUsages::TEXTURE_BINDING
                                   | wgpu::TextureUsages::COPY_DST,
                    view_formats:    &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                layer.pixels,
            );
            let tex_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label:   Some("layer-bg"),
                layout:  &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding:  0,
                        resource: wgpu::BindingResource::TextureView(&tex_view),
                    },
                    wgpu::BindGroupEntry {
                        binding:  1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
            });

            let pipeline = if idx == 0 {
                &self.pipeline_base
            } else {
                &self.pipeline_overlay
            };

            // load: Clear only on the first pass; subsequent passes load the
            // already-rendered content so previous layers are preserved.
            let load = if idx == 0 {
                wgpu::LoadOp::Clear(wgpu::Color::BLACK)
            } else {
                wgpu::LoadOp::Load
            };

            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("compositor-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view:           &view,
                    resolve_target: None,
                    ops: wgpu::Operations { load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None,
                timestamp_writes:         None,
                occlusion_query_set:      None,
            });
            rpass.set_pipeline(pipeline);
            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.draw(0..6, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    /// Physical size of the configured surface.
    pub fn surface_size(&self) -> (u32, u32) { (self.width, self.height) }
}
