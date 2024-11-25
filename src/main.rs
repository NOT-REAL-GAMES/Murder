//! Murder

use std::borrow::Borrow;
use std::ptr::null;

use rand::*;

use bevy::reflect::List;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    }, input::mouse::{MouseScrollUnit, MouseWheel}, picking::focus::HoverMap, prelude::*, winit::WinitSettings 
};


//TODO: - project creation window
//      - project creation process
//          - write project folder
//          - read from the folder
//      - INSPECTOR
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
}

#[derive(Component, Deref, Clone, Copy)]
struct ParentNode{
    pub expanded: bool,
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
struct Flymode{

}

#[derive(Component)]
struct FlyArea{

}

#[derive(Component)]
struct Prefab{
}

#[derive(Component)]
struct QueriedInspectorListing{

}


//TODO: figure out how to do this shit procedurally
fn TestPrefab(
    mdl: &mut ResMut<Assets<Mesh>>,
    mat: &mut ResMut<Assets<StandardMaterial>>
) -> impl Bundle{
    
    (
        Transform{..default()},
        Mesh3d(mdl.add(Cuboid{half_size: Vec3{
            x: rand::thread_rng().gen_range(0.0f32..5.0f32), 
            y: rand::thread_rng().gen_range(0.0f32..5.0f32), 
            z: rand::thread_rng().gen_range(0.0f32..5.0f32)}})),
        MeshMaterial3d(mat.add(StandardMaterial{..default()}))
    )
}

#[derive(Component, Default, Clone, PartialEq)]
struct InspectorListing{
    id: u128,
}

fn typeid<T: std::any::Any>(_: &T) -> &str {
    return std::any::type_name::<T>();
}

fn update_nodeless_parents(
    mut cmd: Commands,
    mut pnls: Query<(Entity), (With<GameObject>, Without<ParentNode>)>,
){
    for (e) in pnls.iter_mut(){
        cmd.entity(e).insert(ParentNode{expanded: false});
        println!("BAZINGA");
    }
}

fn clear_all_selected(
    sel: &mut Query<(Entity, &Selected), (With<GameObject>, With<Selected>)>,
    mut btn: Query<&mut BackgroundColor, With<InspectorListing>>,
    mut cmd: Commands,
    ent: Entity
) {
    for mut bg in btn.iter_mut(){
        bg.0 = Color::srgb(0.3125, 0.3125, 0.3125).into();
    }
    for (e, s) in sel.iter_mut(){
        if e != ent{
            cmd.entity(e).remove::<Selected>();
        }
    }

    
}

fn add_button(cmd: &mut Commands, obj: &GameObject) -> Entity {
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
        cmd: Commands,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut sel: Query<(Entity, &Selected), (With<GameObject>, With<Selected>)>,
        btn: Query<&mut BackgroundColor, With<InspectorListing>>,
        pn: Query<(&ParentNode)>,

    | {            
        let mut cnt = false;

        if trigger.event().button == PointerButton::Primary {
            let mut fuck = commands.entity(e);

            if !keyboard_input.pressed(KeyCode::ControlLeft) &&
                !keyboard_input.pressed(KeyCode::ControlRight) {
                
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
                let mut bla = pn.get(fuck.id());
                let mut uw = (*bla.ok().unwrap());
                uw.expanded = !uw.expanded;

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
        .add_systems(PreUpdate, update_nodeless_parents)
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

    q: Query<&mut BackgroundColor, With<Selected>>,

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
    i: Query<Entity, With<InspectorList>>,
    mut tfq: Query<&GameObject, With<GameObject>>,
    mut sel: Query<Entity, (With<GameObject>, With<Selected>)>,
    mut btn: Query<&mut Visibility, With<AddComponentButton>>,
    pt: Query<(&Parent, Entity), With<GameObject>>,
    pn: Query<(&ParentNode),With<GameObject>>,
    mut qd: Query<(&InspectorListing), With<QueriedInspectorListing>>


){
    for mut v in btn.iter_mut(){
        if sel.iter().len() > 0 {
            *v = Visibility::Visible;
        } else {
            *v = Visibility::Hidden;
        }
    }

    for(child, ent, bg) in iq.iter(){
        let mut found = false;
        for t in tfq.iter(){
            if qd.iter().len() > 0{
                for q in &mut qd{
                    if q == child{
                        found=true;
                    }
                }
            }
            else if child.id == t.id {
                found=true;
            }
        }
        if !found {
            cmd.entity(ent).despawn_recursive();

        }
    }
    for t in tfq.iter_mut(){
        let mut found = false;
        for (child, ent, mut bg) in iq.iter_mut(){
            if qd.iter().len() > 0{
                
            }
            else if child.id == t.id {
                found = true;
                for s in &mut sel{
                    if t.ent == s{
                        (bg).0 = Color::srgb(1.0, 0.0, 0.3125).into();
                    }
                }
            }
        }
        if !found {
            let size = pt.iter().len();
            let mut parented = false;
            for(parent,ent) in pt.iter(){
                if t.ent == ent {                
                    let res = parent.get();
                    parented = true;                
                }
                //TODO: figure out how to use the result here to
                //      find which parent each child belongs to
            }
            if !parented {
                let b = add_button(&mut cmd2,t);
                let mut o = cmd.entity(i.single());
                let c = o.add_child(b);
            }
        }
    }
}



fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 

) {

    commands.spawn(
        DirectionalLight{illuminance:1_000.00,..default()}
    );

    // Camera
    commands.spawn(
        (Camera3d{..default()},
        Transform{translation: Vec3 { x: 0.0, y: 0.0, z: 25.0 },..default()}));

    // root node
    let world = commands.spawn_empty().id();
        commands.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .insert(PickingBehavior{is_hoverable: false, should_block_lower: false})
        .with_children(|parent| {            

            parent.spawn((
                Node{
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },                
                FlyArea{}
            ))        
            .insert(PickingBehavior{is_hoverable: true, should_block_lower: false})
            .observe(|
                    trigger: Trigger<Pointer<Up>>,
                    mut cmd: Commands,
                    mut ent: Query<Entity, With<Flymode>>
                    |{
                        if trigger.event().button == PointerButton::Secondary {
                            cmd.entity(ent.single_mut()).remove::<Flymode>();
                        }
                    }
                ).observe(| 
                    trigger: Trigger<Pointer<Down>>,
                    mut cmd: Commands,
                    mut ent: Query<Entity, With<FlyArea>>           
                    | {
                        if trigger.event().button == PointerButton::Secondary {
                            cmd.entity(ent.single_mut()).insert(Flymode{});
                        }
                    }
                );

            // container for all other examples
            parent
                .spawn(Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    ..default()
                })
                .insert(PickingBehavior{is_hoverable: true, should_block_lower: false})
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
                                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.33)),
                                    
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
                                
                                new.insert(GameObject{
                                    id:rn,
                                    name:"New Game Object".to_string(),
                                    ent:new.id()})
                                    .with_children(
                                        |parent| {
                                            let mut bla = parent.spawn_empty();
                                            bla.insert (GameObject{
                                                id:0,
                                                name: "Child Object".to_string(),
                                                ent: bla.id()
                                            }
                                        );
                                    }
                                );
                            }
                        }
                    );
                    
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
                                q: Query<Entity, (With<Selected>,With<GameObject>)>,
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
                        }
                    );
                }
            );
        }
    );
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