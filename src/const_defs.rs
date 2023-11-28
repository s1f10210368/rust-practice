use super::*;

//2Dカメラの画像を3Dカメラの画像の上にのせる(レンダリングの順位)
pub const CAMERA2D_ORDER: isize = 1;
pub const CAMERA3D_ORDER: isize = 0;

//2Dカメラの画像の背景を透過させる
pub const CAMERA2D_BGCOLOR: ClearColorConfig = ClearColorConfig::None;

//光源
pub const LIGHT_BRIGHTNESS: f32 = 15000.0; //明るさ
pub const LIGHT_POSITION: Vec3 = Vec3::new( 30.0, 100.0, 40.0 ); //位置

//UIテキスト
pub const UI_TEXT_FONT_SIZE: f32 = 50.0;

//極座標カメラの設定値
pub const ORBIT_CAMERA_INIT_R    : f32 = 3.0;      //初期値
pub const ORBIT_CAMERA_INIT_THETA: f32 = PI * 0.7; //初期値(ラジアン)
pub const ORBIT_CAMERA_INIT_PHI  : f32 = 0.0;      //初期値(ラジアン)

pub const ORBIT_CAMERA_MAX_R    : f32 = 5.0;       //最大値
pub const ORBIT_CAMERA_MIN_R    : f32 = 1.0;       //最小値
pub const ORBIT_CAMERA_MAX_THETA: f32 = PI * 0.99; //最大値(ラジアン)
pub const ORBIT_CAMERA_MIN_THETA: f32 = PI * 0.51; //最小値(ラジアン)

//マウスからの入力値の感度調整用係数
pub const MOUSE_WHEEL_Y_COEF : f32 = 0.1;
pub const MOUSE_MOTION_Y_COEF: f32 = 0.01;
pub const MOUSE_MOTION_X_COEF: f32 = 0.01;