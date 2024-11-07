//! Murder

use std::any::*;


use bevy::render::MainWorld;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, ecs::observer::TriggerTargets, input::mouse::{MouseScrollUnit, MouseWheel}, picking::focus::HoverMap, prelude::*, winit::WinitSettings 
};


//TODO: - project creation window
//      - project creation process
//          - write project folder
//          - read from the folder
//      - object selection code
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
struct Selected{

}

#[derive(Component, Default, Clone)]
struct InspectorListing{
    id: u128,
}

fn clear_all_selected(
    mut sel: Query<(Entity, &Selected), (With<GameObject>, With<Selected>)>,
    mut btn: Query<&mut BackgroundColor, With<InspectorListing>>,
    mut cmd: Commands,
) {
    for (mut bg) in btn.iter_mut(){
        bg.0 = Color::srgb(0.3125, 0.3125, 0.3125).into();
    }
    for (e, s) in sel.iter_mut(){
        cmd.entity(e).remove::<Selected>();
    }

    
}

fn add_button(mut cmd: &mut Commands, obj: &GameObject) -> Entity {
    let nm = &obj.name;
    let e = obj.ent;

    let it = cmd.spawn_empty()
    .insert(Node {
        min_height: Val::Px(LINE_HEIGHT),
        max_height: Val::Px(LINE_HEIGHT),
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
        if trigger.event().button == PointerButton::Primary {
            let mut fuck = commands.entity(e);

            if (!keyboard_input.pressed(KeyCode::ControlLeft) &&
                !keyboard_input.pressed(KeyCode::ControlRight)) {

                clear_all_selected(sel, btn, cmd);

            }

            let ffs = fuck.insert(Selected{});
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
        .add_systems(Update, update_entities)
        ;

    app.run();
}

const FONT_SIZE: f32 = 16.;
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
    let mut world = MainWorld::default();

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

fn print_type<T>(_: &T) -> &str { 
    return (std::any::type_name::<T>());
}

fn foo<T: fucking>() -> Selected {
    return Selected{};
}


trait fucking {const shit: bool = false;}

//TODO: only update when scene changes
//      instead of every frame like a moron
fn update_entities(
    mut cmd: Commands,
    mut cmd2: Commands,
    mut iq: Query<(&InspectorListing, Entity, &mut BackgroundColor), With<InspectorListing>>,
    mut i: Query<(Entity), With<InspectorList>>,
    mut tfq: Query<(&GameObject), With<GameObject>>,
    mut sel: Query<(Entity), (With<GameObject>, With<Selected>)>

){

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
        impl fucking for Selected {
            const shit: bool = true;
        };
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera3d{..default()});

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
                    // vertical scroll example
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
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
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