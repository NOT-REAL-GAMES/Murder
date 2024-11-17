use bevy::{
    ecs::system::EntityCommands,
    prelude::{BuildChildren, Bundle, ChildBuilder, Commands},
};

// Prefab section
pub trait Prefab {
    fn insert_into<'b, 'w, 's, 'a>(
        self,
        commands: &'b mut EntityCommands<'w, 's, 'a>,
    ) -> &'b mut EntityCommands<'w, 's, 'a>;
}

impl<T: Bundle> Prefab for T {
    fn insert_into<'b, 'w, 's, 'a>(
        self,
        commands: &'b mut EntityCommands<'w, 's, 'a>,
    ) -> &'b mut EntityCommands<'w, 's, 'a> {
        commands.insert(self)
    }
}

// Parent section
pub struct Parent<B: Bundle, C: Children>(B, C);

impl<B: Bundle, C: Children> Prefab for Parent<B, C> {
    fn insert_into<'b, 'w, 's, 'a>(
        self,
        commands: &'b mut EntityCommands<'w, 's, 'a>,
    ) -> &'b mut EntityCommands<'w, 's, 'a> {
        commands.insert(self.0).with_children(|parent| {
            self.1.attach_to(parent);
        })
    }
}

// Children section
pub trait Children: Sized {
    fn attach_to(self, parent: &mut ChildBuilder);
}

/// C is the previous children, P is the next child.
/// It is in this order because adding a sibling has to wrap the previous children.
pub struct Sibling<C: Children, P: Prefab>(C, P);

impl<T: Prefab> Children for T {
    fn attach_to(self, parent: &mut ChildBuilder) {
        parent.spawn_prefab(self);
    }
}

impl<C: Children, P: Prefab> Children for Sibling<C, P> {
    fn attach_to(self, parent: &mut ChildBuilder) {
        self.0.attach_to(parent);
        parent.spawn_prefab(self.1);
    }
}

// Extension traits
pub trait AddChild: Sized {
    type Output<P: Prefab>: Prefab;

    fn add_child<P: Prefab>(self, child: P) -> Self::Output<P>;
}

impl<T: Bundle> AddChild for T {
    type Output<P: Prefab> = Parent<Self, P>;

    fn add_child<P: Prefab>(self, child: P) -> Self::Output<P> {
        Parent(self, child)
    }
}

impl<P: Bundle, C: Children> AddChild for Parent<P, C> {
    type Output<P2: Prefab> = Parent<P, Sibling<C, P2>>;

    fn add_child<P2: Prefab>(self, child: P2) -> Self::Output<P2> {
        Parent(self.0, Sibling(self.1, child))
    }
}

pub trait EntityCommandsExt {
    fn insert_prefab<P: Prefab>(&mut self, prefab: P) -> &mut Self;
}

impl<'w, 's, 'a> EntityCommandsExt for EntityCommands<'w, 's, 'a> {
    fn insert_prefab<P: Prefab>(&mut self, prefab: P) -> &mut Self {
        prefab.insert_into(self)
    }
}

pub trait CommandsExt<'w, 's> {
    fn spawn_prefab<'a, P: Prefab>(&'a mut self, prefab: P) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> CommandsExt<'w, 's> for Commands<'w, 's> {
    fn spawn_prefab<'a, P: Prefab>(&'a mut self, prefab: P) -> EntityCommands<'w, 's, 'a> {
        let mut out = self.spawn_empty();
        out.insert_prefab(prefab);
        out
    }
}

impl<'w, 's, 'b> CommandsExt<'w, 's> for ChildBuilder<'w, 's, 'b> {
    fn spawn_prefab<'a, P: Prefab>(&'a mut self, prefab: P) -> EntityCommands<'w, 's, 'a> {
        let mut out = self.spawn_empty();
        out.insert_prefab(prefab);
        out
    }
}
