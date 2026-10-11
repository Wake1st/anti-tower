class_name TowerHealth


static var max_health: float
static var health: float


static func set_max(amount: float) -> void:
	max_health = amount
	health = max_health


static func hit(amount: float) -> float:
	health -= amount
	return health / max_health
