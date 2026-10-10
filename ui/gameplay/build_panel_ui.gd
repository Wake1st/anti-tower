class_name BuildPanelUI
extends Control


signal pause_selected()
signal start_selected()


func _on_btn_menu_pressed() -> void:
	pause_selected.emit()


func _on_btn_start_pressed() -> void:
	start_selected.emit()
