class_name Tower
extends Node2D


signal destroyed()

@export var health: float = 10.0


func hit(damage: float) -> void:
	health -= damage
	
	if health <= 0:
		destroyed.emit()
