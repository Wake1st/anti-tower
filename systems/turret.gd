class_name Turret
extends Node2D


const PROJECTILE = preload("uid://dyjrpaxqhc7bq")

@export_range(0,1,0.01) var rate_of_fire: float = 0.5
var cooldown: float

@onready var base_sprite: Sprite2D = $BaseSprite
@onready var barrel_pivot: Node2D = $BarrelPivot
@onready var shoot_sfx: AudioStreamPlayer2D = $ShootSFX

var targets: Array[Unit]


func _ready() -> void:
	cooldown = 1/rate_of_fire


func _process(delta: float) -> void:
	cooldown -= delta
	
	if targets.size() > 0:
		# face target
		var closest_target = targets.get(0)
		if closest_target == null: return # stoopid check
		
		# flip turret 
		var relative_diff_x = closest_target.global_position.x - global_position.x
		base_sprite.flip_h = relative_diff_x >= 0
		
		# point barrel
		var angle_to_target = global_position.angle_to_point(closest_target.global_position)
		barrel_pivot.global_rotation = angle_to_target
		
		# turret fires the moment targets are available
		if cooldown <= 0:
			cooldown = 1/rate_of_fire
			_fire(closest_target)


func _fire(target: Unit) -> void:
	var projectile: Projectile = PROJECTILE.instantiate()
	add_child(projectile)
	projectile.fire(target)
	
	shoot_sfx.play()


func _on_area_2d_area_entered(area: Area2D) -> void:
	var body = area.get_parent()
	targets.push_back(body)


func _on_area_2d_area_exited(_area: Area2D) -> void:
	targets.pop_front()
