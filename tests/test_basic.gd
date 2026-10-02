extends Node2D


@onready var tower: Tower = $Tower
@onready var spawner: Spawner = $Spawner
@onready var highway: Highway = $Highway


func _ready() -> void:
	spawner.spawn.connect(_handle_spawn)
	highway.attack.connect(_handle_attack)
	tower.destroyed.connect(_handle_destroyed)


func _handle_spawn(unit: Unit) -> void:
	highway.spawn(unit)
	print("spawning: ", unit.name)


func _handle_attack(damage: float) -> void:
	tower.hit(damage)
	print("attack: ", damage)


func _handle_destroyed() -> void:
	print("tower destroyed!")
	get_tree().paused = true
