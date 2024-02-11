#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::world::World;
use cell::{Cell, CellId};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod cell;
mod world;

struct MouseInfo {
    first_mouse: bool,
    last_mouse_pos: (usize, usize),
    current_mouse_pos: (usize, usize),
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(640, 480);
        WindowBuilder::new()
            .with_title("Falling Sand")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(64, 48, surface_texture)?
    };

    let mut world = World::new();

    let mut mouse_info = MouseInfo {
        first_mouse: true,
        last_mouse_pos: (0, 0),
        current_mouse_pos: (0, 0),
    };
    let mut half_brush_size: usize = 0;

    let mut placeable_cells = [
        CellId::Sand,
        CellId::Water,
        CellId::Stone,
        CellId::Dirt,
        CellId::Coal,
    ]
    .iter()
    .cycle();
    let mut cell_to_place = placeable_cells.next().unwrap().clone();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                cell_to_place = placeable_cells.next().unwrap().clone();
                println!("Placing {:?}", cell_to_place);
            }

            half_brush_size = half_brush_size.saturating_add_signed(input.scroll_diff() as isize);

            if input.mouse_held(0) || input.mouse_held(1) {
                if let Some((x, y)) = input.mouse() {
                    update_mouse_info(&mut mouse_info, x, y);

                    let cell_to_place = if input.mouse_held(0) {
                        cell_to_place
                    } else {
                        CellId::Air
                    };
                    place_cells(&mouse_info, &mut world, cell_to_place, half_brush_size);

                    mouse_info.last_mouse_pos = mouse_info.current_mouse_pos;
                }
            }

            if input.mouse_released(0) || input.mouse_released(1) {
                mouse_info.first_mouse = true;
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

fn update_mouse_info(mouse_info: &mut MouseInfo, x: f32, y: f32) {
    if mouse_info.first_mouse {
        mouse_info.first_mouse = false;
        mouse_info.last_mouse_pos = ((x / 10.0) as usize, (y / 10.0) as usize);
    }

    let current_mouse_pos = ((x / 10.0) as usize, (y / 10.0) as usize);
    mouse_info.current_mouse_pos = current_mouse_pos;
}

fn place_cells(
    mouse_info: &MouseInfo,
    world: &mut World,
    cell_to_place: CellId,
    half_brush_size: usize,
) {
    for (center_x, center_y) in bresenham::Bresenham::new(
        (
            mouse_info.last_mouse_pos.0 as isize,
            mouse_info.last_mouse_pos.1 as isize,
        ),
        (
            mouse_info.current_mouse_pos.0 as isize,
            mouse_info.current_mouse_pos.1 as isize,
        ),
    ) {
        for x in center_x - half_brush_size as isize..=center_x + half_brush_size as isize {
            for y in center_y - half_brush_size as isize..=center_y + half_brush_size as isize {
                world.set_cell(x as usize, y as usize, Cell::new(cell_to_place));
            }
        }
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
