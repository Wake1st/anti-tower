class_name MainMenu
extends CanvasLayer


signal play_level()

@onready var main_menu_ui: MainMenuUI = $MainMenuUI
@onready var settings_menu_ui: SettingsMenuUI = $SettingsMenuUI
@onready var credits_menu_ui: CreditsMenuUI = $CreditsMenuUI

var control: Control


func _ready() -> void:
	main_menu_ui.play_selected.connect(_handle_play_selected)
	main_menu_ui.settings_selected.connect(_handle_settings_selected)
	main_menu_ui.credits_selected.connect(_handle_credits_selected)
	
	settings_menu_ui.return_selected.connect(_handle_return_selected)
	credits_menu_ui.return_selected.connect(_handle_return_selected)


func _handle_play_selected() -> void:
	play_level.emit()


func _handle_settings_selected() -> void:
	settings_menu_ui.focus()


func _handle_credits_selected() -> void:
	credits_menu_ui.focus()


func _handle_return_selected() -> void:
	main_menu_ui.focus()
