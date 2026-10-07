class_name UpgradeLibrary


enum Set {
	BASIC_RATE,
	BASIC_SPEED,
	BASIC_DAMAGE,
	HEAVY_RATE,
	HEAVY_SPEED,
	HEAVY_DAMAGE,
	MAGIC_RATE,
	MAGIC_SPEED,
	MAGIC_DAMAGE
}

static var lib: Dictionary[Set, float] = {
	Set.BASIC_RATE: 0.6,
	Set.BASIC_SPEED: 4.0,
	Set.BASIC_DAMAGE: 1.0,
	Set.HEAVY_RATE: 0.1,
	Set.HEAVY_SPEED: 1.2,
	Set.HEAVY_DAMAGE: 3.4,
	Set.MAGIC_RATE: 0.2,
	Set.MAGIC_SPEED: 2.6,
	Set.MAGIC_DAMAGE: 0.0
}

static var rate: Dictionary[Set, float] = {
	Set.BASIC_RATE: 0.2,
	Set.BASIC_SPEED: 0.6,
	Set.BASIC_DAMAGE: 0.4,
	Set.HEAVY_RATE: 0.1,
	Set.HEAVY_SPEED: 0.2,
	Set.HEAVY_DAMAGE: 1.0,
	Set.MAGIC_RATE: 0.2,
	Set.MAGIC_SPEED: 0.4,
	Set.MAGIC_DAMAGE: 0.0
}

static var cost: Dictionary[Set, float] = {
	Set.BASIC_RATE: 1,
	Set.BASIC_SPEED: 1,
	Set.BASIC_DAMAGE: 1,
	Set.HEAVY_RATE: 1,
	Set.HEAVY_SPEED: 1,
	Set.HEAVY_DAMAGE: 1,
	Set.MAGIC_RATE: 1,
	Set.MAGIC_SPEED: 1,
	Set.MAGIC_DAMAGE: 1
}


static func get_cost(spawner: Types.Spawn, upgrade: Types.Upgrade) -> float:
	match [spawner, upgrade]:
		[Types.Spawn.BASIC,Types.Upgrade.RATE]: return cost[Set.BASIC_RATE]
		[Types.Spawn.BASIC,Types.Upgrade.SPEED]: return cost[Set.BASIC_SPEED]
		[Types.Spawn.BASIC,Types.Upgrade.DAMAGE]: return cost[Set.BASIC_DAMAGE]
		[Types.Spawn.HEAVY,Types.Upgrade.RATE]: return cost[Set.HEAVY_RATE]
		[Types.Spawn.HEAVY,Types.Upgrade.SPEED]: return cost[Set.HEAVY_SPEED]
		[Types.Spawn.HEAVY,Types.Upgrade.DAMAGE]: return cost[Set.HEAVY_DAMAGE]
		[Types.Spawn.MAGIC,Types.Upgrade.RATE]: return cost[Set.MAGIC_RATE]
		[Types.Spawn.MAGIC,Types.Upgrade.SPEED]: return cost[Set.MAGIC_SPEED]
		[Types.Spawn.MAGIC,Types.Upgrade.DAMAGE]: return cost[Set.MAGIC_DAMAGE]
		_: return -1.0


static func get_rate(spawner: Types.Spawn, upgrade: Types.Upgrade) -> float:
	match [spawner, upgrade]:
		[Types.Spawn.BASIC,Types.Upgrade.RATE]: return rate[Set.BASIC_RATE]
		[Types.Spawn.BASIC,Types.Upgrade.SPEED]: return rate[Set.BASIC_SPEED]
		[Types.Spawn.BASIC,Types.Upgrade.DAMAGE]: return rate[Set.BASIC_DAMAGE]
		[Types.Spawn.HEAVY,Types.Upgrade.RATE]: return rate[Set.HEAVY_RATE]
		[Types.Spawn.HEAVY,Types.Upgrade.SPEED]: return rate[Set.HEAVY_SPEED]
		[Types.Spawn.HEAVY,Types.Upgrade.DAMAGE]: return rate[Set.HEAVY_DAMAGE]
		[Types.Spawn.MAGIC,Types.Upgrade.RATE]: return rate[Set.MAGIC_RATE]
		[Types.Spawn.MAGIC,Types.Upgrade.SPEED]: return rate[Set.MAGIC_SPEED]
		[Types.Spawn.MAGIC,Types.Upgrade.DAMAGE]: return rate[Set.MAGIC_DAMAGE]
		_: return -1.0


static func purchase(spawner: Types.Spawn, upgrade: Types.Upgrade) -> void:
	match [spawner, upgrade]:
		[Types.Spawn.BASIC,Types.Upgrade.RATE]: 
			cost[Set.BASIC_RATE] *= 2
			lib[Set.BASIC_RATE] += rate[Set.BASIC_RATE]
		[Types.Spawn.BASIC,Types.Upgrade.SPEED]: 
			cost[Set.BASIC_SPEED] *= 2
			lib[Set.BASIC_SPEED] += rate[Set.BASIC_SPEED]
		[Types.Spawn.BASIC,Types.Upgrade.DAMAGE]: 
			cost[Set.BASIC_DAMAGE] *= 2
			lib[Set.BASIC_DAMAGE] += rate[Set.BASIC_DAMAGE]
		[Types.Spawn.HEAVY,Types.Upgrade.RATE]: 
			cost[Set.HEAVY_RATE] *= 2
			lib[Set.HEAVY_RATE] += rate[Set.HEAVY_RATE]
		[Types.Spawn.HEAVY,Types.Upgrade.SPEED]: 
			cost[Set.HEAVY_SPEED] *= 2
			lib[Set.HEAVY_SPEED] += rate[Set.HEAVY_SPEED]
		[Types.Spawn.HEAVY,Types.Upgrade.DAMAGE]: 
			cost[Set.HEAVY_DAMAGE] *= 2
			lib[Set.HEAVY_DAMAGE] += rate[Set.HEAVY_DAMAGE]
		[Types.Spawn.MAGIC,Types.Upgrade.RATE]: 
			cost[Set.MAGIC_RATE] *= 2
			lib[Set.MAGIC_RATE] += rate[Set.MAGIC_RATE]
		[Types.Spawn.MAGIC,Types.Upgrade.SPEED]: 
			cost[Set.MAGIC_SPEED] *= 2
			lib[Set.MAGIC_SPEED] += rate[Set.MAGIC_SPEED]
		[Types.Spawn.MAGIC,Types.Upgrade.DAMAGE]: 
			cost[Set.MAGIC_DAMAGE] *= 2
			lib[Set.MAGIC_DAMAGE] += rate[Set.MAGIC_DAMAGE]


static func value(spawner: Types.Spawn, upgrade: Types.Upgrade) -> float:
	match [spawner, upgrade]:
		[Types.Spawn.BASIC,Types.Upgrade.RATE]: return lib[Set.BASIC_RATE]
		[Types.Spawn.BASIC,Types.Upgrade.SPEED]: return lib[Set.BASIC_SPEED]
		[Types.Spawn.BASIC,Types.Upgrade.DAMAGE]: return lib[Set.BASIC_DAMAGE]
		[Types.Spawn.HEAVY,Types.Upgrade.RATE]: return lib[Set.HEAVY_RATE]
		[Types.Spawn.HEAVY,Types.Upgrade.SPEED]: return lib[Set.HEAVY_SPEED]
		[Types.Spawn.HEAVY,Types.Upgrade.DAMAGE]: return lib[Set.HEAVY_DAMAGE]
		[Types.Spawn.MAGIC,Types.Upgrade.RATE]: return lib[Set.MAGIC_RATE]
		[Types.Spawn.MAGIC,Types.Upgrade.SPEED]: return lib[Set.MAGIC_SPEED]
		[Types.Spawn.MAGIC,Types.Upgrade.DAMAGE]: return lib[Set.MAGIC_DAMAGE]
		_: return -1.0
