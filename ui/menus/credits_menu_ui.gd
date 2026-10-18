class_name CreditsMenuUI
extends Control


signal return_selected()

@onready var hover_sfx: AudioStreamPlayer = $HoverSFX


func focus() -> void:
	visible = true


func _on_btn_return_pressed() -> void:
	visible = false
	return_selected.emit()


func _on_focus_entered() -> void:
	hover_sfx.play()
