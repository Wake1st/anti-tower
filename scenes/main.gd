class_name Main
extends Node


const levels: Array[String] = [
	"res://scenes/levels/level_0.tscn",
	"res://scenes/levels/level_1.tscn",
	"res://scenes/levels/level_2.tscn"
]

@onready var main_menu: MainMenu = $MainMenu
@onready var game: Game = $Game

var scene_path: String
var current_scene: Node


func _ready() -> void:
	#DataAccess.load_user_data()
	scene_path = levels[0]
	
	main_menu.play_level.connect(_handle_level_selected)
	game.return_to_main.connect(_handle_return_selected)
	game.next_level_selected.connect(_handle_next_level)


func _process(_delta) -> void:
	var progress = []
	ResourceLoader.load_threaded_get_status(scene_path, progress)
	
	if progress[0] == 1:
		# set new scene
		var packed_scene: PackedScene = ResourceLoader.load_threaded_get(scene_path)
		current_scene = packed_scene.instantiate()
		
		# connect the signals
		game.run(current_scene)


func _handle_return_selected() -> void:
	main_menu.open()
	get_tree().paused = false


func _handle_level_selected() -> void:
	_load_scene(scene_path)


func _handle_next_level() -> void:
	var next_level_index = levels.rfind(scene_path) + 1
	if next_level_index == levels.size():
		# game over man
		next_level_index = 0
	else:
		var path = levels[next_level_index]
		_load_scene(path)


func _load_scene(path: String) -> void:
	# remove old scene
	if current_scene:
		remove_child(current_scene)
		current_scene.queue_free()
	
	# load in new scene
	scene_path = path
	ResourceLoader.load_threaded_request(scene_path)
