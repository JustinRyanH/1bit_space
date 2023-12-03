class_name Projectile
extends Node2D

@export var vfx_bus: VfxBus
@export var projectile_stats: ProjectileStates
@export var velocity: Vector2 = Vector2.ZERO

@onready var hitbox := $Hitbox as Hitbox
@onready var hurtbox := $Hurtbox as Hurtbox

var ignore_targets := []
var should_ignore_targets: bool = true

func _init() -> void:
	vfx_bus = VfxBus.new()
	projectile_stats = ProjectileStates.new()

func _ready() -> void:
	hitbox.damage = projectile_stats.damage
	hurtbox.ignore_targets = ignore_targets
	hitbox.ignore_targets = ignore_targets

func setup(initial_velocity: Vector2, direction: Vector2) -> void:
	velocity = initial_velocity + (direction * projectile_stats.speed)

func _process(delta: float) -> void:
	translate(velocity * delta)

func add_ignore_targets(target: Node2D) -> void:
	ignore_targets.append(target)

func take_damage(_damage: int) -> void:
	queue_free()

func _on_projectile_life_timer_timeout() -> void:
	queue_free()

func _on_ignore_timer_timeout() -> void:
	hurtbox.ignore_targets = []
	hitbox.ignore_targets = []
