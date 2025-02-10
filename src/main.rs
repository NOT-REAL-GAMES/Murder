//! Murder: an editor for Bevy made in Bevy


use std::f32::consts::FRAC_PI_2;
use bevy::render::render_resource::AsBindGroup;

use bevy::{asset::AssetLoader, core_pipeline::{prepass::*,experimental::taa::*, *}, prelude::*, render::{render_resource::{TextureViewDescriptor, TextureViewDimension}, settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, window::{PrimaryWindow, WindowResolution}};


use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel, AccumulatedMouseMotion},
    picking::focus::HoverMap,
    pbr::*,pbr::experimental::meshlet::*,
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
struct Showing{}

#[derive(Component)]
struct Root{}

#[derive(Component)]
struct World{}

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
    ).set(        
        WindowPlugin{
        primary_window: Some(Window{resolution:
            WindowResolution::new(1280.0,720.0).with_scale_factor_override(0.5),
            ..default()}),
        ..default()
    }
    )).add_plugins((
        TemporalAntiAliasPlugin,
        MeshletPlugin{cluster_buffer_slots: 1048576},
    ))  
        // RESOURCES
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
        
        // SYSTEMS
        .add_systems(PreStartup, setup)        
        .add_systems(Startup, set_window_title)

        .add_systems(PreUpdate, fallback_to_root)
        .add_systems(PreUpdate, devcam_look)

        .add_systems(Update, update_scroll_position)
        .add_systems(Update, update_inspector_list)

        .add_systems(PostUpdate, color_selected)
        
        .add_systems(Last, delete_objects)
        ;

    app.run();
}


//TODO: add editable component that title is read from
fn set_window_title(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.title = "Murder".to_string();
    } 
}

fn delete_objects(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    s: Query<(&InspectorListing, &Selected)>,

) {
    if kb.pressed(KeyCode::Delete){
        for sel in s.iter(){
            commands.entity(sel.0.obj.ent).despawn_recursive();
        }
    }
}

fn update_inspector_list(
    mut commands: Commands,
    g: Query<&GameObject, Without<Showing>>,
    i: Query<(Entity, &InspectorListing)>,
    il: Query<(Entity, &InspectorList)>,
    ch: Query<(&Children, &GameObject),With<Queried>>,

){


    for l in i.iter(){
        let mut found = false;
        let mut child_of_queried = false;
        
        if Some(g.get(l.1.obj.ent)).is_some(){
            found = true;
        }

        if ch.iter().len() == 1{
            for qc in ch.single().0.iter(){
                if l.1.obj.ent == *qc{
                    child_of_queried = true;
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

        if !found && child_of_queried {
            println!("GameObject found without corresponding listing. Adding.");
            add_button(&mut commands, o,il.single().0);
            commands.entity(o.ent).insert(Showing{});
        }
    }
}

fn add_button(
    commands: &mut Commands,
    o: &GameObject,
    il: Entity
){
    let bla = commands.spawn((
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
    il: Query<(Entity, &Interaction, &Button, &BackgroundColor)>,
    s: Query<(Entity, &Selected)>
){

    for l in il.iter(){

        let col: Color = match l.1 {
            Interaction::None => Color::srgba(0.4, 0.4, 0.4, l.3.0.alpha()) ,
            Interaction::Hovered => Color::srgba(0.6, 0.4, 0.4, l.3.0.alpha()),
            Interaction::Pressed => Color::srgba(0.3, 0.15, 0.15, l.3.0.alpha())
        }; 

        commands.entity(l.0).insert(
            BackgroundColor(col)
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

fn fallback_to_root(
    mut cmd: Commands,
    mut root: Query<(Entity,&Root)>,
    queried: Query<&Queried>
){
    if queried.iter().len() == 0{
        println!("No queried object found. Falling back to root.");
        cmd.entity(root.single_mut().0).insert(Queried{});
    }
}

fn devcam_look(
    mut tf: Query<(&Transform, &Camera3d, Entity)>,
    cum: Res<AccumulatedMouseMotion>,
    kb: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands
){

    let t = tf.single().0;

    let mut pos = t.translation;
    let rot =  t.rotation.to_euler(EulerRot::YXZ);

    let mult = 1000.0;

    if kb.pressed(KeyCode::KeyW){
        pos += t.forward() * mult;
    }

    if kb.pressed(KeyCode::KeyS){
        pos += t.back() * mult;
    }

    if kb.pressed(KeyCode::KeyA){
        pos += t.left() * mult;
    }

    if kb.pressed(KeyCode::KeyD){
        pos += t.right() * mult;
    }
    
    cmd.entity(tf.single_mut().2).insert(Transform{
        translation: pos,
        rotation: Quat::from_euler(EulerRot::YXZ, 
            rot.0-cum.delta.x*0.01, 
            (rot.1-cum.delta.y*0.01).clamp(
                -(FRAC_PI_2 - 0.01),
                FRAC_PI_2 - 0.01),
            rot.2),
        ..default()
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let config: CascadeShadowConfig = CascadeShadowConfigBuilder {
        maximum_distance: 1000000.0,
        first_cascade_far_bound: 10000.0,
        num_cascades: 4,
        ..default()
    }.into();
    
    let cam = commands.spawn( (
        Camera3d{..default()},
        Msaa::Off,        
        ShadowFilteringMethod::Temporal,
        //TemporalAntiAliasing{reset:false},
        DepthPrepass,
        MotionVectorPrepass,
        DeferredPrepass,
    )).id();
    let sun = commands.spawn((

        DirectionalLight{
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 1.8,
            illuminance: 25_000.0,
            shadows_enabled: true,
            soft_shadow_size: Some(10.0),
            color: Color::srgb(1.0, 0.0, 0.3125) },
        Transform{
            rotation: Quat::from_euler(
                EulerRot::XYZ, 
                -45.0, 70.0, 0.0),
            ..default()
        },config)).id();

    let root = commands.spawn_empty().id();
    commands.entity(root).insert(
    (GameObject{
                ent: root,
                name: "root".to_string(), 
                id: 0,
                code: "".to_string()
            },Queried{},Root{})
        );

    commands.insert_resource(AmbientLight{
        color: Color::srgb(0.2, 0.2, 0.33), brightness: 1000.0
    });

    commands.insert_resource(ClearColor(
        Color::srgb(
            0.1,
            0.33,
            0.6
        )
    ));

    commands.spawn(
        (        
            BackgroundColor(
                Color::srgb(0.33, 0.33, 0.33)
            ),

            Node{
                right: Val::Px(0.0),
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                position_type: PositionType::Absolute,
                ..default()
            },

            Button{},
    )).observe(|_t: Trigger<Pointer<Down>>,mut cmd: Commands,mut queried:Query<(Entity,&Queried)>
        | {
        let new = cmd.spawn_empty().id(); 
        cmd.entity(new).insert(GameObject{
            ent: new,
            name: "YIPPEE".to_string(),
            id:rand::random::<u32>(),
            code:"".to_string()
        });
        cmd.entity(queried.single_mut().0).add_child(new);
    });

    commands.spawn(
        (        
            BackgroundColor(
                Color::srgb(0.33, 0.33, 0.33)
            ),

            Node{
                right: Val::Px(0.0),
                top: Val::Px(16.0),
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                position_type: PositionType::Absolute,
                ..default()
            },

            Button{},
    )).observe(move|
        _t: Trigger<Pointer<Down>>,
        queried:Query<(Entity,&Queried)>,
        showing:Query<(Entity,&Showing)>,
        mut cmd: Commands,
        | {
            for q in queried.iter(){
                cmd.entity(q.0).remove::<Queried>();
            }for q in showing.iter(){
                cmd.entity(q.0).remove::<Showing>();
            }
    });

    
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
            InspectorList{},
            ScrollLerp{x:0.0,y:0.0}
        ));
    });

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
            if let Ok(_scroll_position) = scrolled_node_query.get_mut(*entity) {
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

    if hovering{
        if (dy>0.0 && bla.y<0.0) || (dy<0.0 && bla.y>0.0) {
            bla.y = 0.0;
        } else {
            bla.y += dy;
        }
        if (dx>0.0 && bla.x<0.0) || (dx<0.0 && bla.x>0.0) {
            bla.x = 0.0;
        } else {
            bla.x += dx;
        }
    }

    if let Ok(mut scroll_position) = scrolled_node_query.get_mut(scr.single_mut().0) {
        scroll_position.offset_x -= bla.x;
        scroll_position.offset_y -= bla.y;
        
    }
        
    bla.x = bla.x.lerp(0.0,0.01);
    bla.y = bla.y.lerp(0.0,0.01);

    cmd.entity(scr.single_mut().0).insert(bla);
    
}


