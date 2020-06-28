use nalgebra_glm::{ Vec3 };
use super::utils::{ degree_to_radian };

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

// Default camera values
const YAW: f32          = -90.0f32;
const PITCH: f32        = 0.0f32;
const SPEED: f32        = 2.5f32;
const SENSITIVITY: f32  = 0.1f32;
const ZOOM: f32         = 45.0f32;


// An abstract camera class that processes input and calculates the corresponding Euler Angles, Vectors and Matrices for use in OpenGL
pub struct Camera
{
    // camera Attributes
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    // euler Angles
    pub yaw: f32,
    pub pitch: f32,
    // camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera
{
    pub fn new() -> Camera
    {
        let mut cam = Camera {
            front: nalgebra_glm::vec3(0.0f32, 0.0, -1.0),
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,

            right: nalgebra_glm::vec3(0.0f32, 0.0, 0.0),
            up: nalgebra_glm::vec3(0.0f32, 0.0, 0.0),

            position: nalgebra_glm::vec3(0.0f32, 0.0, 0.0),
            world_up: nalgebra_glm::vec3(0.0f32, 1.0, 0.0),
            yaw: YAW,
            pitch: PITCH,
        };

        cam.update_camera_vectors();

        cam
    }

    // returns the view matrix calculated using Euler Angles and the LookAt Matrix
    pub fn get_view_matrix(&self) -> nalgebra_glm::Mat4
    {
        nalgebra_glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    // processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32)
    {
        let velocity = self.movement_speed * delta_time;

        match direction {
            CameraMovement::FORWARD => {
                self.position += self.front * velocity;
            },
            CameraMovement::BACKWARD => {
                self.position -= self.front * velocity;
            },
            CameraMovement::LEFT => {
                self.position -= self.right * velocity;
            },
            CameraMovement::RIGHT => {
                self.position += self.right * velocity;
            },
            CameraMovement::UP => {
                self.position += self.up * velocity;
            },
            CameraMovement::DOWN => {
                self.position += (self.up * -1.0) * velocity;
            }
        }
    }

    // processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    pub fn process_mouse_movement(&mut self, xoffset: &mut f32, yoffset: &mut f32, constrain_pitch: bool)
    {
        *xoffset *= self.mouse_sensitivity;
        *yoffset *= self.mouse_sensitivity;

        self.yaw   += *xoffset;
        self.pitch += *yoffset;

        // make sure that when pitch is out of bounds, screen doesn't get flipped
        if constrain_pitch
        {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        // update Front, Right and Up Vectors using the updated Euler angles
        self.update_camera_vectors();
    }

    // processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
    pub fn process_mouse_scroll(&mut self, yoffset: f32)
    {
        self.zoom -= yoffset;
        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0; 
        }
    }

    // calculates the front vector from the Camera's (updated) Euler Angles
    fn update_camera_vectors(&mut self)
    {
        // calculate the new Front vector
        let mut front: Vec3 = nalgebra_glm::vec3(0.0f32, 0.0, 0.0);
        front.x = f32::cos( degree_to_radian( self.yaw) ) * f32::cos( degree_to_radian(self.pitch) );
        front.y = f32::sin( degree_to_radian( self.pitch) );
        front.z = f32::sin( degree_to_radian( self.yaw) ) * f32::cos( degree_to_radian(self.pitch) );
        self.front = nalgebra_glm::normalize(&front);

        // also re-calculate the Right and Up vector
        self.right = nalgebra_glm::normalize( &nalgebra_glm::cross(&self.front, &self.world_up) );  // normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.up    = nalgebra_glm::normalize( &nalgebra_glm::cross(&self.right, &self.front) );
    }
}