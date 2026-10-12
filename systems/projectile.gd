class_name Projectile
extends CharacterBody2D


const SPEED_BOOST: float = 100

@export var speed: float = 9.0

@onready var sprite: Sprite2D = $Sprite2D

var target: Unit


func fire(tar: Unit) -> void:
	target = tar


func destroy() -> void:
	call_deferred("queue_free")


func _physics_process(delta: float) -> void:
	# check to ensure there's still a target
	if target == null:
		destroy()
	else:
		# move towards target
		var norm = (target.global_position - global_position).normalized()
		var movement = norm * SPEED_BOOST * speed * delta
		move_and_collide(movement)
		
		# face target
		global_rotation = norm.angle()
