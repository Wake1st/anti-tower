class_name Projectile
extends CharacterBody2D


const SPEED_BOOST: float = 100

@export var speed: float = 3.0

var target: Unit


func fire(tar: Unit) -> void:
	print("projectile %s - fired at - %s" % [name, tar.name])
	target = tar


func destroy() -> void:
	print("projectile destroyed: ", name)
	call_deferred("queue_free")


func _physics_process(delta: float) -> void:
	# check to ensure there's still a target
	if target == null:
		destroy()
	else:
		var movement = (
			target.global_position - global_position
		).normalized() * SPEED_BOOST * speed * delta
		move_and_collide(movement)
