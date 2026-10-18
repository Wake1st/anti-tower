class_name SettingsMenuUI
extends Control


signal return_selected()

@onready var master_slider: HSlider = %MasterSlider
@onready var sfx_slider: HSlider = %SFXSlider
@onready var music_slider: HSlider = %MusicSlider
@onready var hover_sfx: AudioStreamPlayer = $HoverSFX


func focus() -> void:
	visible = true
	
	# set initial volume values
	master_slider.value = GameData.mainVolume
	music_slider.value = GameData.musicVolume
	sfx_slider.value = GameData.sfxVolume


func _on_btn_return_pressed() -> void:
	visible = false
	return_selected.emit()


func _on_focus_entered() -> void:
	hover_sfx.play()


func _on_master_slider_value_changed(value: float) -> void:
	AudioServer.set_bus_volume_linear(AudioServer.get_bus_index("Master"), value)
	GameData.mainVolume = value
	GameData.save_user_data()


func _on_music_slider_value_changed(value: float) -> void:
	AudioServer.set_bus_volume_linear(AudioServer.get_bus_index("Music"), value)
	GameData.musicVolume = value
	GameData.save_user_data()


func _on_sfx_slider_value_changed(value: float) -> void:
	AudioServer.set_bus_volume_linear(AudioServer.get_bus_index("SFX"), value)
	GameData.sfxVolume = value
	GameData.save_user_data()
