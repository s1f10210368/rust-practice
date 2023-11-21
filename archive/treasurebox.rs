use bevy::{ prelude::*, render::*, render::settings::* };
use std::f32::consts::*;

fn main()
{   //Note：手元の環境だとVulkanのままでは影が描画されなかったので、DX12へ切り替えた。
    let backends = Some ( Backends::DX12 );
    let wgpu_settings = WgpuSettings { backends, ..default() };
    let backend_dx12 = RenderPlugin { wgpu_settings };

    App::new()
        .add_plugins
        (   DefaultPlugins //各種の面倒を見てもらう
                /*.set( backend_dx12 )*/ //Note：この行をコメントアウトするとデフォルトになる
        )
        .add_systems
        (   Startup, 
            (   spawn_camera3d_and_light, //カメラを作る
                spawn_3d_lockedchest,     //3Dオブジェクトを作る
            )
        )
        .add_systems
        (   Update,
            (   bevy::window::close_on_esc, //[ESC]キーで終了
            )
        )
        .run();
}

//カメラと光源を作る
fn spawn_camera3d_and_light( mut cmds: Commands )
{   //3Dカメラ
    cmds.spawn( Camera3dBundle::default() )
        .insert
        (   Transform::from_translation( Vec3::new( -1.0, 1.0, 2.0 ) ) //カメラの位置
                .looking_at( Vec3::ZERO, Vec3::Y ) //カメラレンズの向き
        );

    //光源
    cmds.spawn( DirectionalLightBundle::default() )
        .insert
        (   DirectionalLight
            {   illuminance: 15000.0,  //明るさ
                shadows_enabled: true, //影の描画を有効化
                ..default()
            }
        )
        .insert
        (   Transform::from_xyz( 30.0, 100.0, 40.0 ) //光源の位置
                .looking_at( Vec3::ZERO, Vec3::Z )   //光源の向き
        );
}

//3Dオブジェクトを作る
fn spawn_3d_lockedchest
(   mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   //地面
    cmds.spawn( PbrBundle::default() )
        .insert( meshes.add( shape::Plane::from_size( 2.0 ).into() ) )
        .insert( Transform::from_translation( Vec3::ZERO ) )
        .insert( materials.add( Color::rgb( 0.5, 0.7, 0.3 ).into() ) );

    //宝箱
    cmds.spawn( PbrBundle::default() )
        .insert( materials.add( Color::NONE.into() ) ) //透明
        .insert( Transform::from_translation( Vec3::new( 0.0, 0.5, 0.0 ) ) )
        .with_children
        (   | cmds |
            {   //本体
                let shape_box = shape::Box::new( 0.7, 0.3, 0.4 );
                cmds.spawn( PbrBundle::default() )
                    .insert( meshes.add( shape_box.into() ) )
                    .insert( Transform::from_translation( Vec3::Y * -0.35 ) )
                    .insert( materials.add( Color::MAROON.into() ) );

                //上蓋
                let shape_cylinder = shape::Cylinder { height: 0.695, radius: 0.195, ..default() };
                cmds.spawn( PbrBundle::default() )
                    .insert( meshes.add( shape_cylinder.into() ) )
                    .insert
                    (   Transform::from_translation( Vec3::Y * -0.2 )
                            .with_rotation( Quat::from_rotation_z( PI * 0.5 ) )
                    )
                    .insert( materials.add( Color::MAROON.into() ) );

                //錠前
                let shape_cube = shape::Cube::new( 0.1 );
                cmds.spawn( PbrBundle::default() )
                    .insert( meshes.add( shape_cube.into() ) )
                    .insert( Transform::from_translation( Vec3::Y * -0.2 + Vec3::Z * 0.17 ) )
                    .insert( materials.add( Color::GRAY.into() ) )
                    .with_children
                    (   | cmds |
                        {   //鍵穴
                            let cylinder = shape::Cylinder { height: 0.11, radius: 0.01, ..default() };
                            cmds.spawn( PbrBundle::default() )
                                .insert( meshes.add( cylinder.into() ) )
                                .insert
                                (   Transform::from_translation( Vec3::Y * 0.02 )
                                        .with_rotation( Quat::from_rotation_x( PI * 0.5 ) )
                                )
                                .insert( materials.add( Color::BLACK.into() ) );

                            let shape_box = shape::Box::new( 0.01, 0.04, 0.11 );
                            cmds.spawn( PbrBundle::default() )
                                .insert( meshes.add( shape_box.into() ) )
                                .insert( Transform::from_translation( Vec3::Y * 0.0 ) )
                                .insert( materials.add( Color::BLACK.into() ) );
                        }
                    );
            }
        );
}