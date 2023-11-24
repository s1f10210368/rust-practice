//external crates
use bevy::
{   prelude::*,
    render::*, render::settings::*,
    core_pipeline::clear_color::*,
};

//standard library
use std::f32::consts::*;

//internal submodules
mod spawn_objs;
mod const_defs;
use const_defs::*;

//------------------------------------------------------------------------------

fn main()
{   //Note：手元の環境だとVulkanのままでは影が描画されなかったので、DX12へ切り替えた。
    let backends = Some ( Backends::DX12 );
    let wgpu_settings = WgpuSettings { backends, ..default() };
    let backend_dx12 = RenderPlugin { wgpu_settings };

    App::new()
        .add_plugins
        (   DefaultPlugins //各種の面倒を見てもらう
                /*.set( backend_dx12 )*/ //Note：この行をコメントアウトするとbackendがデフォルト選択される
        )
        .add_systems
        (   Startup, 
            (   spawn_objs::camera3d_and_light, //3Dカメラとライトを作る
                spawn_objs::locked_chest,       //3Dオブジェクトを作る
                spawn_objs::camera2d,           //2Dカメラを作る(情報表示用)
                spawn_objs::display_board,      //UIテキストを作る(情報表示用)
            )
        )
        .add_systems
        (   Update,
            (   //極座標を更新する
                (   catch_input_from_keyboard, //キー入力による更新
                    // catch_input_from_mouse,
                    // catch_input_from_gamepad,
                )
                .before( move_orbit_camera ), //実行順の制御

                bevy::window::close_on_esc, //[ESC]キーで終了
                move_orbit_camera,          //極座標カメラを動かす
                show_parameter,             //情報を表示する
            )
        )
        .run();
}

//------------------------------------------------------------------------------

//極座標の型
#[derive( Clone, Copy )]
struct Orbit
{   r    : f32, //極座標のr（注目点からカメラまでの距離）
    theta: f32, //極座標のΘ（注目点から見たカメラの垂直角度）
    phi  : f32, //極座標のφ（注目点から見たカメラの水平角度）
}

//極座標から直交座標へ変換するメソッド
impl Orbit
{   fn into_vec3( self ) -> Vec3
    {   let x = self.r * self.theta.sin() * self.phi.sin();
        let y = self.r * self.theta.cos() * -1.0;
        let z = self.r * self.theta.sin() * self.phi.cos();
        Vec3::new( x, y, z )
    }
}

//------------------------------------------------------------------------------

//極座標カメラに付けるComponent
#[derive( Component )]
struct OrbitCamera { orbit: Orbit }

//極座標カメラの初期位置
impl Default for OrbitCamera
{   fn default() -> Self
    {   Self
        {   orbit: Orbit
            {   r    : ORBIT_CAMERA_INIT_R,
                theta: ORBIT_CAMERA_INIT_THETA,
                phi  : ORBIT_CAMERA_INIT_PHI,
            }
        }
    }
}

//UIテキストに付けるComponent
#[derive( Component )]
struct DisplayBoard;

//------------------------------------------------------------------------------

//キー入力によって極座標カメラの位置を更新する
fn catch_input_from_keyboard
(   mut q_camera: Query<&mut OrbitCamera>,
    time: Res<Time>,
    inkey: Res<Input<KeyCode>>,
)
{   let Ok ( mut camera ) = q_camera.get_single_mut() else { return };
    let orbit = &mut camera.orbit;

    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間

    for keycode in inkey.get_pressed()
    {   match keycode
        {   KeyCode::Z =>
                orbit.r = ( orbit.r + time_delta ).min( ORBIT_CAMERA_MAX_R ),
            KeyCode::X =>
                orbit.r = ( orbit.r - time_delta ).max( ORBIT_CAMERA_MIN_R ),
            KeyCode::Up =>
                orbit.theta = ( orbit.theta + time_delta ).min( ORBIT_CAMERA_MAX_THETA ),
            KeyCode::Down =>
                orbit.theta = ( orbit.theta - time_delta ).max( ORBIT_CAMERA_MIN_THETA ),
            KeyCode::Left =>
            {   orbit.phi -= time_delta;
                orbit.phi += if orbit.phi < 0.0 { TAU } else { 0.0 };
            }
            KeyCode::Right =>
            {   orbit.phi += time_delta;
                orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
            }
            _ => (),
        }
    }
}

//------------------------------------------------------------------------------

//極座標カメラを動かす
fn move_orbit_camera
(   mut q_camera: Query<( &OrbitCamera, &mut Transform )>,
)
{   let Ok ( ( camera, mut transform ) ) = q_camera.get_single_mut() else { return };

    //カメラの位置と向きを更新する
    let translation = camera.orbit.into_vec3();
    *transform = Transform::from_translation( translation )
        .looking_at( Vec3::ZERO, Vec3::Y );
}

//------------------------------------------------------------------------------

//極座標の情報を表示する
fn show_parameter
(   mut q_text: Query<&mut Text, With<DisplayBoard>>,
    q_camera: Query<&OrbitCamera>,
)
{   let Ok ( mut text ) = q_text.get_single_mut() else { return };
    let Ok ( camera ) = q_camera.get_single() else { return };
    let orbit = &camera.orbit;

    let r     = orbit.r;
    let theta = orbit.theta.to_degrees(); //ラジアンから度へ変換
    let phi   = orbit.phi.to_degrees();   //ラジアンから度へ変換
    let info  = format!( " r:{r:3.02}\n theta:{theta:06.02}\n phi:{phi:06.02}" );

    text.sections[ 0 ].value = info;
}