class_name Spawner
extends Sprite2D


signal spawn(unit: Unit)

@export var spawn_type: Types.Spawn
@export var unit_scene: PackedScene
@export var spawn_wav: AudioStreamWAV

@export_range(0,1,0.01) var units_per_second: float = 0.2
var cooldown: float

@onready var spawn_sfx: AudioStreamPlayer2D = $SpawnSFX


func _ready() -> void:
	_reset_rate()
	
	spawn_sfx.stream = spawn_wav


func process(delta: float) -> void:
	cooldown -= delta
	
	if cooldown <= 0:
		_reset_rate()
		
		var unit = unit_scene.instantiate()
		unit.spawn_type = spawn_type
		spawn.emit(unit)
		
		spawn_sfx.play()


func _reset_rate() -> void:
	units_per_second = UpgradeLibrary.value(spawn_type, Types.Upgrade.RATE)
	cooldown = 1/units_per_second
