class_name Spawner
extends Node2D


const UNIT = preload("uid://5exn17g76420")

signal spawn(unit: Unit)

@export var spawn_type: Types.Spawn

@export_range(0,1,0.01) var units_per_second: float = 0.2
var cooldown: float



func _ready() -> void:
	_reset_rate()


func process(delta: float) -> void:
	cooldown -= delta
	
	if cooldown <= 0:
		_reset_rate()
		
		var unit = UNIT.instantiate()
		unit.spawn_type = spawn_type
		spawn.emit(unit)


func _reset_rate() -> void:
	units_per_second = UpgradeLibrary.value(spawn_type, Types.Upgrade.RATE)
	cooldown = 1/units_per_second
