class_name MainMenuUI
extends Control


signal play_selected()
signal settings_selected()
signal credits_selected()


func focus() -> void:
	visible = true


func _on_btn_play_pressed() -> void:
	play_selected.emit()


func _on_btn_settings_pressed() -> void:
	visible = false
	settings_selected.emit()


func _on_btn_credits_pressed() -> void:
	visible = false
	credits_selected.emit()
