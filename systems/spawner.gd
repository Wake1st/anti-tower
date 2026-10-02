class_name Spawner
extends Node2D


const UNIT = preload("uid://5exn17g76420")

signal spawn(unit: Unit)

@export var units_per_second: float = 0.4

var cooldown: float


func _ready() -> void:
	cooldown = 1/units_per_second


func _process(delta) -> void:
	cooldown -= delta
	
	if cooldown <= 0:
		cooldown = 1/units_per_second
		
		spawn.emit(UNIT.instantiate())
