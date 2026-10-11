class_name BtnUpgrade
extends Button


const TEMPLATE: String = "%s - %s (+%s)\n(next = $%s)"

signal upgrade_purchased()

@export var upgrade_type: Types.Upgrade
@export var spawn_type: Types.Spawn


func update_ui() -> void:
	disabled = UpgradeLibrary.get_cost(spawn_type, upgrade_type) > Score.currency
	
	text = TEMPLATE % [
		Types.Upgrade.find_key(upgrade_type), 
		UpgradeLibrary.value(spawn_type, upgrade_type),
		UpgradeLibrary.get_rate(spawn_type, upgrade_type),
		UpgradeLibrary.get_cost(spawn_type, upgrade_type)
	]


func _ready() -> void:
	update_ui()


func _on_pressed() -> void:
	Score.charge(UpgradeLibrary.get_cost(spawn_type, upgrade_type))
	UpgradeLibrary.purchase(spawn_type, upgrade_type)
	
	upgrade_purchased.emit()
