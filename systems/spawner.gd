class_name Spawner
extends Node2D


const UNIT = preload("uid://5exn17g76420")

signal spawn(unit: Unit)

@export_range(0,1,0.01) var units_per_second: float = 0.2
var cooldown: float


func _ready() -> void:
	cooldown = 1/units_per_second


func _process(delta: float) -> void:
	cooldown -= delta
	
	if cooldown <= 0:
		cooldown = 1/units_per_second
		
		spawn.emit(UNIT.instantiate())
