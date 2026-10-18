class_name Main
extends Node


@onready var loader: LevelLoader = $LevelLoader
@onready var game: Game = $Game
@onready var main_menu: MainMenu = $MainMenu


func _ready() -> void:
	get_tree().paused = true
	
	GameData.load_user_data()
	Score.add_points(5)
	
	main_menu.play_level.connect(_handle_level_selected)
	game.return_to_main.connect(_handle_return_selected)
	game.reset_selected.connect(_handle_reset_selected)
	game.next_level_selected.connect(_handle_next_level)
	loader.loaded.connect(_handle_level_loaded)
	loader.ended.connect(_handle_levels_ended)


func _handle_return_selected() -> void:
	main_menu.open()


func _handle_reset_selected() -> void:
	loader.load_current()


func _handle_level_selected() -> void:
	loader.load_current()


func _handle_next_level() -> void:
	loader.load_next()


func _handle_level_loaded(level: Level) -> void:
	game.run(level)


func _handle_levels_ended() -> void:
	main_menu.open()
