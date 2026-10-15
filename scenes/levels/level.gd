class_name Level
extends Node2D


signal tower_damaged(amount: float)
signal tower_destroyed(amount: float)

@onready var tower: Tower = $Tower
@onready var basic_spawner: Spawner = $BasicSpawner
@onready var heavy_spawner: Spawner = $HeavySpawner
@onready var magic_spawner: Spawner = $MagicSpawner
@onready var highway: Highway = $Highway

var standby: bool = true


func start() -> void:
	TowerHealth.set_max(tower.health)
	
	standby = false


func stop() -> void:
	standby = true


func _ready() -> void:
	basic_spawner.spawn.connect(_handle_spawn)
	heavy_spawner.spawn.connect(_handle_spawn)
	magic_spawner.spawn.connect(_handle_spawn)
	highway.attack.connect(_handle_attack)
	tower.destroyed.connect(_handle_destroyed)


func _process(delta: float) -> void:
	if standby: return
	
	basic_spawner.process(delta)
	heavy_spawner.process(delta)
	magic_spawner.process(delta)
	highway.process(delta)



func _handle_spawn(unit: Unit) -> void:
	highway.spawn(unit)


func _handle_attack(damage: float) -> void:
	tower.hit(damage)
	tower_damaged.emit(damage)


func _handle_destroyed(amount: float) -> void:
	tower_destroyed.emit(amount)
