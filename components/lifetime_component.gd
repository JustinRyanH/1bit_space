extends Node

@export var timer: Timer
@export var destroy_component: DestoryComponent

func _ready() -> void:
	timer.timeout.connect(destroy_component.destroy)
	
