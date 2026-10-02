class_name Turret
extends Node2D


const PROJECTILE = preload("uid://dyjrpaxqhc7bq")

@export_range(0,1,0.01) var rate_of_fire: float = 0.5
var cooldown: float

var targets: Array[Unit]


func _ready() -> void:
	cooldown = 1/rate_of_fire


func _process(delta: float) -> void:
	cooldown -= delta
	
	if cooldown <= 0 && targets.size() > 0:
			# fire at closest
			var closest_target = targets.get(0)
			_fire(closest_target)
			
			# turret fires the moment targets are available
			cooldown = 1/rate_of_fire


func _fire(target: Unit) -> void:
	var projectile: Projectile = PROJECTILE.instantiate()
	add_child(projectile)
	projectile.fire(target)


func _on_area_2d_area_entered(area: Area2D) -> void:
	var body = area.get_parent()
	targets.push_back(body)


func _on_area_2d_area_exited(_area: Area2D) -> void:
	targets.pop_front()
