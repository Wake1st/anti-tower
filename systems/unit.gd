class_name Unit
extends PathFollow2D


signal finished(unit: Unit)

@export var speed: float = 0.1
@export var damage: float = 1.0


func update(delta: float) -> void:
	progress_ratio += delta * speed
	
	if progress_ratio >= 1.0:
		finished.emit(self)


func destroy() -> void:
	queue_free()
