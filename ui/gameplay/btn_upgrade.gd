class_name BtnUpgrade
extends TextureButton


const TEMPLATE: String = "%s - %s (+%s)\n(next -> %s)"

@export var upgrade_type: Types.Upgrade
var spawn_type: Types.Spawn

@onready var label: Label = %Label


func open(type: Types.Spawn) -> void:
	spawn_type = type
	update_ui()


func update_ui() -> void:
	disabled = UpgradeLibrary.get_cost(spawn_type, upgrade_type) > Score.points
	
	label.text = TEMPLATE % [
		Types.Upgrade.find_key(upgrade_type), 
		UpgradeLibrary.value(spawn_type, upgrade_type),
		UpgradeLibrary.get_rate(spawn_type, upgrade_type),
		UpgradeLibrary.get_cost(spawn_type, upgrade_type)
	]


func _on_pressed() -> void:
	Score.points -= UpgradeLibrary.get_cost(spawn_type, upgrade_type)
	UpgradeLibrary.purchase(spawn_type, upgrade_type)
	
	update_ui()
