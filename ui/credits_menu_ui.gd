class_name CreditsMenuUI
extends Control


signal return_selected()


func focus() -> void:
	visible = true


func _on_btn_return_pressed():
	visible = false
	return_selected.emit()
