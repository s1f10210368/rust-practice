// 画面にhello, worldをbevyを用いて表示
// bevyとはrustを使用してゲームやアプリを作成する際に使用するもの

// 以下の1行でBevyフレームワークの全ての主要要素をインポート
use bevy::prelude::*;

fn main()
{
    App::new()
        .add_systems(Update, system)
        .run();
}

fn system()
{
    println!("Hello, world!");
}