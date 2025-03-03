use bevy::prelude::*;

//TODO: Make more use of this file :þ


#[derive(Component)]
pub struct Queried{}

#[derive(Component)]
pub struct AddTo{}


#[derive(Component)]
pub struct Showing{}

#[derive(Component)]
pub struct Root{}

#[derive(Component, Clone, Copy)]
pub struct ScrollLerp{
    pub x: f32,
    pub y: f32
}//

#[derive(Component)]
pub struct InspectorList{

}