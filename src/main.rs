//external crates
use bevy::
{   prelude::*,
    render::*, render::settings::*,
    core_pipeline::clear_color::*,
    input::mouse::*,
    window::WindowMode::*,
};

//standard library
use std::f32::consts::*;

//internal submodules
mod spawn_objs;
mod const_defs;
use const_defs::*;
use catch_input::*;
use spawn_objs::*;
mod catch_input;

//------------------------------------------------------------------------------

fn main()
{   //Note：手元の環境だとVulkanのままでは影が描画されなかったので、DX12へ切り替えた。
    let backends = Some ( Backends::DX12 );
    let wgpu_settings = WgpuSettings { backends, ..default() };
    let backend_dx12 = RenderPlugin { wgpu_settings };

    App::new()
        //DefaultPluginsに各種の面倒を見てもらう
        .add_plugins
        (   DefaultPlugins
                //Note：この行をコメントアウトするとデフォルトのbackend
                /*.set( backend_dx12 )*/
        )

        //各種オブジェクトを作成する
        .add_systems
        (   Startup, 
            (   spawn_objs::camera3d_and_light, //3Dカメラとライト
                spawn_objs::locked_chest,       //3Dオブジェクト(宝箱)
                spawn_objs::camera2d,           //2Dカメラ(情報表示用)
                spawn_objs::display_board,      //UIテキスト(情報表示用)
            )
        )

        //メインルーチンを登録する
        .add_systems
        (   Update,
            (   (   (   catch_input::from_keyboard, //極座標を更新(キー入力)
                        catch_input::from_mouse,    //極座標を更新(マウス)
                        catch_input::from_gamepad,  //極座標を更新(ゲームパッド)
                    ),
                    move_orbit_camera,              //極座標カメラを移動
                )
                .chain(), //実行順を固定

                bevy::window::close_on_esc, //[ESC]キーで終了
                toggle_window_mode,         //ウィンドウとフルスクリーンの切換
                show_parameter,             //情報を表示 
            )
        )

        //アプリを実行する
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
pub struct OrbitCamera { orbit: Orbit }

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

//ウィンドウとフルスクリーンの切換(トグル動作)
pub fn toggle_window_mode
(   mut q_window: Query<&mut Window>,
    inkey: Res<Input<KeyCode>>,
    inbtn: Res<Input<GamepadButton>>,
)
{   let Ok( mut window ) = q_window.get_single_mut() else { return };

    //[Alt]+[Enter]キーの状態
    let is_key_pressed =
        ( inkey.pressed( KeyCode::AltRight ) || inkey.pressed( KeyCode::AltLeft ) )
            && inkey.just_pressed( KeyCode::Return );

    //入力がないなら
    if ! is_key_pressed  { return }

    //ウィンドウとフルスクリーンを切り替える
    window.mode = match window.mode
    {   Windowed => SizedFullscreen, //or BorderlessFullscreen, Fullscreen
        _        => Windowed,
    };
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

    //極座標の情報
    let r     = orbit.r;
    let theta = orbit.theta.to_degrees(); //ラジアンから度へ変換
    let phi   = orbit.phi.to_degrees();   //ラジアンから度へ変換
    let info  = format!( " r:{r:3.02}\n theta:{theta:06.02}\n phi:{phi:06.02}" );

    //表示の更新
    text.sections[ 0 ].value = format!( "{info}" );
}
