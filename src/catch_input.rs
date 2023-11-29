use super::*;

//------------------------------------------------------------------------------

//マウス入力によって極座標カメラの位置を更新する
pub fn from_mouse
(   mut q_camera: Query<&mut OrbitCamera>,
    mouse_nutton: Res<Input<MouseButton>>,
    mut e_mouse_motion: EventReader<MouseMotion>,
    mut e_mouse_wheel: EventReader<MouseWheel>,
)
{   let Ok ( mut camera ) = q_camera.get_single_mut() else { return };
    let orbit = &mut camera.orbit;

    //ホイール
    for mouse_wheel in e_mouse_wheel.iter()
    {   orbit.r += mouse_wheel.y * MOUSE_WHEEL_Y_COEF; //感度良すぎるので
        orbit.r = orbit.r
            .min( ORBIT_CAMERA_MAX_R )
            .max( ORBIT_CAMERA_MIN_R );
    }

    //右ボタンが押されていないなら
    if ! mouse_nutton.pressed( MouseButton::Left ) { return }

    //マウスの上下左右
    for mouse_motion in e_mouse_motion.iter()
    {   //上下首振り
        orbit.theta += mouse_motion.delta.y * MOUSE_MOTION_Y_COEF; //感度良すぎるので
        orbit.theta = orbit.theta
            .min( ORBIT_CAMERA_MAX_THETA )
            .max( ORBIT_CAMERA_MIN_THETA );

        //左右回転
        orbit.phi -= mouse_motion.delta.x * MOUSE_MOTION_X_COEF; //感度良すぎるので
        orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
        orbit.phi += if orbit.phi <  0.0 { TAU } else { 0.0 };
    }
}

//------------------------------------------------------------------------------

//キー入力によって極座標カメラの位置を更新する
pub fn from_keyboard
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
            KeyCode::Right =>
            {   orbit.phi -= time_delta;
                orbit.phi += if orbit.phi < 0.0 { TAU } else { 0.0 };
            }
            KeyCode::Left =>
            {   orbit.phi += time_delta;
                orbit.phi -= if orbit.phi >= TAU { TAU } else { 0.0 };
            }
            _ => (),
        }
    }
}