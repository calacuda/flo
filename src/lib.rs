pub mod camera_controller;
pub mod constants;
pub mod fps_logger;
pub mod gltf_loader;
pub mod mesh;
pub mod mesh_textured;
pub mod skinned_mesh;
pub mod texture;
pub mod utils;
pub mod vulkan_common;
pub mod vulkan_renderer_unified;
// pub mod egui_integration;
pub mod memory_pool;

// Re-export ash for use in consuming applications
pub use ash;

use bevy::a11y::AccessibilityPlugin;
use bevy::animation::AnimationPlugin;
use bevy::asset::AssetPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::gltf::GltfPlugin;
use bevy::input::keyboard::KeyboardFocusLost;
use bevy::input::InputPlugin;
use bevy::pbr::PbrPlugin;
use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::texture::ImagePlugin;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::transform::TransformPlugin;
// use bevy::window::{Window, WindowPlugin};
use bevy::winit::{WakeUp, WinitPlugin};

pub fn setup_bevy_app() -> App {
    // setup_bevy_app_with_window(2560.0, 1440.0, "Flo Engine Example")
    setup_bevy_app_with_window()
}

pub fn setup_bevy_app_with_window(// width: f32, height: f32, title: &str
) -> App {
    // std::env::set_var("RUST_BACKTRACE", "0");

    let mut app = App::new();
    app.add_event::<KeyboardFocusLost>().add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        // WindowPlugin {
        //     primary_window: Some(Window {
        //         title: title.to_string(),
        //         resolution: (width, height).into(),
        //         ..default()
        //     }),
        //     ..default()
        // },
        AccessibilityPlugin,
        InputPlugin::default(),
        WinitPlugin::<WakeUp>::default(),
        TransformPlugin,
        RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        },
        ImagePlugin::default(),
        CorePipelinePlugin::default(),
        PbrPlugin::default(),
        ScenePlugin,
        GltfPlugin::default(),
        AnimationPlugin,
    ));

    app
}

