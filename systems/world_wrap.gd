class_name WorldWrap
extends Node2D

@export var camera: Camera2D
@export var bounds: CollisionShape2D

var start: Vector2
var end: Vector2

func _ready():
	_setup_bounds_to_be_camera()
	
func _setup_bounds_to_be_camera():
	var camera_transform = camera.get_canvas_transform()
	var origin = camera.get_screen_center_position()
	var camera_size = camera.get_viewport_rect().size / camera.scale
	var transform_invert = camera_transform.affine_inverse()
	
	start = transform_invert * Vector2(0, 0)
	end = transform_invert * camera_size
	
	var shape = RectangleShape2D.new()
	shape.size = end * 2.0
	bounds.shape = shape


func _on_world_boundaries_area_exited(area: Area2D) -> void:
	wrap_actor(area)

func _on_world_boundaries_body_exited(body: Node2D) -> void:
	wrap_actor(body)
	
func get_bounds() -> Rect2:
	return Rect2(start, end * 2.0)

func wrap_actor(actor: Node2D) -> void:
	if actor.has_method("wrap_to"):
		var node_position = actor.global_position
		var new_x = wrap_x(node_position.x)
		var new_y = wrap_y(node_position.y)
		actor.wrap_to(Vector2(new_x, new_y))

func wrap_x(x: float):
	if x < start.x:
		return end.x
	if x > end.x:
		return start.x
	return x

func wrap_y(y: float):
	if y < start.y:
		return end.y
	if y > end.y:
		return start.y
	return y

