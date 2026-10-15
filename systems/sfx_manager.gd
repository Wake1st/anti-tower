class_name SFXManager
extends Node


func play(wav: AudioStreamWAV) -> void:
	var player = AudioStreamPlayer.new()
	add_child(player)
	
	player.finished.connect(_handle_sfx_finished.bind(player))
	player.stream = wav
	player.bus = "SFX"
	player.play()


func _handle_sfx_finished(player: AudioStreamPlayer) -> void:
	player.queue_free()
