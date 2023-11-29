use godot::engine::{Area2D, CollisionPolygon2D};
use godot::prelude::*;


struct WorldExtends {
    start: Vector2,
    end: Vector2,
}

impl WorldExtends {
    pub fn get_x_wrap(&self, x_position: f32) -> f32 {
        if x_position < self.start.x {
            return self.end.x;
        } else if x_position > self.end.x  {
            return self.start.x;
        }
        return x_position;
    }

    pub fn get_y_wrap(&self, y_position: f32) -> f32 {
        if y_position < self.start.y {
            return self.end.y;
        } else if y_position > self.end.y  {
            return self.start.y;
        }
        return y_position;
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct World {
    area: Option<Gd<Area2D>>,
    area_polygon: Option<Gd<CollisionPolygon2D>>,
    #[export]
    camera: Option<Gd<Camera2D>>,

    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            area: None,
            area_polygon: None,
            camera: None,
            base
        }
    }

    fn ready(&mut self) {
        self.area = self.base.try_get_node_as("Area2D");
        self.area_polygon = self.base.try_get_node_as("Area2D/CollisionPolygon2D");
        self.camera = self.base.try_get_node_as("Camera2D");

        self.resize_area_to_camera_view();
    }
}

#[godot_api]
impl World {
    #[func]
    fn on_body_exit(&mut self, mut body: Gd<Node2D>) {
        let Some(world_extends) = self.get_world_extends() else { return; };
        let body_position = body.get_position();
        let new_x = world_extends.get_x_wrap(body_position.x);
        let new_y = world_extends.get_y_wrap(body_position.y);
        body.set_position(Vector2::new(new_x, new_y));
    }

    fn resize_area_to_camera_view(&mut self) {
        let Some(WorldExtends { start, end }) = self.get_world_extends() else { return; };
        let Some(mut area_polygon) = self.area_polygon.clone() else { return; };

        let extends = [start, Vector2::new(start.x, end.y), end, Vector2::new(end.x, start.y)];
        area_polygon.set_polygon(PackedVector2Array::from(&extends));
    }

    fn get_world_extends(&self) -> Option<WorldExtends> {
        let Some(camera2d) = self.camera.clone() else { return None; };

        let transform = camera2d.get_canvas_transform();
        let camera_size = camera2d.get_viewport_rect().size;

        let transform_invert = transform.affine_inverse();
        let start = transform_invert * Vector2::new(0., 0.);
        let end = transform_invert * camera_size;

        return Some(WorldExtends { start, end });
    }
}