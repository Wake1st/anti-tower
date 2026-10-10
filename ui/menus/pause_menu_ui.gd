class_name PauseMenuUI
extends Control


signal resume_selected()
signal settings_selected()
signal menu_selected()


func focus() -> void:
	visible = true


func _on_btn_resume_pressed() -> void:
	visible = false
	resume_selected.emit()


func _on_btn_settings_pressed() -> void:
	settings_selected.emit()


func _on_btn_menu_pressed() -> void:
	visible = false
	menu_selected.emit()
