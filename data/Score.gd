class_name Score


static var points: float
static var currency: float


static func add_points(amount: float) -> void:
	points += amount


static func charge(amount: float) -> void:
	currency -= amount


static func reset() -> void:
	currency = points
