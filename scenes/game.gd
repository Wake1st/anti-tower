class_name Game
extends Node


signal return_to_main()
signal reset_selected()
signal next_level_selected()

@onready var pause_menu_ui: PauseMenuUI = %PauseMenuUI
@onready var settings_menu_ui: SettingsMenuUI = %SettingsMenuUI
@onready var build_panel_ui: BuildPanelUI = %BuildPanelUI
@onready var score_panel_ui: ScorePanelUI = %ScorePanelUI

@onready var begin_sfx: AudioStreamPlayer = $BeginSFX
@onready var win_sfx: AudioStreamPlayer = $WinSFX
@onready var reset_sfx: AudioStreamPlayer = $ResetSFX
@onready var select_sfx: AudioStreamPlayer = $SelectSFX
@onready var deselect_sfx: AudioStreamPlayer = $DeselectSFX

var level: Level


func run(lvl: Level) -> void:
	if level:
		_unplug_level()
		level.queue_free()
	
	Score.reset()
	UpgradeLibrary.reset()
	build_panel_ui.reset()
	
	level = lvl
	_plugin_level()
	
	get_tree().paused = false


func _ready() -> void:
	pause_menu_ui.resume_selected.connect(_handle_resume_selected)
	pause_menu_ui.settings_selected.connect(_handle_settings_selected)
	pause_menu_ui.menu_selected.connect(_handle_menu_selected)
	settings_menu_ui.return_selected.connect(_handle_return_selected)
	
	build_panel_ui.pause_selected.connect(_handle_pause_selected)
	build_panel_ui.start_selected.connect(_handle_start_selected)
	build_panel_ui.stop_selected.connect(_handle_stop_selected)
	
	score_panel_ui.menu_selected.connect(_handle_menu_selected)
	score_panel_ui.next_selected.connect(_handle_next_level)


func _input(event) -> void:
	if event.is_action("pause_game"):
		get_tree().paused = true
		pause_menu_ui.focus()
		select_sfx.play()


func _handle_resume_selected() -> void:
	get_tree().paused = false
	deselect_sfx.play()


func _handle_settings_selected() -> void:
	pause_menu_ui.visible = false
	settings_menu_ui.focus()
	select_sfx.play()


func _handle_menu_selected() -> void:
	# exit level
	return_to_main.emit()
	deselect_sfx.play()


func _handle_next_level() -> void:
	next_level_selected.emit()
	select_sfx.play()


func _handle_return_selected() -> void:
	pause_menu_ui.focus()
	deselect_sfx.play()


func _handle_pause_selected() -> void:
	get_tree().paused = true
	
	pause_menu_ui.focus()
	select_sfx.play()


func _handle_start_selected() -> void:
	if reset_sfx.playing: reset_sfx.stop()
	
	level.start()
	begin_sfx.play()


func _handle_stop_selected() -> void:
	if begin_sfx.playing: begin_sfx.stop()
	
	level.stop()
	reset_selected.emit()
	reset_sfx.play()


func _handle_tower_damaged(amount: float) -> void:
	build_panel_ui.tower_damaged(TowerHealth.hit(amount))


func _handle_tower_destroyed(amount: float) -> void:
	get_tree().paused = true
	
	Score.add_points(amount)
	
	level.stop()
	score_panel_ui.open()
	
	win_sfx.play()


func _plugin_level() -> void:
	add_child(level)
	level.tower_damaged.connect(_handle_tower_damaged)
	level.tower_destroyed.connect(_handle_tower_destroyed)


func _unplug_level() -> void:
	level.tower_damaged.disconnect(_handle_tower_damaged)
	level.tower_destroyed.disconnect(_handle_tower_destroyed)
	remove_child(level)
