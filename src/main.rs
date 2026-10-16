// La version de entrega abre la ventana SDL sin una consola adicional en Windows.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use carga_modelos::{
    framebuffer::{Framebuffer, SCREEN_HEIGHT, SCREEN_WIDTH},
    obj_loader::load_obj,
    renderer::{prepare_vertices, render},
};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

fn project_root() -> Result<PathBuf, String> {
    let current = std::env::current_dir().map_err(|e| e.to_string())?;
    if current.join("assets/Nave_Espacial.obj").exists() {
        return Ok(current);
    }
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    // Funciona desde bin/ y target/release/, incluso al abrir por doble clic.
    for parent in executable.ancestors().skip(1) {
        if parent.join("assets/Nave_Espacial.obj").exists() {
            return Ok(parent.to_path_buf());
        }
    }
    Ok(current)
}

fn run() -> Result<(), String> {
    std::env::set_current_dir(project_root()?).map_err(|e| e.to_string())?;
    let mut headless = false;
    let mut once = false;
    let mut obj = PathBuf::from("assets/Nave_Espacial.obj");
    let mut output = PathBuf::from("docs/render.png");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--headless" => headless = true,
            "--once" => once = true,
            "--obj" => obj = PathBuf::from(args.next().ok_or("Falta la ruta despues de --obj")?),
            "--output" => {
                output = PathBuf::from(args.next().ok_or("Falta la ruta despues de --output")?)
            }
            _ => return Err(
                "Uso: carga_modelos [--headless] [--once] [--obj modelo.obj] [--output imagen.png]"
                    .into(),
            ),
        }
    }
    let model = load_obj(Path::new(&obj))?;
    let mut buffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let vertices = prepare_vertices(&model, buffer.width, buffer.height);
    let count = render(&model, &vertices, &mut buffer);
    buffer.save_png(&output)?;
    if headless || once {
        println!(
            "Rust | Vertices: {} | Triangulos recorridos: {count}\nImagen: {}",
            model.vertices.len(),
            output.display()
        );
    }
    if headless {
        return Ok(());
    }

    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video
        .window(
            "Lab 4 - Nave espacial - Rust",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA32,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .map_err(|e| e.to_string())?;
    let mut events = sdl.event_pump()?;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => buffer.save_png(&output)?,
                _ => {}
            }
        }
        buffer.clear();
        render(&model, &vertices, &mut buffer);
        // SDL solo presenta el framebuffer. Los triangulos ya los relleno Rust en CPU.
        texture.with_lock(None, |pixels, pitch| {
            for (y, row) in buffer.pixels.chunks_exact(SCREEN_WIDTH * 4).enumerate() {
                pixels[y * pitch..y * pitch + row.len()].copy_from_slice(row);
            }
        })?;
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
        if once {
            break;
        }
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        if !std::env::args().any(|arg| arg == "--headless" || arg == "--once") {
            let _ = sdl2::messagebox::show_simple_message_box(
                sdl2::messagebox::MessageBoxFlag::ERROR,
                "No se pudo abrir la nave",
                &error,
                None,
            );
        }
        std::process::exit(1);
    }
}
