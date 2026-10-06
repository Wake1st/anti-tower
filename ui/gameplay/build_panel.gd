class_name BuildPanel
extends Control


signal pause_selected()
signal start_selected()
signal upgrade_selected(spawner: Types.Spawn, upgrade: Types.Upgrade)

@onready var spawn_options: HBoxContainer = %SpawnOptions
@onready var upgrade_options: HBoxContainer = %UpgradeOptions
@onready var btn_menu: Button = %BtnMenu

var current_spawner: Types.Spawn


func _on_btn_menu_pressed() -> void:
	if current_spawner == Types.Spawn.NONE:
		pause_selected.emit()
	else:
		_focus_spawns()


func _on_btn_start_pressed() -> void:
	start_selected.emit()


func _on_btn_basic_spawner_pressed() -> void:
	_focus_upgrades()
	current_spawner = Types.Spawn.BASIC


func _on_btn_magic_pressed() -> void:
	_focus_upgrades()
	current_spawner = Types.Spawn.HEAVY


func _on_btn_heavy_pressed() -> void:
	_focus_upgrades()
	current_spawner = Types.Spawn.MAGIC


func _on_btn_rate_pressed() -> void:
	upgrade_selected.emit(current_spawner, Types.Upgrade.RATE)


func _on_btn_speed_pressed() -> void:
	upgrade_selected.emit(current_spawner, Types.Upgrade.SPEED)


func _on_btn_damage_pressed() -> void:
	upgrade_selected.emit(current_spawner, Types.Upgrade.DAMAGE)


func _focus_spawns() -> void:
	btn_menu.text = "M\nE\nN\nU"
	
	upgrade_options.visible = false
	spawn_options.visible = true
	
	current_spawner = Types.Spawn.NONE


func _focus_upgrades() -> void:
	btn_menu.text = "B\nA\nC\nK"
	
	spawn_options.visible = false
	upgrade_options.visible = true
