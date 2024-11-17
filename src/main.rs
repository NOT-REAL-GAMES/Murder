//! Murder

use std::ptr::null;
use std::{any::*, hash::Hash};
use std::borrow::Borrow;

use bevy::color::palettes::css::RED;
use bevy::ecs::bundle::DynamicBundle;
use bevy::reflect::List;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, ecs::{archetype::Archetypes, component::Components, observer::TriggerTargets, world::WorldId}, input::mouse::{MouseScrollUnit, MouseWheel}, math::primitives, picking::focus::HoverMap, prelude::*, winit::WinitSettings 
};


//TODO: - project creation window
//      - project creation process
//          - write project folder
//          - read from the folder
//      - INSPECTOR
//          - object selection code - WIP
//          - object deselection code - WIP
//          - parenting/reordering objects
//          - renaming objects
//      - menu for selected object/s
//      - ability to add components 
//      - divide project into files

#[derive(Component, Clone)]
struct GameObject{
    id: u128,
    name: String,
    ent: Entity,
    btn: Entity,
}

#[derive(Component)]
struct InspectorList{

}

#[derive(Component)]
struct AddComponentButton{

}

#[derive(Component)]
struct Selected{

}

#[derive(Component)]
struct Prefab{
}

fn TestPrefab(
    mdl: &mut ResMut<Assets<Mesh>>,
    mat: &mut ResMut<Assets<StandardMaterial>>
) -> impl Bundle{
    
    (
        Transform{..default()},
        Mesh3d(mdl.add(Cuboid{half_size: Vec3{x: 1.0, y: 1.0, z:1.0}})),
        MeshMaterial3d(mat.add(
            StandardMaterial{
                base_color:RED.into(),
                ..default()}))
    )
}

#[derive(Component, Default, Clone)]
struct InspectorListing{
    id: u128,
}

fn typeid<T: std::any::Any>(_: &T) -> &str {
    return std::any::type_name::<T>();
}


fn clear_all_selected(
    mut sel: &mut Query<(Entity, &Selected), (With<GameObject>, With<Selected>)>,
    mut btn: Query<&mut BackgroundColor, With<InspectorListing>>,
    mut cmd: Commands,
    mut ent: Entity
) {
    for (mut bg) in btn.iter_mut(){
        bg.0 = Color::srgb(0.3125, 0.3125, 0.3125).into();
    }
    for (e, s) in sel.iter_mut(){
        if e != ent{
            cmd.entity(e).remove::<Selected>();
        }
    }

    
}

fn add_button(mut cmd: &mut Commands, obj: &GameObject) -> Entity {
    let nm = &obj.name;
    let e = obj.ent;

    let it = cmd.spawn_empty()
    .insert(Node {
        min_height: Val::VMin(2.5),
        max_height: Val::VMin(2.5),
        ..default()
    })
    .observe(move |
        trigger: Trigger<Pointer<Click>>,
        mut commands: Commands,
        mut cmd: Commands,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut sel: Query<(Entity, &Selected), (With<GameObject>, With<Selected>)>,
        mut btn: Query<&mut BackgroundColor, With<InspectorListing>>,

    | {            
        let mut cnt = false;

        if trigger.event().button == PointerButton::Primary {
            let mut fuck = commands.entity(e);

            if (!keyboard_input.pressed(KeyCode::ControlLeft) &&
                !keyboard_input.pressed(KeyCode::ControlRight)) {
                
                clear_all_selected(&mut sel, btn, cmd, e);

            } else {
                for (e, s) in &mut sel{
                    if e == fuck.id() {
                        fuck.remove::<Selected>();
                        cnt = true;
                    } 
                }
            }
            
            if !cnt {
                let ffs = fuck.insert(Selected{});
            }
        }
    })
    .insert(BackgroundColor(Color::srgb(0.3125, 0.3125, 0.3125)))
    .insert(Button{})
    .insert(InspectorListing{id:obj.id})
    .insert(PickingBehavior {
        should_block_lower: false,
        is_hoverable: true,
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn((
                Text(format!("{nm}")),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                Label,
                AccessibilityNode(NodeBuilder::new(
                    Role::ListItem,
                )),
            ))
            .insert(Node{
                align_self: AlignSelf::Center,
                ..default()
            })
            .insert(PickingBehavior {
                should_block_lower: false,
                is_hoverable: false,
                ..default()
            });
        })    
    .id();
    

    return it;
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin{
            primary_window: Some(Window {
                title: "Murder".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, update_scroll_position)
        .add_systems(Update, highlight)
        .add_systems(PostUpdate, update_entities)
        ;

    app.run();
}

const FONT_SIZE: f32 = 12.;
const LINE_HEIGHT: f32 = 24.;

fn highlight(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
        ),
    (Changed<Interaction>, With<Button>, Without<Selected>)>,

    mut q: Query<(&mut BackgroundColor), With<Selected>>,

){

    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.29, 0.15, 0.2225).into();
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.58, 0.3, 0.445).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.3125, 0.3125, 0.3125).into();
            }
        }
    }
}



//TODO: only update when scene changes
//      instead of every frame like a moron
fn update_entities(
    mut cmd: Commands,
    mut cmd2: Commands,
    mut iq: Query<(&InspectorListing, Entity, &mut BackgroundColor), With<InspectorListing>>,
    mut i: Query<(Entity), With<InspectorList>>,
    mut tfq: Query<(&GameObject), With<GameObject>>,
    mut sel: Query<(Entity), (With<GameObject>, With<Selected>)>,
    mut btn: Query<&mut Visibility, With<AddComponentButton>>

){
    for mut v in btn.iter_mut(){
        if sel.iter().len() > 0 {
            *v = Visibility::Visible;
        } else {
            *v = Visibility::Hidden;
        }
    }

    for(child, ent, mut bg) in iq.iter(){
        let mut found = false;
        for (t) in tfq.iter(){
            if(child.id == t.id){
                found=true;
            }
        }
        if !found {
            cmd.entity(ent).despawn_recursive();

        }
    }
    for (mut t) in tfq.iter_mut(){
        let mut found = false;
        for (child, ent, mut bg) in iq.iter_mut(){
            if(child.id == t.id){
                found = true;
                for (s) in &mut sel{
                    if t.ent == s{
                        (bg).0 = Color::srgb(1.0, 0.0, 0.3125).into();
                    }
                }
            }
        }
        if !found {
            
            let b = add_button(&mut cmd2,t);
            let mut o = cmd.entity(i.single());
            let c = o.add_child(b);
        }
    }
}



fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {

    commands.spawn(
        DirectionalLight{illuminance:100_000.00,..default()}
    );

    // Camera
    commands.spawn(
        (Camera3d{..default()},
        Transform{translation: Vec3 { x: 0.0, y: 0.0, z: 10.0 },..default()}));

    // root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {            

            // container for all other examples
            parent
                .spawn(Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|parent| {
                    // inspector
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Start,
                            width: Val::Vw(25.),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Scrolling list
                            parent
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Column,
                                        align_self: AlignSelf::Stretch,
                                        height: Val::Vh(50.),
                                        overflow: Overflow{
                                            x: OverflowAxis::Clip, 
                                            y: OverflowAxis::Scroll},
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                                ))
                                .insert(InspectorList{});
                        });
                    
                    parent
                        .spawn((Node{
                            width: Val::VMin(4.0),
                            height: Val::VMin(4.0),
                            right: Val::Px(0.0),
                            
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        Button{..default()},
                        BackgroundColor(Color::srgb(0.3125,0.3125,0.3125))
                        ))
                        .observe(|
                            trigger: Trigger<Pointer<Click>>,
                            mut commands: Commands
                        | {
                            if trigger.event().button == PointerButton::Primary {
                                let rn = rand::random::<u128>();
                                let rns = rn.to_string();
                                println!("Created object: {rns}");
                                let mut new = commands.spawn_empty();
                                
                                new.insert(GameObject{id:rn,name:"New Game Object".to_string(),ent:new.id(),btn:Entity::PLACEHOLDER});
                            }
                        });
                    
                    parent
                        .spawn(
                            (Node{
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(16.0),
                                left: Val::Px(16.0),
                                width: Val::Auto,
                                height: Val::Auto,    
                                ..default()
                            },
                            Button{..default()},
                            BackgroundColor(Color::srgb(0.3125,0.3125,0.3125)),
                            AddComponentButton{},
                            Visibility::Hidden 
                        ))
                        .with_children(|parent|{
                            parent.spawn((
                                Node{..default()},
                                Text("Add Component".to_string()),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                Label,
                            )
                            )
                            .observe(|
                                trigger: Trigger<Pointer<Click>>,
                                mut q: Query<Entity, (With<Selected>,With<GameObject>)>,
                                mut mdl: ResMut<Assets<Mesh>>,
                                mut mat: ResMut<Assets<StandardMaterial>>,
                                mut commands: Commands
                            | {
                                if trigger.event().button == PointerButton::Primary {
                                    for e in q.iter(){
                                        commands.entity(e).insert(TestPrefab(&mut mdl,&mut mat));
                                    }
                                }
                            });
                        });
                    });
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