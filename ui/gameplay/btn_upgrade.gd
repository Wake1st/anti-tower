class_name BtnUpgrade
extends Button


const TEMPLATE: String = "%s - %s (+%s)\n(next = $%s)"

@export var upgrade_type: Types.Upgrade
@export var spawn_type: Types.Spawn


func _ready() -> void:
	update_ui()


func update_ui() -> void:
	disabled = UpgradeLibrary.get_cost(spawn_type, upgrade_type) > Score.points
	
	text = TEMPLATE % [
		Types.Upgrade.find_key(upgrade_type), 
		UpgradeLibrary.value(spawn_type, upgrade_type),
		UpgradeLibrary.get_rate(spawn_type, upgrade_type),
		UpgradeLibrary.get_cost(spawn_type, upgrade_type)
	]


func _on_pressed() -> void:
	Score.points -= UpgradeLibrary.get_cost(spawn_type, upgrade_type)
	UpgradeLibrary.purchase(spawn_type, upgrade_type)
	
	update_ui()
