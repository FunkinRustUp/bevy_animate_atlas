<img align="right" width="300" src="https://raw.githubusercontent.com/FunkinRustUp/bevy_animate_atlas/refs/heads/main/logo.gif" alt="Logo" />

<h1 align="center">Bevy Animate Atlas</h1>

<br/>

Adobe Animate **sparrow V2** and **texture atlas** parser and renderer for Bevy.

Technically it's just a port of the **MacroAnimate** crate.

## Features

- **Minimalist Sparrow Parser**: Fast, dependency-free texture layout parsing powered by lightweight string scanning.
- **Dynamic Rigging Pipeline**: Real-time hierarchical evaluation for multi-layered and nested bone transformations.
- **Procedural Vertex Generation**: Converts complex matrix deformations into native Bevy mesh components on the fly.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.18.1"
bevy_animate_atlas = "0.1.0"
```

## Quick Start

### 1. Rendering Sparrow Atlases

```rs
use bevy::prelude::*;
use macroanimate::{parse_sparrow, SparrowFrame};

// A simple component to track animation state
#[derive(Component)]
struct SpriteAnimation {
    frames: Vec<SparrowFrame>,
    current_frame: usize,
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sparrow)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("assets/character.png");
    let xml_data = std::fs::read_to_string("assets/character.xml").unwrap();

    // Parse frames with a matching prefix
    let frames = parse_sparrow(&xml_data, "idle");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        SpriteAnimation {
            frames,
            current_frame: 0,
            timer: Timer::from_seconds(1.0 / 24.0, TimerMode::Repeating),
        },
    ));
}

fn animate_sparrow(time: Res<Time>, mut query: Query<(&mut Sprite, &mut Transform, &mut SpriteAnimation)>) {
    for (mut sprite, mut transform, mut anim) in query.iter_mut() {
        anim.timer.tick(time.delta());
        
        if anim.timer.just_finished() {
            if let Some(f) = anim.frames.get(anim.current_frame) {
                // Update the visible source rectangle on the texture sheet
                sprite.rect = Some(Rect::new(f.x, f.y, f.x + f.width, f.y + f.height));
                
                // Adjust position based on the Sparrow frame offset (Inverting Y for Bevy)
                transform.translation = Vec3::new(100.0 - f.frame_x, -(150.0 - f.frame_y), 0.0);
            }
            
            anim.current_frame = (anim.current_frame + 1) % anim.frames.len();
        }
    }
}
```

### 2. Rendering Animate Atlases

```rs
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use macroanimate::{parse_texture_atlas, get_texture_parts, build_part_mesh};

fn animate_atlas_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    // Assume you track your `frame_index` and `atlas` metadata in resources or components
) {
    let texture = asset_server.load("assets/test/spritemap1.png");
    let material = materials.add(ColorMaterial::from(texture));

    let spritemap = std::fs::read_to_string("assets/test/spritemap1.json").unwrap();
    let animation = std::fs::read_to_string("assets/test/Animation.json").unwrap();
    let atlas = parse_texture_atlas(&spritemap, &animation);
    
    let frame_index = 0;

    // Resolve matrix transformations for the current frame
    let parts = get_texture_parts(&atlas, "idle", frame_index);

    // Parent container for the rig
    let parent = commands.spawn(SpatialBundle::default()).id();

    // Iterate and build vertex meshes using the sheet metadata configurations
    for (z_index, part) in parts.iter().enumerate() {
        if let Some(sprite) = atlas.sprites.get(&part.sprite_name) {
            
            // Generate a procedural Bevy mesh using the matrix layout
            let mesh_data = build_part_mesh(
                sprite,
                &part.matrix,
                2044.0,        // Texture sheet width
                1923.0,        // Texture sheet height
                750.0,         // Target layout X offset
                250.0,         // Target layout Y offset
                z_index as f32 // Layer sorting order
            );

            let mesh_handle = meshes.add(mesh_data);

            // Spawn the limb as a child layer
            let child = commands.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh_handle),
                material: material.clone(),
                ..default()
            }).id();

            commands.entity(parent).add_child(child);
        }
    }
}
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Additional Info

This crate is mainly built for the FNF RustUp Engine, any features that will be supported will go through if the RustUp Engine needs any support for it.
