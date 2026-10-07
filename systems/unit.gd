class_name Unit
extends PathFollow2D


const SPEED_DAMP: float = 0.01

signal finished(unit: Unit)

@export var spawn_type: Types.Spawn
@export var speed: float = 4.0
@export var damage: float = 1.0


func update(delta: float) -> void:
	progress_ratio += SPEED_DAMP * speed * delta
	
	if progress_ratio >= 1.0:
		finished.emit(self)


func destroy() -> void:
	call_deferred("queue_free")


func _ready() -> void:
	speed = UpgradeLibrary.value(spawn_type, Types.Upgrade.SPEED)
	damage = UpgradeLibrary.value(spawn_type, Types.Upgrade.DAMAGE)


func _on_area_2d_body_entered(body: Projectile) -> void:
	body.destroy()
	destroy()
