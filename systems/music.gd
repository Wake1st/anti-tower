class_name Music
extends AudioStreamPlayer


const NATURE = preload("uid://dhphik70ayrra")
const BGM = preload("uid://qlo3ndci1kcb")


func _ready() -> void:
	stream = BGM
	play()


func _on_finished() -> void:
	if stream == BGM:
		stream = NATURE
	else:
		stream = BGM
