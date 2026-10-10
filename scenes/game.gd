class_name Game
extends Node


signal return_to_main()
signal next_level_selected()

@onready var pause_menu_ui: PauseMenuUI = %PauseMenuUI
@onready var settings_menu_ui: SettingsMenuUI = %SettingsMenuUI
@onready var build_panel_ui: BuildPanelUI = %BuildPanelUI
@onready var score_panel_ui: ScorePanelUI = %ScorePanelUI
@onready var level: Level


func run(lvl: Level) -> void:
	if level:
		_unplug_level()
		level.queue_free()
	
	level = lvl
	_plugin_level()


func _ready() -> void:
	pause_menu_ui.resume_selected.connect(_handle_resume_selected)
	pause_menu_ui.settings_selected.connect(_handle_settings_selected)
	pause_menu_ui.menu_selected.connect(_handle_menu_selected)
	settings_menu_ui.return_selected.connect(_handle_return_selected)
	
	build_panel_ui.pause_selected.connect(_handle_pause_selected)
	build_panel_ui.start_selected.connect(_handle_start_selected)
	
	score_panel_ui.menu_selected.connect(_handle_menu_selected)
	score_panel_ui.next_selected.connect(_handle_next_level)


func _input(event) -> void:
	if event.is_action("pause_game"):
		pause_menu_ui.focus()


func _handle_resume_selected() -> void:
	pass


func _handle_settings_selected() -> void:
	pause_menu_ui.visible = false
	settings_menu_ui.focus()


func _handle_menu_selected() -> void:
	# exit level
	return_to_main.emit()


func _handle_next_level() -> void:
	next_level_selected.emit()


func _handle_return_selected() -> void:
	pause_menu_ui.focus()


func _handle_pause_selected() -> void:
	pause_menu_ui.focus()


func _handle_start_selected() -> void:
	level.start()


func _handle_tower_damaged(amount: float) -> void:
	print("ammount: ", amount) 


func _handle_tower_destroyed() -> void:
	level.stop()
	score_panel_ui.open()


func _plugin_level() -> void:
	add_child(level)
	level.tower_damaged.connect(_handle_tower_damaged)
	level.tower_destroyed.connect(_handle_tower_destroyed)


func _unplug_level() -> void:
	level.tower_damaged.disconnect(_handle_tower_damaged)
	level.tower_destroyed.disconnect(_handle_tower_destroyed)
	remove_child(level)
