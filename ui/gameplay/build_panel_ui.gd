class_name BuildPanelUI
extends Control


signal pause_selected()
signal start_selected()
signal stop_selected()

const CURRENCY_LABEL: String = "$ %s"

@onready var btn_start: Button = %BtnStart
@onready var tower_health: ProgressBar = %TowerHealth
@onready var currency_amount: Label = %CurrencyAmount

var running: bool


func reset() -> void:
	_update()
	
	tower_health.value = 1.0


func tower_damaged(amount: float) -> void:
	tower_health.value = amount


func _update() -> void:
	for btn: BtnUpgrade in get_tree().get_nodes_in_group("upgrade_btn"):
		btn.update_ui()
	
	currency_amount.text = CURRENCY_LABEL % Score.currency


func _handle_upgrade_purchased() -> void:
	_update()


func _on_btn_menu_pressed() -> void:
	pause_selected.emit()


func _on_btn_start_pressed() -> void:
	if running:
		running = false
		btn_start.text = "GO"
		stop_selected.emit()
	else:
		running = true
		btn_start.text = "RESET"
		start_selected.emit()
