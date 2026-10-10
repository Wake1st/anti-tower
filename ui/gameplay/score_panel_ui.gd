class_name ScorePanelUI
extends Control


signal menu_selected()
signal next_selected()

@onready var player: AnimationPlayer = $AnimationPlayer


func open() -> void:
	player.play("open")


func _on_btn_next_level_pressed() -> void:
	next_selected.emit()
	player.play_backwards("open")


func _on_btn_main_menu_pressed() -> void:
	menu_selected.emit()
	player.play_backwards("open")
