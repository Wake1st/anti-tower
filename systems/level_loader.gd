class_name LevelLoader
extends Node


signal loaded(level: Level)
signal ended()

const levels: Array[String] = [
	"res://scenes/levels/level_0.tscn",
	"res://scenes/levels/level_1.tscn",
	"res://scenes/levels/level_2.tscn",
	"res://scenes/levels/level_3.tscn",
	"res://scenes/levels/level_4.tscn"
]

var scene_path: String
var current_scene: Node


func load_current() -> void:
	_load_scene(scene_path)


func load_next() -> void:
	var next_level_index = levels.rfind(scene_path) + 1
	if next_level_index == levels.size():
		# game over man
		next_level_index = 0
		scene_path = levels[0]
		ended.emit()
	else:
		var path = levels[next_level_index]
		_load_scene(path)


func _ready() -> void:
	scene_path = levels[0]


func _process(_delta) -> void:
	var progress = []
	ResourceLoader.load_threaded_get_status(scene_path, progress)
	
	if progress[0] == 1:
		# set new scene
		var packed_scene: PackedScene = ResourceLoader.load_threaded_get(scene_path)
		current_scene = packed_scene.instantiate()
		
		# the level is ready
		loaded.emit(current_scene)


func _load_scene(path: String) -> void:
	# remove old scene
	if current_scene && current_scene.get_parent() == self:
		remove_child(current_scene)
		current_scene.queue_free()
	
	# load in new scene
	scene_path = path
	ResourceLoader.load_threaded_request(scene_path)
