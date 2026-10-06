class_name Game
extends Node


@onready var pause_menu_ui: PauseMenuUI = %PauseMenuUI
@onready var settings_menu_ui: SettingsMenuUI = %SettingsMenuUI
@onready var build_panel: BuildPanel = %BuildPanel
@onready var level: Level = $Level0


func _ready() -> void:
	pause_menu_ui.resume_selected.connect(_handle_resume_selected)
	pause_menu_ui.settings_selected.connect(_handle_settings_selected)
	pause_menu_ui.menu_selected.connect(_handle_menu_selected)
	settings_menu_ui.return_selected.connect(_handle_return_selected)
	
	build_panel.pause_selected.connect(_handle_pause_selected)
	build_panel.start_selected.connect(_handle_start_selected)
	build_panel.upgrade_selected.connect(_handle_upgrade_selected)
	
	level.tower_damaged.connect(_handle_tower_damaged)
	level.tower_destroyed.connect(_handle_tower_destroyed)


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


func _handle_pause_selected() -> void:
	get_tree().paused = true
	pause_menu_ui.focus()


func _handle_start_selected() -> void:
	level.start()


func _handle_upgrade_selected(spawner: Types.Spawn, upgrade: Types.Upgrade) -> void:
	UpgradeLibrary.increase(spawner,upgrade)


func _handle_tower_damaged(amount: float) -> void:
	print("ammount: ", amount)


func _handle_tower_destroyed() -> void:
	get_tree().paused = true
	
	# some kind of score screen
