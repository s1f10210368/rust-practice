use bevy::prelude::*;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems
        (
            Startup,
            (
                spawn_camera2d, // カメラを作成
                spawn_text2d_helloworld, // 2Dテキストを作成
            )
        )
        .add_systems
        (
            Update,
            (
                bevy::window::close_on_esc, // escキーで終了するように
            )
        )
        .run();
}

// カメラ作成関数
fn spawn_camera2d(mut cmds: Commands)
{
    cmds.spawn(Camera2dBundle::default());
}

//2Dテキスト作成関数
fn spawn_text2d_helloworld(mut cmds: Commands)
{
    let textstyle = TextStyle{font_size: 100.0, ..default()};
    let text = Text::from_section("Hello, world!", textstyle);
    cmds.spawn(Text2dBundle{text, ..default()});
}

fn system()
{
    println!("Hello, world!");
}