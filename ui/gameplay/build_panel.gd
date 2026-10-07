class_name BuildPanel
extends Control


signal pause_selected()
signal start_selected()

@onready var spawn_options: HBoxContainer = %SpawnOptions
@onready var upgrade_options: HBoxContainer = %UpgradeOptions
@onready var btn_menu: Button = %BtnMenu

@onready var btn_rate: BtnUpgrade = %BtnRate
@onready var btn_speed: BtnUpgrade = %BtnSpeed
@onready var btn_damage: BtnUpgrade = %BtnDamage

var current_spawner: Types.Spawn


func _on_btn_menu_pressed() -> void:
	if current_spawner == Types.Spawn.NONE:
		pause_selected.emit()
	else:
		_focus_spawns()


func _on_btn_start_pressed() -> void:
	start_selected.emit()


func _on_btn_basic_spawner_pressed() -> void:
	_focus_upgrades(Types.Spawn.BASIC)


func _on_btn_heavy_pressed() -> void:
	_focus_upgrades(Types.Spawn.HEAVY)


func _on_btn_magic_pressed() -> void:
	_focus_upgrades(Types.Spawn.MAGIC)


func _focus_spawns() -> void:
	btn_menu.text = "M\nE\nN\nU"
	
	upgrade_options.visible = false
	spawn_options.visible = true
	
	current_spawner = Types.Spawn.NONE


func _focus_upgrades(spawn: Types.Spawn) -> void:
	current_spawner = spawn
	
	btn_rate.open(current_spawner)
	btn_speed.open(current_spawner)
	btn_damage.open(current_spawner)
	
	btn_menu.text = "B\nA\nC\nK"
	
	spawn_options.visible = false
	upgrade_options.visible = true
