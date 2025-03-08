use bevy::ecs::component::HookContext;
use bevy::prelude::*;
use bevy::ecs::*;
use rand::RngCore;
use crate::components::world::DeferredWorld;
use crate::*;

use rand::rngs::ThreadRng;
use rand::{thread_rng,Rng};

#[derive(Component)]
#[component(on_insert = test)]
pub struct TestComponent{
    
}

fn test(mut world: DeferredWorld, ctx: HookContext) {
    
        unsafe {        
            let w = world.world.world_mut();


            let mut mesh = w.get_resource_or_init::<Assets<Mesh>>();
            let bla = mesh.add(Cuboid{half_size:Vec3 { x: 0.25, y: 0.25, z: 0.25 }});

            let mut mat = w.get_resource_or_init::<Assets<StandardMaterial>>();
            let bla2 = mat.add(StandardMaterial::default());

            let c = w.get_resource::<SampledShape>().unwrap();
            
            let r = c.0.sample_boundary(&mut thread_rng());

            w.commands().entity(ctx.entity).insert((
                Mesh3d(bla),
                MeshMaterial3d(bla2),
                NeedsUpdating,
                Transform{translation:r,..default()},
                Visibility::Visible)
            );
            
            w.flush();
        }



}