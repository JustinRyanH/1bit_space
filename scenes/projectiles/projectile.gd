class_name Projectile
extends Node2D

@export var projectile_stats: ProjectileStates
@export var velocity: Vector2 = Vector2.ZERO

@onready var hitbox := $Hitbox as Hitbox
@onready var hurtbox: Hurtbox = $Hurtbox

var ignore_targets := []
var should_ignore_targets: bool = true

func _ready() -> void:
	hitbox.damage = projectile_stats.damage
	hitbox.disabled = true
	hurtbox.disabled = true
	modulate.b = 0
	
func setup(initial_velocity: Vector2, direction: Vector2) -> void:
	velocity = initial_velocity + (direction * projectile_stats.speed)

func _process(delta: float) -> void:
	translate(velocity * delta)

func take_damage(_damage: int) -> void:
	queue_free()

func _on_projectile_life_timer_timeout() -> void:
	queue_free()

func _on_ignore_timer_timeout() -> void:
	hitbox.disabled = false
	hurtbox.disabled = false
	modulate.b = 1
