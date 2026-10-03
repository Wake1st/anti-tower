class_name SettingsMenuUI
extends Control


signal return_selected()


func focus() -> void:
	visible = true


func _on_btn_return_pressed() -> void:
	visible = false
	return_selected.emit()
