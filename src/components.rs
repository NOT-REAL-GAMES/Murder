use bevy::ecs::component::HookContext;
use bevy::prelude::*;
use bevy::ecs::*;
use crate::components::world::DeferredWorld;


#[derive(Component)]
#[component(on_add = test)]
pub struct TestComponent{
    
}

fn test(mut world: DeferredWorld, ctx: HookContext) {
    
        unsafe {        
            let w = world.world.world_mut();


            let mut mesh = w.get_resource_or_init::<Assets<Mesh>>();
            let bla = mesh.add(Cuboid{half_size:Vec3 { x: 5000000.0, y: 1.0, z: 5000000.0 }});

            let mut mat = w.get_resource_or_init::<Assets<StandardMaterial>>();
            let bla2 = mat.add(StandardMaterial::default());

            w.commands().entity(ctx.entity).insert((Mesh3d(bla),MeshMaterial3d(bla2),Transform{translation: Vec3 { x: 0.0, y: -100.0, z: 0.0 },..default()},Visibility::Visible));
            
            w.flush();
        }



}