use std::borrow::Cow;
use std::rc::Rc;

use  glium::texture::Texture2d;
use glium::{implement_vertex, uniform, Display, Frame, Program, Surface};



use rusttype::gpu_cache::Cache;
use rusttype::{point, vector, Font, PositionedGlyph, Scale};
use std::error::Error;

use crate::{
    gui::{transform, Rect},
    vec2,
};

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
	colour: [f32; 4],
}
pub struct FontID(pub i32);
pub struct TextObject {
	pub text: String,
	pub font: FontID
}

pub struct TextSystem<'a> {
	program : Program,	
	cache: Cache<'a>,
	cache_tex: Rc<Texture2d>,
	font: Font<'a>
}

impl<'a> TextSystem<'a> {
	pub fn new(display: &Display) -> Result<TextSystem, Box<dyn Error>> {
        let vertex_shader_src = include_str!("./shaders/text.vert");
        let fragment_shader_src = include_str!("./shaders/text.frag");
        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

		let dpi_factor = display.gl_window().window().scale_factor();
		let (cache_width, cache_height) = ((512.0 * dpi_factor) as u32, (512.0 * dpi_factor) as u32);
		let mut cache = Cache::builder()
			.dimensions(cache_width, cache_height)
			.build();
			
		let cache_tex = Rc::new(glium::texture::Texture2d::with_format(
			display,
			glium::texture::RawImage2d {
				data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
				width: cache_width,
				height: cache_height,
				format: glium::texture::ClientFormat::U8,
			},
			glium::texture::UncompressedFloatFormat::U8,
			glium::texture::MipmapsOption::NoMipmap,
		)?);
		
		let font_data = include_bytes!("../../assets/UbuntuMono-R.ttf");
		let Some(font) = Font::try_from_bytes(font_data as &[u8]) else {
			panic!();
		};

		Ok(TextSystem { program, cache, cache_tex, font})
    }
    pub fn draw(
        &mut self,
        target: &mut Frame,
        display: &Display,
        camera_transform: transform::Transform,
        object: &TextObject
    )  {


		let dpi_factor = display.gl_window().window().scale_factor();
        let (width, _): (u32, _) = display
            .gl_window()
            .window()
            .inner_size()
            .into();
        let dpi_factor = dpi_factor as f32;
		let glyphs: Vec<PositionedGlyph<'_>> = {
			let font = &self.font;
			let scale = Scale::uniform(24.0 * dpi_factor);
			let text: &str = &object.text;
			let mut result = Vec::new();
			let v_metrics = font.v_metrics(scale);
			let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
			let mut caret = point(width as f32/2., v_metrics.ascent);
			let mut last_glyph_id = None;
			for c in text.chars() {
				if c.is_control() {
					match c {
						'\r' => {
							caret = point(0.0, caret.y + advance_height);
						}
						'\n' => {}
						_ => {}
					}
					continue;
				}
				let base_glyph = font.glyph(c);
				if let Some(id) = last_glyph_id.take() {
					caret.x += font.pair_kerning(scale, id, base_glyph.id());
				}
				last_glyph_id = Some(base_glyph.id());
				let mut glyph = base_glyph.scaled(scale).positioned(caret);
				if let Some(bb) = glyph.pixel_bounding_box() {
					if bb.max.x > width as i32 {
						caret = point(0.0, caret.y + advance_height);
						glyph.set_position(caret);
						last_glyph_id = None;
					}
				}
				caret.x += glyph.unpositioned().h_metrics().advance_width;
				result.push(glyph);
			}
			result
		};
        for glyph in &glyphs {
			let new = glyph.clone();
            self.cache.queue_glyph(0,new );
        }
        self.cache.cache_queued(|rect, data| {
            self.cache_tex.main_level().write(
                glium::Rect {
                    left: rect.min.x,
                    bottom: rect.min.y,
                    width: rect.width(),
                    height: rect.height(),
                },
                glium::texture::RawImage2d {
                    data: Cow::Borrowed(data),
                    width: rect.width(),
                    height: rect.height(),
                    format: glium::texture::ClientFormat::U8,
                },
            );
        }).unwrap();

		let vertex_buffer = self.vertex_buffer(display, glyphs);
		let uniforms = uniform! {
            tex: self.cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };
        target.draw(
            &vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &self.program,
            &uniforms,
            &glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        ).unwrap();
	}

    fn vertex_buffer(&mut self, display: &Display, glyphs: Vec<PositionedGlyph<'_>>) -> glium::VertexBuffer<Vertex> {
        implement_vertex!(Vertex, position, tex_coords, colour);
        let colour = [1.0, 0.0, 0.0, 1.0];
        let (screen_width, screen_height) = {
                    let (w, h) = display.get_framebuffer_dimensions();
                    (w as f32, h as f32)
                };
        let origin = point(0., 0.);
        let vertices: Vec<Vertex> = glyphs
                    .iter()
                    .flat_map(|g| {
                        if let Ok(Some((uv_rect, screen_rect))) = self.cache.rect_for(0, g) {
                            let gl_rect = rusttype::Rect {
                                min: origin
                                    + (vector(
                                        screen_rect.min.x as f32 / screen_width - 0.5,
                                        1.0 - screen_rect.min.y as f32 / screen_height - 0.5,
                                    )) * 2.0,
                                max: origin
                                    + (vector(
                                        screen_rect.max.x as f32 / screen_width - 0.5,
                                        1.0 - screen_rect.max.y as f32 / screen_height - 0.5,
                                    )) * 2.0,
                            };
                            arrayvec::ArrayVec::<[Vertex; 6]>::from([
                                Vertex {
                                    position: [gl_rect.min.x, gl_rect.max.y],
                                    tex_coords: [uv_rect.min.x, uv_rect.max.y],
                                    colour,
                                },
                                Vertex {
                                    position: [gl_rect.min.x, gl_rect.min.y],
                                    tex_coords: [uv_rect.min.x, uv_rect.min.y],
                                    colour,
                                },
                                Vertex {
                                    position: [gl_rect.max.x, gl_rect.min.y],
                                    tex_coords: [uv_rect.max.x, uv_rect.min.y],
                                    colour,
                                },
                                Vertex {
                                    position: [gl_rect.max.x, gl_rect.min.y],
                                    tex_coords: [uv_rect.max.x, uv_rect.min.y],
                                    colour,
                                },
                                Vertex {
                                    position: [gl_rect.max.x, gl_rect.max.y],
                                    tex_coords: [uv_rect.max.x, uv_rect.max.y],
                                    colour,
                                },
                                Vertex {
                                    position: [gl_rect.min.x, gl_rect.max.y],
                                    tex_coords: [uv_rect.min.x, uv_rect.max.y],
                                    colour,
                                },
                            ])
                        } else {
                            arrayvec::ArrayVec::new()
                        }
                    })
                    .collect();
        glium::VertexBuffer::new(display, &vertices).unwrap()
    }
}
