pub use std::f32::consts::FRAC_PI_2;
pub use std::ops::Deref;
pub use std::time::{Duration, Instant};
pub use std::str::Split;

pub use std::any::Any;
pub use bevy::ecs::world::World;
pub use logos::Logos;

pub use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

pub use std::collections::*;

pub use bevy::text::FontSmoothing;
pub use bevy::{ core_pipeline::{prepass::*,experimental::taa::*, *}, prelude::*, render::{render_resource::{TextureViewDescriptor, TextureViewDimension}, settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};
pub use bevy::ui::widget::TextNodeFlags;
pub use bevy::ui::ContentSize;
pub use bevy::window::{PresentMode, WindowResized};
pub use bevy::math::*;
pub use bevy::{
    asset::AssetLoader, 
    core_pipeline::{
        prepass::*,
        experimental::taa::*}, 
    prelude::*, 
    window::{PrimaryWindow, WindowResolution}};

pub use bevy::{
    input::mouse::{MouseWheel, AccumulatedMouseMotion},
    picking::hover::HoverMap,
    pbr::*,
};

#[derive(Component, PartialEq, Clone)]
pub struct GameObject{
    pub ent: Entity,
    pub name: String,
    pub id: u32,
    pub code: String
}
