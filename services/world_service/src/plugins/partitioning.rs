// Copyright (C) 2025 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use bevy::{app::{Last, Plugin}, ecs::{component::Component, entity::Entity, lifecycle::HookContext, query::{Added, Changed}, resource::Resource, system::{Commands, Query, ResMut}}, math::{Vec3, bounding::Aabb3d}};
use spart::{geometry::{Cube, EuclideanDistance, Point3D}, octree::Octree};

use crate::plugins::Movement;

pub struct PartitioningPlugin;

impl Plugin for PartitioningPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Last, (
            add_tree_positions,
            update_tree_positions,
        ));

        app
            .world_mut()
            .register_component_hooks::<WorldSpaceNode>()
            .on_remove(
                    |mut world,
                        HookContext {
                            entity,
                            ..
                        }| {
                    
                    if 
                        let Some(node) = world.get::<WorldSpaceNode>(entity).cloned() &&
                        let Some(mut space) = world.get_resource_mut::<WorldSpace>()
                    {
                        space.tree.delete(&node.0);
                    }
                },
            );
    }
}

#[derive(Resource)]
pub struct WorldSpace {
    tree: Octree<Entity>,
}

impl WorldSpace {
    pub fn new(bounds: Aabb3d) -> Self {
        let tree = Octree::new(
            &Cube {
                x: bounds.min.x.into(),
                y: bounds.min.y.into(),
                z: bounds.min.z.into(),
                width: (bounds.max.x - bounds.min.x).into(),
                height: (bounds.max.y - bounds.min.y).into(),
                depth: (bounds.max.z - bounds.min.z).into(),
            },
            4, // max depth
        ).unwrap();

        WorldSpace { tree }
    }

    pub fn find_in_range(
        &self,
        point: Vec3,
        radius: f32,
    ) -> Vec<Entity> {
        self.tree
            .range_search::<EuclideanDistance>(&Point3D { 
                x: point.x.into(),
                y: point.y.into(),
                z: point.z.into(),
                data: None,
            }, 
            radius.into())
            .into_iter()
            .filter_map(|p| p.data)
            .collect()
    }
}

#[derive(Component, Clone)]
pub struct WorldSpaceNode(Point3D<Entity>);

fn add_tree_positions(
    mut space: ResMut<WorldSpace>,
    added: Query<(Entity, &Movement), Added<Movement>>,
    mut commands: Commands,
) {
    for (entity, movement) in added.iter() {
        let point = Point3D::new(
            movement.position.x.into(),
            movement.position.y.into(),
            movement.position.z.into(),
            Some(entity),
        );

        space.tree.insert(point.clone());

        commands
            .entity(entity)
            .insert(WorldSpaceNode(point));
    }
}

fn update_tree_positions(
    mut space: ResMut<WorldSpace>,
    mut updated: Query<(&Movement, &mut WorldSpaceNode), Changed<Movement>>,
) {
    for (movement, mut node) in updated.iter_mut() {
        space.tree.delete(&node.0);

        node.0.x = movement.position.x.into();
        node.0.y = movement.position.y.into();
        node.0.z = movement.position.z.into();

        space.tree.insert(node.0.clone());
    }
}