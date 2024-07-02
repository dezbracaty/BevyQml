// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cxx_qt_module
// ANCHOR: book_bridge_macro
/// The bridge definition for our QObject
use bevy::{
    color::palettes::css::{ORANGE, SILVER, WHITE},
    math::vec3,
    prelude::*,
};


#[derive(Component)]
struct Curve(CubicCurve<Vec3>);

#[cxx_qt::bridge(cxx_file_stem = "rust_cxx_qt_object")]
pub mod qobject {
    // ANCHOR_END: book_bridge_macro

    // ANCHOR: book_qstring_import
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;
    }
    // ANCHOR_END: book_qstring_import

    // ANCHOR: book_rustobj_struct_signature
    unsafe extern "RustQt" {
        // The QObject definition
        // We tell CXX-Qt that we want a QObject class with the name MyObject
        // based on the Rust struct MyObjectRust.
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type MyObject = super::MyObjectRust;
    }
    // ANCHOR_END: book_rustobj_struct_signature

    // ANCHOR: book_rustobj_invokable_signature
    unsafe extern "RustQt" {
        // Declare the invokable methods we want to expose on the QObject
        #[qinvokable]
        fn increment_number(self: Pin<&mut MyObject>);

        #[qinvokable]
        fn say_hi(self: &MyObject, string: &QString, number: i32);
    }
    // ANCHOR_END: book_rustobj_invokable_signature
}

// ANCHOR: book_use
use core::pin::Pin;
use cxx_qt_lib::QString;
// ANCHOR_END: book_use

/// The Rust struct for the QObject
// ANCHOR: book_rustobj_struct
#[derive(Default)]
pub struct MyObjectRust {
    number: i32,
    string: QString,
}
// ANCHOR_END: book_rustobj_struct

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Define your control points
    // These points will define the curve
    // You can learn more about bezier curves here
    // https://en.wikipedia.org/wiki/B%C3%A9zier_curve
    let points = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.),
    ]];

    // Make a CubicCurve
    let bezier = CubicBezier::new(points).to_curve();

    // Spawning a cube to experiment on
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::from(ORANGE)),
            transform: Transform::from_translation(points[0][0]),
            ..default()
        },
        Curve(bezier),
    ));

    // Some light to see something
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50., 50.)),
        material: materials.add(Color::from(SILVER)),
        ..default()
    });

    // The camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 3., 0.), Vec3::Y),
        ..default()
    });
}

fn animate_cube(time: Res<Time>, mut query: Query<(&mut Transform, &Curve)>, mut gizmos: Gizmos) {
    let t = (time.elapsed_seconds().sin() + 1.) / 2.;

    for (mut transform, cubic_curve) in &mut query {
        // Draw the curve
        gizmos.linestrip(cubic_curve.0.iter_positions(50), WHITE);
        // position takes a point from the curve where 0 is the initial point
        // and 1 is the last point
        transform.translation = cubic_curve.0.position(t);
    }
}

// ANCHOR: book_rustobj_invokable_impl
impl qobject::MyObject {
    /// Increment the number Q_PROPERTY
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.number();
        self.set_number(previous + 1);
    }

    /// Print a log message with the given string and number
    pub fn say_hi(&self, string: &QString, number: i32) {
                App::new()
                    .add_plugins(DefaultPlugins)
                    .add_systems(Startup, setup)
                    .add_systems(Update, animate_cube)
                    .run();
            
            
        
        println!("Hi from Rust! String is '{string}' and number is {number}");
    }
    
}
// ANCHOR_END: book_rustobj_invokable_impl

// ANCHOR_END: book_cxx_qt_module
