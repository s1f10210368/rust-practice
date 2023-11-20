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
                change_color_helloworld,    //文字の色を変える
                change_size_helloworld,     //文字の大きさを変える
            )
        )
        .run();
}

//カメラを作る
fn spawn_camera2d( mut cmds: Commands )
{   cmds.spawn( Camera2dBundle::default() );
}

//マーカーの準備
#[derive( Component )] struct HelloWorld;

//2Dテキストを作る
fn spawn_text2d_helloworld( mut cmds: Commands )
{   //"Hello, world!"を１文字ごとに分割
    let mut sections = Vec::new();
    for char in "Hello, world!".chars()
    {   let value = char.to_string();
        let style = TextStyle { font_size: 100.0, ..default() };
        sections.push( TextSection { value, style } );
    }
    let text = Text { sections, ..default() };

    cmds.spawn( ( Text2dBundle { text, ..default() }, HelloWorld ) );
}

//文字の色を変える
fn change_color_helloworld
(   mut q_text: Query<&mut Text, With<HelloWorld>>,
    time: Res<Time>,
    mut angle: Local<f32>,
)
{   let Ok ( mut text ) = q_text.get_single_mut() else { return };

    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間
    *angle += 360.0 * time_delta;
    *angle -= if *angle > 360.0 { 360.0 } else { 0.0 };

    //text.sectionsをイテレーターで回して、文字ごとに色を変える
    for ( i, char ) in text.sections.iter_mut().enumerate()
    {   //hue(色相)
        let mut hue = *angle + 10.0 * i as f32;
        hue -= if hue > 360.0 { 360.0 } else { 0.0 };

        //文字の色を変更
        char.style.color = Color::hsl( hue, 1.0, 0.5 );
    } 
}

//文字の大きさを変える
fn change_size_helloworld
(   mut q_text: Query<&mut Text, With<HelloWorld>>,
    time: Res<Time>,
    mut angle: Local<f32>,
)
{   let Ok ( mut text ) = q_text.get_single_mut() else { return };

    let time_delta = time.delta().as_secs_f32(); //前回の実行からの経過時間
    *angle += 360.0 * time_delta;
    *angle -= if *angle > 360.0 { 360.0 } else { 0.0 };

    //text.sectionsをイテレーターで回して、文字ごとに大きさを変える
    for ( i, char ) in text.sections.iter_mut().enumerate()
    {   //sin()を使って文字の大きさを伸縮させる
        let mut angle = *angle + 10.0 * i as f32;
        angle -= if angle > 360.0 { 360.0 } else { 0.0 };
        let size = ( 2.0 + angle.to_radians().sin() ) * 50.0;

        //文字の大きさを変更
        //Note：小数点以下を処理しないと実行時にメモリフットプリントが爆発する
        char.style.font_size = ( size * 10.0 ).floor() / 10.0;
    }
}