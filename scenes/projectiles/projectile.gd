class_name Projectile
extends Node2D

@export var vfx_bus: VfxBus
@export var projectile_stats: ProjectileStates
@export var velocity: Vector2 = Vector2.ZERO

@export var timed_death_particles: PackedScene
@export var hit_death_particles: PackedScene

@onready var hitbox := $Hitbox as Hitbox
@onready var hurtbox := $Hurtbox as Hurtbox

var ignore_targets := []
var should_ignore_targets: bool = true

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
	
func get_damage() -> BasicDamage:
	var basic_damage := BasicDamage.new(projectile_stats.damage, self)
	basic_damage.position = global_position
	basic_damage.direction = Vector2(cos(global_rotation), sin(global_rotation))
	print("Vector(", basic_damage.direction,")")
	return basic_damage

func take_damage(_impact_damage: BasicDamage) -> void:
	var death_vfx := hit_death_particles.instantiate() as Node2D
	if death_vfx:
		death_vfx.global_position = global_position
		var new_rotation = global_rotation + PI
		death_vfx.global_rotation = new_rotation
		vfx_bus.spawn_particle_gpu.emit(death_vfx)

	queue_free()

func _on_projectile_life_timer_timeout() -> void:
	var death_vfx := timed_death_particles.instantiate() as Node2D
	if death_vfx:
		death_vfx.global_position = global_position
		vfx_bus.spawn_particle_gpu.emit(death_vfx)
	queue_free()

func _on_ignore_timer_timeout() -> void:
	hurtbox.ignore_targets = []
	hitbox.ignore_targets = []
