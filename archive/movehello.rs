use bevy::prelude::*;

fn main()
{   App::new()
        .add_plugins( DefaultPlugins ) //各種の面倒を見てもらう
        .add_systems
        (   Startup, 
            (   spawn_camera2d,          //カメラを作る
                spawn_text2d_helloworld, //2Dテキストを作る
            )
        )
        .add_systems
        (   Update,
            (   bevy::window::close_on_esc, //[ESC]キーで終了
                move_helloworld,            //移動
                rotate_helloworld,          //回転
                scale_helloworld,           //拡縮
            )
        )
        .run();
}

//カメラを作る
fn spawn_camera2d( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() );
}

//マーカーの準備
#[derive( Component )] struct HelloWorld1;
#[derive( Component )] struct HelloWorld2;
#[derive( Component )] struct HelloWorld3;

//2Dテキストを作る
fn spawn_text2d_helloworld( mut cmds: Commands )
{   let func = | x: i32 |
    {   let textstyle = TextStyle { font_size: 50.0, ..default() };
        let text = Text::from_section( "Hello, world!", textstyle );
        let transform = Transform::from_xyz( x as f32, 0.0, 0.0 );
    
        Text2dBundle { text, transform, ..default() }
    };
    
    cmds.spawn( ( func( -400 ), HelloWorld1 ) ); //マーカー１付きspawn
    cmds.spawn( ( func(    0 ), HelloWorld2 ) ); //マーカー２付きspawn
    cmds.spawn( ( func(  400 ), HelloWorld3 ) ); //マーカー３付きspawn
}

//移動
fn move_helloworld
(   mut q_transform: Query<&mut Transform, With<HelloWorld1>>, //マーカー１で検索
    time: Res<Time>,
    mut angle: Local<f32>, //ローカル変数
)
{   let Ok ( mut transform ) = q_transform.get_single_mut() else { return };

    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間
    *angle += 360.0 * time_delta;
    *angle -= if *angle > 360.0 { 360.0 } else { 0.0 };
 
    //楕円軌道の移動
    let x = angle.to_radians().cos() * 400.0; //横軸は半径400
    let y = angle.to_radians().sin() * 200.0; //縦軸は半径200
    transform.translation = Vec3::new( x, y, 0.0 );
}

//回転
fn rotate_helloworld
(   mut q_transform: Query<&mut Transform, With<HelloWorld2>>, //マーカー２で検索
    time: Res<Time>,
)
{   let Ok ( mut transform ) = q_transform.get_single_mut() else { return };

    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間
    let angle = 360.0 * time_delta;
    let quat = Quat::from_rotation_z( angle.to_radians() );

    //回転(四元数Quatは掛け算で回る)
    transform.rotation *= quat;
}

//拡縮
fn scale_helloworld
(   mut q_transform: Query<&mut Transform, With<HelloWorld3>>, //マーカー３で検索
    time: Res<Time>,
    mut angle: Local<f32>, //ローカル変数
)
{   let Ok ( mut transform ) = q_transform.get_single_mut() else { return };

    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間
    *angle += 360.0 * time_delta;
    *angle -= if *angle > 360.0 { 360.0 } else { 0.0 };
 
    //拡縮(sin()がマイナスになると表示が反転する)
    transform.scale = Vec3::ONE * angle.to_radians().sin();
}