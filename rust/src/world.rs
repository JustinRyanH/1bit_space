use godot::engine::{Area2D, CollisionPolygon2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct World {
    area: Option<Gd<Area2D>>,
    area_polygon: Option<Gd<CollisionPolygon2D>>,
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
    fn resize_area_to_camera_view(&mut self) {
        let Some(mut area_polygon) = self.area_polygon.clone() else { return; };
        let Some(camera2d) = self.camera.clone() else { return; };

        let transform = camera2d.get_canvas_transform();
        let camera_size = camera2d.get_viewport_rect().size;

        let transform_invert = transform.affine_inverse();
        let start = transform_invert * Vector2::new(0., 0.);
        let end = transform_invert * camera_size;
        let extends = [start, Vector2::new(start.x, end.y), end, Vector2::new(end.x, start.y)];
        area_polygon.set_polygon(PackedVector2Array::from(&extends));
    }
}