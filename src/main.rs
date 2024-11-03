//! This example illustrates scrolling in Bevy UI.

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, ecs::query::QueryData, input::mouse::{MouseScrollUnit, MouseWheel}, picking::focus::HoverMap, prelude::*, winit::WinitSettings
};
use rand::prelude::*;


//TODO: not permanent, this will be replaced
//      once i figure out a better solution
#[derive(Component)]
struct GameObject{
    id: u128
}

#[derive(Component)]
struct InspectorList{

}

#[derive(Component)]
struct InspectorListing{
    id: u128
}

fn add_button(mut cmd: &mut Commands, obj_id: u128) -> Entity {
    let it = cmd.spawn_empty()
    .insert(Node {
        min_height: Val::Px(LINE_HEIGHT),
        max_height: Val::Px(LINE_HEIGHT),
        ..default()
    })
    .insert(BackgroundColor(Color::srgb(0.3125, 0.3125, 0.3125)))
    .insert(Button{})
    .insert(InspectorListing{id:obj_id})
    .insert(PickingBehavior {
        should_block_lower: false,
        is_hoverable: true,
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn((
                Text(format!("New Game Object")),
                TextFont {
                    ..default()
                },
                Label,
                AccessibilityNode(NodeBuilder::new(
                    Role::ListItem,
                )),
            ))
            .insert(PickingBehavior {
                should_block_lower: false,
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
        .add_systems(Update, select)
        .add_systems(Update, update_entities)
        ;

    app.run();
}

const FONT_SIZE: f32 = 20.;
const LINE_HEIGHT: f32 = 24.;

fn select(

    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
        ),
    (Changed<Interaction>, With<Button>),>
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
    mut iq: Query<(&InspectorListing, Entity), With<InspectorListing>>,
    mut i: Query<(Entity), With<InspectorList>>,
    mut tq: Query<(&GameObject), With<GameObject>>
){
    let mut bla = 0;

    for(child, ent) in iq.iter(){
        let mut found = false;
        for t in tq.iter(){
            if(child.id == t.id){
                found=true;
            }
        }
        if !found {
            cmd.entity(ent).despawn_recursive();

        }
    }
    for t in tq.iter(){
        let mut found = false;
        for (child, ent) in iq.iter(){
            if(child.id == t.id){
                found = true;
            }
        }
        if !found {
            cmd.entity(i.single()).add_child(add_button(&mut cmd2,t.id));
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
                                        overflow: Overflow::scroll_y(), // n.b.
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
                            trigger: Trigger<Pointer<Up>>,
                            mut commands: Commands
                        | {
                            if trigger.event().button == PointerButton::Primary {
                                let rn = rand::random::<u128>();
                                commands.spawn_empty()
                                .insert(GameObject{id:rn});
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