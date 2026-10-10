class_name Highway
extends Path2D


signal attack(damage: float)


func spawn(unit: Unit) -> void:
	unit.finished.connect(_handle_finished)
	add_child(unit)


func process(delta: float) -> void:
	for follow: Unit in get_children():
		follow.update(delta)


func _handle_finished(unit: Unit) -> void:
	unit.finished.disconnect(_handle_finished)
	remove_child(unit)
	attack.emit(unit.damage)
	
	unit.destroy()
