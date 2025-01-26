//! Murder

use std::ptr::null;

use bevy::{prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};

use rand::*;

use accesskit::{Node as Accessible, Role};
use bevy::{
    a11y::AccessibilityNode,
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    winit::WinitSettings,
};

#[derive(Component, PartialEq, Clone)]
struct GameObject{
    ent: Entity,
    name: String,
    id: u128
}

#[derive(Component)]
struct InspectorListing{
    obj: GameObject
}

#[derive(Component)]
struct InspectorList{

}

#[derive(Component)]
struct Selected{

}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(
        RenderPlugin {
            render_creation:
                RenderCreation::Automatic(
                    WgpuSettings{
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }
                ),
            ..default()
        }
    ))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, update_scroll_position)
        .add_systems(Update, update_inspector_list)
        .add_systems(Update, color_selected);

    app.run();
}

const LINE_HEIGHT: f32 = 21.;

fn update_inspector_list(
    mut commands: Commands,
    g: Query<&GameObject>,
    i: Query<(Entity, &InspectorListing)>,
    il: Query<(Entity, &InspectorList)>
){
    for l in i.iter(){
        let mut found = false;
        for o in g.iter(){
            if l.1.obj == *o{
                found = true;
            }
        }
        if !found{
            println!("Listing found for GameObject that doesn't exist. Deleting.");
            commands.entity(l.0).despawn_recursive();
        }
    }

    for o in g.iter(){
        let mut found = false;
        for l in i.iter(){
            if l.1.obj == *o{
                found = true;
            }
        }
        if !found {
            println!("GameObject found without corresponding listing. Adding.");
            add_button(&mut commands, o,il.single().0);
        }
    }
}

fn add_button(
    commands: &mut Commands,
    o: &GameObject,
    il: Entity
){
    let obj = o.clone();
    let mut bla = commands.spawn((
        Node{
            min_width: Val::Percent(100.0),
            min_height: Val::Px(24.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()},
        BackgroundColor(
            Color::srgb(0.4, 0.4, 0.4)
        ),
        PickingBehavior{
            should_block_lower: false,
            is_hoverable: true},
        Button{},
        InspectorListing{obj:o.clone()}
    )).with_children(|parent|{
        parent.spawn(Text(o.name.clone()));
    }).observe(move|
        t: Trigger<Pointer<Click>>,
        kb: Res<ButtonInput<KeyCode>>,
        s: Query<(Entity, &Selected)>,
        mut cmd:Commands|{
        if t.event().button == PointerButton::Primary{
            if !kb.pressed(KeyCode::ControlLeft){
                for sel in s.iter(){
                    cmd.entity(sel.0).remove::<Selected>();
                }
            }
            cmd.entity(t.entity()).insert(Selected{});
        }
    }).id();
    commands.entity(il).add_children(&[bla]);
}

fn color_selected(
    mut commands: Commands,
    il: Query<(Entity, &InspectorListing)>,
    s: Query<(Entity, &Selected)>
){
    for l in il.iter(){
        commands.entity(l.0).insert(
            BackgroundColor(
                Color::srgb(0.4, 0.4, 0.4)
            )
        );
    }

    for sel in s.iter(){
        commands.entity(sel.0).insert(
            BackgroundColor(
                Color::srgb(1.0, 0.0, 0.3125)
            )
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3d{..default()});

    for i in 1..100{
        let mut o = commands.spawn_empty();
        o.insert(GameObject{
            ent: o.id(),
            name:i.to_string(),
            id:rand::random::<u128>()});
    }
    
    commands.spawn((
        Node{
            width: Val::Percent(25.0),
            height: Val::Percent(50.0),
            overflow: Overflow::scroll(),
            flex_direction: FlexDirection::Column,
            ..default()},
        BackgroundColor(
            Color::srgba(0.0, 0.0, 0.0,0.33)
        ),
    )).with_children(|parent|{
        parent.spawn(
            (Node{
                width: Val::Percent(100.0),
                height: Val::Auto,
                overflow: Overflow::scroll_y(),
                flex_direction: FlexDirection::Column,
                ..default()},
            InspectorList{}
        ));
    });

}

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}


