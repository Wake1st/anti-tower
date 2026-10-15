class_name Highway
extends Path2D


signal attack(damage: float)

@onready var sfx_manager: SFXManager = $SFXManager


func spawn(unit: Unit) -> void:
	unit.attacked.connect(_handle_attacked)
	unit.finished.connect(_handle_finished)
	add_child(unit)


func process(delta: float) -> void:
	for follow: Unit in get_tree().get_nodes_in_group("unit"):
		follow.update(delta)


func _handle_attacked(unit: Unit) -> void:
	unit.destroy()
	
	sfx_manager.play(unit.death_wav)


func _handle_finished(unit: Unit) -> void:
	unit.finished.disconnect(_handle_finished)
	attack.emit(unit.damage)
	
	unit.destroy()
	
	sfx_manager.play(unit.hit_wav)
