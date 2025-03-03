//! Murder: an editor for Bevy made in Bevy
#![feature(duration_millis_float)]

use std::f32::consts::FRAC_PI_2;
use std::time::Instant;
use bevy::ecs::world::World;

use bevy::ui::widget::TextNodeFlags;
use bevy::ui::ContentSize;
use crate::common_traits::VectorSpace;
use bevy::window::PresentMode;
use bevy::math::*;
use bevy::{
    asset::AssetLoader, 
    core_pipeline::{
        prepass::*,
        experimental::taa::*}, 
    prelude::*, 
    window::{PrimaryWindow, WindowResolution}};

use bevy::{
    input::mouse::{MouseWheel, AccumulatedMouseMotion},
    picking::hover::HoverMap,
    pbr::*,
};

mod editor;
mod components;

use crate::editor::*;
use crate::components::*;

#[derive(Component)]
struct TextScaleMult{
    mult: Val
}

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
struct Selected{

}

#[derive(Resource)]
pub struct time{
    start: Instant
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        
        .set(        
            WindowPlugin{
            primary_window: Some(Window{resolution:
                WindowResolution::new(1280.0,720.0),
            ..default()}),
        ..default()
    }
    )).add_plugins((
        TemporalAntiAliasPlugin,
        //MeshletPlugin{cluster_buffer_slots: 1048576},
    ))  
        // RESOURCES
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
        .insert_resource(time{start:Instant::now()})
        // SYSTEMS
        .add_systems(PreStartup, setup)        
        .add_systems(Startup, set_window_title)
        .add_systems(PostStartup, get_all_components)
        
        .add_systems(PreUpdate, fallback_to_root)
        .add_systems(PreUpdate, devcam_look)
        .add_systems(Update, update_scroll_position)
        .add_systems(Update, update_inspector_list)
        .add_systems(PostUpdate, scale_ui_text)

        .add_systems(PostUpdate, color_selected)
        
        .add_systems(Last, delete_objects)
        ;

    app.run();
}

fn get_all_components(
    w: &mut World
){
    for c in w.components().iter() {
        let bla = c.name();
        if bla.contains("murder::components::"){
            println!("{bla}");
        }
    }
}


//TODO: add editable component that title is read from
fn set_window_title(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.single_mut() {
        window.title = "Murder".to_string();
        window.resolution = WindowResolution::new(1280.0, 720.0);
        window.present_mode = PresentMode::Mailbox;

    } 
}

fn delete_objects(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    s: Query<(&InspectorListing, &Selected)>,

) {
    if kb.pressed(KeyCode::Delete){
        for sel in s.iter(){
            commands.entity(sel.0.obj.ent).despawn();
        }
    }
}

fn update_inspector_list(
    mut commands: Commands,
    ass: Res<AssetServer>,
    g: Query<&GameObject, Without<Showing>>,
    i: Query<(Entity, &InspectorListing)>,
    il: Query<(Entity, &InspectorList)>,
    ch: Query<(&Children, &GameObject),With<Queried>>,
    mut w: Query<&mut Window>,

    bla: Res<time>,
    mut ui: ResMut<UiScale>

){
 
    for l in i.iter(){
        let mut found = false;
        let mut child_of_queried = false;
        
        if Some(g.get(l.1.obj.ent)).is_some(){
            found = true;
        }

        if ch.iter().len() == 1{
            for qc in ch.single().unwrap().0.iter(){
                if l.1.obj.ent == qc{
                    child_of_queried = true;
                }
            }
        }

        if !found{
            println!("Listing found for GameObject that doesn't exist. Deleting.");
            commands.entity(l.0).despawn();
        }
        if !child_of_queried{
            //is queried?
                println!("Listing found for child GameObject that isn't queried. Deleting.");
                commands.entity(l.0).despawn();    
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

            
            for qc in ch.single().unwrap().0.iter(){
               if o.ent == qc{
                    child_of_queried = true;
                }
            }
        }

        if !found && child_of_queried {
            println!("GameObject found without corresponding listing. Adding.");
            add_button(&mut commands, ass.clone(), o,il.single().unwrap().0, w.single_mut().unwrap());
            commands.entity(o.ent).insert(Showing{});
        }
    }

    //necessary hack to make ui text render without having to resize the window :(
    ui.0 = 1.0 + (bevy::math::ops::sin( bla.start.elapsed().as_millis_f32() / 10000.0) / 10000.0) ;
}

fn add_button(
    commands: &mut Commands,
    ass: AssetServer,
    o: &GameObject,
    il: Entity,
    mut w: Mut<'_, bevy::prelude::Window>
){

    let fuck = Val::resolve(
        Val::VMin(2.0),
        1.0,
        w.size()).unwrap();

        println!("{fuck}");

    let bla = commands.spawn((
        Node{
            min_width: Val::Percent(100.0),
            min_height: Val::VMin(2.5),
            max_height: Val::VMin(2.5),
            overflow: Overflow::clip(),
            border: UiRect::all(Val::Px(1.0)),
            ..default()},
        BackgroundColor(
            Color::srgb(0.4, 0.4, 0.4)
        ),
        Pickable{
            should_block_lower: false,
            is_hoverable: true},
        Button{},
        GlobalZIndex(1),

        InspectorListing{obj:o.clone()},
    )).with_children(|parent|{
        parent.spawn((
            Node{
                width:Val::Percent(100.0),
                height:Val::Percent(100.0),
                ..default()},
            TextNodeFlags{needs_recompute:true, needs_measure_fn: true},
            Text::new(o.name.clone().as_str()),
            TextFont{
                font: ass.load("Roboto.ttf"),
                font_size: 999.0,
                ..default()
            },
            TextScaleMult{
                mult: Val::VMin(2.0)
            },
            Visibility::Visible,
            GlobalZIndex(2),


        ));
    }).observe(move|
        t: Trigger<Pointer<Pressed>>,
        kb: Res<ButtonInput<KeyCode>>,
        s: Query<(Entity, &Selected)>,
        q: Query<(Entity, &Queried)>,
        il: Query<&InspectorListing>,
        mut cmd:Commands|{

        let mut add = true;
        
        if t.event().button == PointerButton::Primary{
            if s.iter().len() == 1{
                if kb.pressed(KeyCode::ControlLeft){
                    
                }
                else if s.single().unwrap().0 == t.target(){
                    for sel in s.iter(){
                        cmd.entity(sel.0).remove::<Selected>();
                    }
                    for qr in q.iter(){
                        cmd.entity(qr.0).remove::<Queried>();
                    }
                    cmd.entity(il.get(t.target()).unwrap().obj.ent).insert(Queried{});
                }
            }
            if !kb.pressed(KeyCode::ControlLeft){
                for sel in s.iter(){
                    cmd.entity(sel.0).remove::<Selected>();
                }
            }
            else {
                if let Ok(sel) = s.get(t.target()) {
                    cmd.entity(sel.0).remove::<Selected>();
                    add = false;
                }
            }

            if add {
                cmd.entity(t.target()).insert(Selected{});
            }
        }
    }).id();
    commands.entity(il).add_children(&[bla]);

}

fn scale_ui_text(
    mut cmd: Commands,
    mut q: Query<(&mut TextFont,&Transform,&mut TextScaleMult,Entity)>,
    w: Query<&Window>
){
    if w.iter().len() == 0{
        return; //stupid error prevention
    }

    let res = w.single().unwrap().size();

    for mut t in q.iter_mut(){

        let fuck = Val::resolve(t.2.mult, 1.0, res).unwrap();

        t.0.font_size = fuck;

        cmd.entity(t.3).insert(TextNodeFlags{needs_recompute:true, needs_measure_fn: true});
    }

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
        cmd.entity(root.single_mut().unwrap().0).insert(Queried{});
    }
}

fn devcam_look(
    mut tf: Query<(&Transform, &Camera3d, Entity)>,
    cum: Res<AccumulatedMouseMotion>,
    kb: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands
){

    let t = tf.single().unwrap().0;

    let mut pos = t.translation;
    let rot =  t.rotation.to_euler(EulerRot::YXZ);

    let mult = 1.0;

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
    
    cmd.entity(tf.single_mut().unwrap().2).insert(Transform{
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
    
    commands.spawn((Text("what the fuck".to_string()),TextScaleMult{mult:Val::VMin(10.0)}));

    commands.spawn((
        Camera2d::default(),
        Camera{
            order: 10,
            clear_color:ClearColorConfig::None,
            
            ..default()},Msaa::Off));

    let cam = commands.spawn( (
        Camera3d{..default()},
        Camera{order: 1,..default()},
        Transform{translation:Vec3 { x: 0.0, y: 1000.0, z: 0.0 },..default()},
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
            color: Color::srgb(1.0, 0.9, 0.7),
            ..default()
        },
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
        color: Color::srgb(0.2, 0.2, 0.33), brightness: 1000.0,..default()
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
                width: Val::VMin(2.0),
                height: Val::VMin(2.0),
                position_type: PositionType::Absolute,
                ..default()
            },

            Button{},
    )).observe(|t: Trigger<Pointer<Pressed>>,mut cmd: Commands,mut queried:Query<(Entity,&Queried)>
        | {
            if t.button == PointerButton::Primary{
                let new = cmd.spawn_empty().id(); 
                cmd.entity(new).insert((GameObject{
                    ent: new,
                    name: "YIPPEE".to_string(),
                    id:rand::random::<u32>(),
                    code:"".to_string()
                }));

                cmd.entity(new).insert(TestComponent{});
                
                cmd.entity(queried.single_mut().unwrap().0).add_child(new);
            }
    });

    commands.spawn(
        (        
            BackgroundColor(
                Color::srgb(0.33, 0.33, 0.33)
            ),

            Node{
                right: Val::Px(0.0),
                top: Val::VMin(2.0),
                width: Val::VMin(2.0),
                height: Val::VMin(2.0),
                position_type: PositionType::Absolute,
                ..default()
            },

            Button{},
    )).observe(move|
        t: Trigger<Pointer<Pressed>>,
        queried:Query<(Entity,&Queried)>,
        showing:Query<(Entity,&Showing)>,
        mut cmd: Commands,
        | {
            if t.button == PointerButton::Primary {
                for q in queried.iter(){
                    cmd.entity(q.0).remove::<Queried>();
                }for q in showing.iter(){
                    cmd.entity(q.0).remove::<Showing>();
                }
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
            Pickable{
                should_block_lower: false,
                is_hoverable: true
            },
            GlobalZIndex(99999),
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
        (dx,dy) = {
            (mouse_wheel_event.x, mouse_wheel_event.y)
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

    if let Ok(mut scroll_position) = scrolled_node_query.get_mut(scr.single_mut().unwrap().0) {
        scroll_position.offset_x -= bla.x;
        scroll_position.offset_y -= bla.y;
        
    }
        
    bla.x = VectorSpace::lerp(bla.x,0.0,0.01);
    bla.y = VectorSpace::lerp(bla.y,0.0,0.01);

    cmd.entity(scr.single_mut().unwrap().0).insert(bla);
    
}


