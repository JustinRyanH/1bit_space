class_name Projectile
extends Node2D

@export var projectile_stats: ProjectileStates
var speed_modifier = 0

@onready var hitbox := $Hitbox as Hitbox

func _ready() -> void:
	hitbox.damage = projectile_stats.damage

func _process(delta: float) -> void:
	var vel = Vector2.UP.rotated(rotation) * (projectile_stats.speed + speed_modifier) * delta
	print(vel)
	translate(vel)

func take_damage(damage: int) -> void:
	queue_free()


func _on_projectile_life_timer_timeout() -> void:
	queue_free()
