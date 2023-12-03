class_name Projectile
extends Node2D

@export var projectile_stats: ProjectileStates
@export var velocity: Vector2 = Vector2.ZERO

@onready var hitbox := $Hitbox as Hitbox

func _ready() -> void:
	hitbox.damage = projectile_stats.damage
	
func setup(initial_velocity: Vector2, direction: Vector2) -> void:
	pass
	

func _process(delta: float) -> void:
	translate(velocity * delta)

func take_damage(damage: int) -> void:
	queue_free()


func _on_projectile_life_timer_timeout() -> void:
	queue_free()
