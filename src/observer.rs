use alexandria::{Matrix, Vector3, Vector4};
use colosseum::{Camera, Window};
use std::f32::consts::PI;

type Input = alexandria::StateTrackingInput;

pub struct Observer {
    camera: Camera,
}

const SPEED: f32 = 2.0;
const SENSITIVITY: f32 = 50.0;

impl Observer {
    pub fn new(window: &mut Window<Input>) -> Self {
        window.inner().set_mouse_lock(true);

        let mut camera = Camera::new(window);
        camera.set_rotation(Vector3::new(PI / 2.0, 0.0, 0.0));
        camera.set_position(Vector3::new(0.0, 1.0, 0.0));

        Observer { camera }
    }

    pub fn update(&mut self, delta_time: f32, window: &Window<Input>) {
        let mut position_mod = Vector3::ZERO;
        let mut rotation_mod = Vector3::ZERO;
        let mut updated = false;

        let input = window.input();

        let rotation = self.camera.rotation();

        let forward = (Matrix::rotation(rotation.x(), rotation.y(), 0.0) * Vector4::FORWARD).xyz();
        let right = (Matrix::rotation_y(rotation.y()) * Vector4::RIGHT).xyz();

        if input.get_key(b'W') {
            position_mod += forward;
            updated = true;
        }

        if input.get_key(b'S') {
            position_mod -= forward;
            updated = true;
        }

        if input.get_key(b'A') {
            position_mod -= right;
            updated = true;
        }

        if input.get_key(b'D') {
            position_mod += right;
            updated = true;
        }

        if input.get_key(0x10) {
            position_mod += Vector3::UP;
            updated = true;
        }

        if input.get_key(0x11) {
            position_mod -= Vector3::UP;
            updated = true;
        }

        let mouse_x = 2.0 * (input.get_mouse_x() as f32) / window.width();
        let mouse_y = 2.0 * (input.get_mouse_y() as f32) / window.height();

        if mouse_x != 0.0 || mouse_y != 0.0 {
            rotation_mod += Vector3::new(mouse_y, mouse_x, 0.0);
            updated = true;
        }

        if updated {
            let position = self.camera.position();

            // Clamp new rotation
            let mut new_rotation = rotation + rotation_mod * delta_time * SENSITIVITY;
            if new_rotation.x() < -PI / 2.0 {
                new_rotation.set_x(-PI / 2.0);
            } else if new_rotation.x() > PI / 2.0 {
                new_rotation.set_x(PI / 2.0)
            }

            new_rotation.set_y(new_rotation.y() % (2.0 * PI));

            self.camera
                .set_position(position + position_mod * delta_time * SPEED);
            self.camera.set_rotation(new_rotation);
        }
    }

    pub fn set_active(&mut self, window: &mut Window<Input>) {
        self.camera.set_active(window);
    }
}
