use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::Material2d;
use bevy::sprite::Material2dPlugin;
use bevy::sprite::MaterialMesh2dBundle;
use pyri_state::prelude::*;

use crate::core::UpdateSet;
use crate::game::audio::music::on_full_beat;
use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Ground>();
}

const GROUND_Z_INDEX: f32 = -10.0;
const GROUND_MESH_SIZE: f32 = 1024.0;
const GROUND_SNAP_INTERVAL: f32 = GROUND_MESH_SIZE / 4.0;
const GROUND_SNAP: Vec2 = Vec2::splat(GROUND_SNAP_INTERVAL);

#[derive(Resource)]
pub struct Ground {
    material: Handle<GroundMaterial>,
    mesh: Handle<Mesh>,
}

impl Configure for Ground {
    fn configure(app: &mut App) {
        app.add_plugins(Material2dPlugin::<GroundMaterial>::default());
        app.init_resource::<Self>();

        app.add_systems(
            Update,
            Screen::Playing.on_update((
                update_background,
                update_background_beat
                    .in_set(UpdateSet::Update)
                    .run_if(on_full_beat(8)),
            )),
        );
    }
}

impl FromWorld for Ground {
    fn from_world(world: &mut World) -> Self {
        let material = world
            .resource_mut::<Assets<GroundMaterial>>()
            .add(GroundMaterial::default());
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Rectangle::default());

        Self { material, mesh }
    }
}

#[derive(Default, AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct GroundMaterial {
    #[uniform(100)]
    uniforms: Uniforms,
}

#[derive(ShaderType, Default, Clone, Debug)]
struct Uniforms {
    pub camera_x: f32,
    pub camera_y: f32,
    pub random: f32,
    pub time: f32,
}

impl Material2d for GroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ground.wgsl".into()
    }
}

pub fn ground(mut entity: EntityWorldMut) {
    let ground = r!(entity.world().get_resource::<Ground>());
    let material = ground.material.clone();
    let mesh = ground.mesh.clone();

    entity.insert((
        Name::new("Background"),
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            transform: Transform::from_translation(Vec2::ZERO.extend(GROUND_Z_INDEX))
                .with_scale(Vec3::splat(GROUND_MESH_SIZE)),
            material,
            ..default()
        },
    ));
}

fn update_background(
    mut ground_material: ResMut<Assets<GroundMaterial>>,
    camera_query: Query<&Transform, (With<IsDefaultUiCamera>, Without<Handle<GroundMaterial>>)>,
    mut ground_query: Query<&mut Transform, With<Handle<GroundMaterial>>>,
    time: Res<Time>,
) {
    for (_, material) in ground_material.iter_mut() {
        for mut ground_transform in &mut ground_query {
            for camera_transform in &camera_query {
                let translation = ((camera_transform.translation.truncate() / GROUND_SNAP).floor()
                    * GROUND_SNAP)
                    .extend(GROUND_Z_INDEX);
                ground_transform.translation = translation;
                material.uniforms.camera_x = translation.x;
                material.uniforms.camera_y = translation.y;
                material.uniforms.time = time.elapsed_seconds();
            }
        }
    }
}

fn update_background_beat(mut ground_material: ResMut<Assets<GroundMaterial>>) {
    for (_, material) in ground_material.iter_mut() {
        material.uniforms.random = rand::random();
    }
}
