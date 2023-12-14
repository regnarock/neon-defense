use crate::inventory::SpawnInventory;
use crate::inventory::{self};
use crate::random::RandomDeterministic;
use bevy::ecs::system::EntityCommand;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::HashMap;
use rand::seq::SliceRandom;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(inventory::InventoryPlugin::<Building>::default());
        app.add_systems(Startup, (create_assets, spawn_layout).chain());
    }
}

#[derive(Resource)]
pub struct VisualAssets {
    pub mesh_def: HashMap<BuildingMesh, Mesh2dHandle>,
    pub size_def: HashMap<BuildingSize, f32>,
    pub color_def: HashMap<BuildingColor, Handle<ColorMaterial>>,
}

pub(crate) fn create_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(VisualAssets {
        mesh_def: [
            (
                BuildingMesh::Triangle,
                meshes
                    .add(
                        Mesh::new(PrimitiveTopology::TriangleList)
                            .with_inserted_attribute(
                                Mesh::ATTRIBUTE_POSITION,
                                vec![[-0.5, -0.5, 0.0], [0.0, 0.5, 0.0], [0.5, -0.5, 0.0]],
                            )
                            .with_indices(Some(Indices::U32(vec![0, 1, 2]))),
                    )
                    .into(),
            ),
            (
                BuildingMesh::Circle,
                meshes.add(Mesh::from(shape::Circle::default())).into(),
            ),
            (
                BuildingMesh::Quad,
                meshes.add(Mesh::from(shape::Quad::default())).into(),
            ),
        ]
        .into(),
        size_def: [
            (BuildingSize::Big, 1f32),
            (BuildingSize::Medium, 0.75f32),
            (BuildingSize::Small, 0.5f32),
        ]
        .into(),
        color_def: [
            (
                BuildingColor::Black,
                materials.add(ColorMaterial::from(Color::BLACK)),
            ),
            (
                BuildingColor::White,
                materials.add(ColorMaterial::from(Color::WHITE)),
            ),
            (
                BuildingColor::Pink,
                materials.add(ColorMaterial::from(Color::PINK)),
            ),
            (
                BuildingColor::Blue,
                materials.add(ColorMaterial::from(Color::BLUE)),
            ),
        ]
        .into(),
    });
}

const ITEM_VISUAL_SIZE: f32 = 64f32;

pub(crate) fn spawn_layout(mut commands: Commands) {
    let mut rng = crate::random::RandomDeterministic::new_from_seed(0);
    let inventory = vec![
        commands.spawn(get_random_building(&mut rng)).id(),
        commands.spawn(get_random_building(&mut rng)).id(),
        commands.spawn(get_random_building(&mut rng)).id(),
        commands.spawn(get_random_building(&mut rng)).id(),
        commands.spawn(get_random_building(&mut rng)).id(),
        commands.spawn(get_random_building(&mut rng)).id(),
    ];
    commands
        .spawn_empty()
        .add(SpawnInventory::<Building>::new(
            inventory,
            inventory::InventoryConfiguration {
                positions: vec![
                    vec3(-350f32, 0f32, 0f32),
                    vec3(-350f32, ITEM_VISUAL_SIZE + 10f32, 0f32),
                    vec3(-350f32, (ITEM_VISUAL_SIZE + 10f32) * 2f32, 0f32),
                    vec3(-350f32, (ITEM_VISUAL_SIZE + 10f32) * 3f32, 0f32),
                    vec3(-350f32, (ITEM_VISUAL_SIZE + 10f32) * 4f32, 0f32),
                    vec3(-350f32, (ITEM_VISUAL_SIZE + 10f32) * 5f32, 0f32),
                ],
            },
        ))
        .insert(RandomDeterministic::new_from_seed(0));
}

#[derive(Component, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Building {
    mesh: BuildingMesh,
    size: BuildingSize,
    color: BuildingColor,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum BuildingMesh {
    Triangle,
    Circle,
    Quad,
}
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum BuildingSize {
    Small,
    Medium,
    Big,
}
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum BuildingColor {
    Black,
    White,
    Pink,
    Blue,
}

impl inventory::ItemSpriteBuilder for Building {
    type C = BuildingItemSpriteBuilder;
    fn build_sprite(&self) -> Self::C {
        BuildingItemSpriteBuilder { building: *self }
    }
}

pub struct BuildingItemSpriteBuilder {
    pub building: Building,
}

impl EntityCommand for BuildingItemSpriteBuilder {
    fn apply(self, id: Entity, world: &mut World) {
        let assets = world.get_resource::<VisualAssets>().unwrap();
        let visual = MaterialMesh2dBundle {
            mesh: assets.mesh_def[&self.building.mesh].clone(),
            transform: Transform::default().with_scale(Vec3::splat(
                ITEM_VISUAL_SIZE * assets.size_def[&self.building.size],
            )),
            material: assets.color_def[&self.building.color].clone(),
            ..default()
        };
        world.entity_mut(id).insert(visual);
    }
}

pub fn get_random_building(rng: &mut crate::random::RandomDeterministic) -> Building {
    let choices_mesh = [
        (BuildingMesh::Triangle, 2),
        (BuildingMesh::Circle, 2),
        (BuildingMesh::Quad, 2),
    ];
    let choices_size = [
        (BuildingSize::Big, 1),
        (BuildingSize::Medium, 2),
        (BuildingSize::Small, 1),
    ];
    let choices_color = [
        (BuildingColor::Black, 5),
        (BuildingColor::White, 5),
        (BuildingColor::Pink, 1),
        (BuildingColor::Blue, 1),
    ];
    let building = Building {
        mesh: choices_mesh
            .choose_weighted(&mut rng.random, |i| i.1)
            .unwrap()
            .0,
        size: choices_size
            .choose_weighted(&mut rng.random, |i| i.1)
            .unwrap()
            .0,
        color: choices_color
            .choose_weighted(&mut rng.random, |i| i.1)
            .unwrap()
            .0,
    };
    building
}