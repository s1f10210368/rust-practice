use super::*;
use bevy::sprite::Anchor;

//3Dカメラと光源を作る
pub fn camera3d_and_light
(   q_window: Query<&Window>,
    mut cmds: Commands,
)
{   //viewportの設定値(表示エリアの矩形)を作る
    let Ok( window ) = q_window.get_single() else { return };
    let x = ( window.width()  - VIEWPORT_WIDTH  ) as u32 / 2; //表示エリアの左上X座標
    let y = ( window.height() - VIEWPORT_HEIGHT ) as u32 / 2; //表示エリアの左上Y座標
    let physical_position = UVec2 { x, y };
    let physical_size = VIEWPORT_SIZE.as_uvec2();
    let viewport = Some ( Viewport { physical_position, physical_size, ..default() } );

    //3Dカメラ
    let orbit_camera = OrbitCamera::default();
    let vec3 = orbit_camera.orbit.into_vec3();
    cmds.spawn( ( Camera3dBundle::default(), orbit_camera ) )
        .insert( Camera { order: CAMERA3D_ORDER, viewport, ..default() } )
        .insert
        (   Transform::from_translation( vec3 )    //カメラの位置
                .looking_at( Vec3::ZERO, Vec3::Y ) //カメラレンズの向き
        );

    //光源
    let light = DirectionalLight
    {   illuminance: LIGHT_BRIGHTNESS,
        shadows_enabled: true, //影の描画を有効化
        ..default()
    };
    cmds.spawn( DirectionalLightBundle::default() )
        .insert( light )
        .insert
        (   Transform::from_translation( LIGHT_POSITION ) //光源の位置
                .looking_at( Vec3::ZERO, Vec3::Z )        //光源の向き
        );
}

//------------------------------------------------------------------------------

//2Dカメラを作る
pub fn camera2d( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() )
        .insert( Camera { order: CAMERA2D_ORDER, ..default() } )
        .insert( Camera2d { clear_color: CAMERA2D_BGCOLOR } );
}

//UIテキストを作る
pub fn display_board
(   q_window: Query<&Window>,
    mut cmds: Commands,
)
{   let textstyle = TextStyle { font_size: UI_TEXT_FONT_SIZE, ..default() };
    let text = Text::from_section( "", textstyle ); //placeholderのみ

    //Cameraにviewportを設定したらテキスト表示がバグったので(変に拡大して二重表示された)、
    //TextBundleの使用をやめてText2dBundleへ変更した。(機能的にやりたいことは実現できる)
    //Text2dBundleは座標原点がウィンドウ中央になるのでテキストを左上に寄せるるため、
    //transformとtext_anchorを追加設定した。
    let Ok( window ) = q_window.get_single() else { return };
    let translation = Vec3::new( window.width() / -2.0, window.height() / 2.0, 0.0 );
    let transform = Transform::from_translation( translation );
    let text_anchor = Anchor::TopLeft;

    //Text2dBundleを作る
    cmds.spawn( ( Text2dBundle { text, transform, text_anchor, ..default() }, DisplayBoard ) );
}

//------------------------------------------------------------------------------

//3Dオブジェクトを作る(宝箱)
pub fn locked_chest
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