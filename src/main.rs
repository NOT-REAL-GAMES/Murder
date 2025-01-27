//! Murder

use std::{fs::File, io::Write, ptr::null};

use bevy::{prelude::*, reflect::Array, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};

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
    id: u32,
    code: String
}

#[derive(Component)]
struct InspectorListing{
    obj: GameObject
}

#[derive(Component)]
struct Queried{}

#[derive(Component)]
struct ParentNode{}

#[derive(Component)]
struct Root{}


#[derive(Component)]
struct InspectorList{

}

#[derive(Component)]
struct Selected{

}

#[derive(Component, Clone, Copy)]
struct ScrollLerp{
    x: f32,
    y: f32
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
        .add_systems(Startup, setup)
        .add_systems(Update, update_scroll_position)
        .add_systems(Update, update_inspector_list)
        .add_systems(PostUpdate, color_selected);

    app.run();
}

const LINE_HEIGHT: f32 = 21.;

fn update_inspector_list(
    mut commands: Commands,
    g: Query<&GameObject>,
    i: Query<(Entity, &InspectorListing)>,
    il: Query<(Entity, &InspectorList)>,
    ch: Query<(&Children, &GameObject),(With<Queried>)>,

){


    for l in i.iter(){
        let mut found = false;
        let mut child_of_queried = false;
        for o in g.iter(){
            if l.1.obj == *o{
                found = true;
                
            }
            
            if ch.iter().len() == 1{
                for qc in ch.single().0.iter(){
                    if l.1.obj.ent == *qc{
                        child_of_queried = true;
                    }
                }
            }
                
        }
        if !found{
            println!("Listing found for GameObject that doesn't exist. Deleting.");
            commands.entity(l.0).despawn_recursive();
        }
        if !child_of_queried{
            //is queried?
                println!("Listing found for child GameObject that isn't queried. Deleting.");
                commands.entity(l.0).despawn_recursive();    
        }
    }

    for o in g.iter(){
        let mut found = false;
        let mut child_of_queried = false;
        let mut fuck: &GameObject;
        for l in i.iter(){
            if l.1.obj == *o{
                found = true;

            }
                
        }
        if ch.iter().len() == 1{

            for qc in ch.single().0.iter(){
               if o.ent == *qc{
                    child_of_queried = true;
                }
            }
        }

        if (!found && child_of_queried) {
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
        t: Trigger<Pointer<Down>>,
        kb: Res<ButtonInput<KeyCode>>,
        s: Query<(Entity, &Selected)>,
        q: Query<(Entity, &Queried)>,
        il: Query<&InspectorListing>,
        go: Query<&GameObject>,
        mut cmd:Commands|{
        if t.event().button == PointerButton::Primary{
            if s.iter().len() == 1{
                if s.single().0 == t.entity(){
                    for sel in s.iter(){
                        cmd.entity(sel.0).remove::<Selected>();
                    }
                    for qr in q.iter(){
                        cmd.entity(qr.0).remove::<Queried>();
                    }
                    cmd.entity(il.get(t.entity()).unwrap().obj.ent).insert(Queried{});
                }
            }
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

    let mut root = commands.spawn_empty();
    root.insert(
        (GameObject{
            ent: root.id(),
            name: "root".to_string(), 
            id: 0,
            code: "".to_string()
        },Queried{},Root{})).with_children(|parent|{
        for i in 1..100{
            let mut o = parent.spawn_empty();
            o.insert(GameObject{
                ent: o.id(),
                name:i.to_string(),
                id:rand::random::<u32>(),
                code:"".to_string()})
                .with_children(|parent|{
                    let mut c = parent.spawn_empty();
                    c.insert(
                        GameObject{
                            ent: c.id(),
                            name: "fuck".to_string(),
                            id:rand::random::<u32>(),
                            code:"".to_string()
                        }
                    );
                }
            );
        }
    
    });
    
    let test = commands.spawn((
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
            InspectorList{},
            ScrollLerp{x:0.0,y:0.0}
        ));
    }).id();

}



/// Updates the scroll position of scrollable nodes in response to mouse input
/// TODO: make this work for every scrollable
fn update_scroll_position(
    mut cmd: Commands,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition, With<ScrollLerp>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scr: Query<(Entity, &ScrollLerp)>,
) {
    
    let mut hovering = false;
    let mut bla = ScrollLerp { x: 0.0, y: 0.0 };

    for (_pointer, pointer_map) in hover_map.iter() {
        for (entity, _hit) in pointer_map.iter() {
            if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                bla = *((scr.get(*entity)).unwrap().1);
                hovering = true;
            }
        }
    }
    

    let (mut dx, mut dy) = (0.0, 0.0);
    for mouse_wheel_event in mouse_wheel_events.read() {
        (dx,dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * 2.0,
                mouse_wheel_event.y * 2.0,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

    }
    

    if (dy>0.0 && bla.y<0.0) || (dy<0.0 && bla.y>0.0) {
        bla.y = 0.0;
    }
    if (dx>0.0 && bla.x<0.0) || (dx<0.0 && bla.x>0.0) {
        bla.x = 0.0;
    }

    if hovering{
        bla.x += dx;
        bla.y += dy;
    }

    if let Ok(mut scroll_position) = scrolled_node_query.get_mut(scr.single_mut().0) {
        scroll_position.offset_x -= bla.x;
        scroll_position.offset_y -= bla.y;
        
    }
        
    bla.x = bla.x.lerp(0.0,0.01);
    bla.y = bla.y.lerp(0.0,0.01);

    cmd.entity(scr.single_mut().0).insert(bla);
    
}


