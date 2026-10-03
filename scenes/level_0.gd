class_name Level
extends Node


@onready var pause_menu_ui: PauseMenuUI = %PauseMenuUI
@onready var settings_menu_ui: SettingsMenuUI = %SettingsMenuUI


func _ready() -> void:
	pause_menu_ui.resume_selected.connect(_handle_resume_selected)
	pause_menu_ui.settings_selected.connect(_handle_settings_selected)
	pause_menu_ui.menu_selected.connect(_handle_menu_selected)
	
	settings_menu_ui.return_selected.connect(_handle_return_selected)


func _input(event) -> void:
	if event.is_action("pause_game"):
		get_tree().paused = true
		pause_menu_ui.focus()


func _handle_resume_selected() -> void:
	get_tree().paused = false


func _handle_settings_selected() -> void:
	pause_menu_ui.visible = false
	settings_menu_ui.focus()


func _handle_menu_selected() -> void:
	# exit level
	pass


func _handle_return_selected() -> void:
	pause_menu_ui.focus()
