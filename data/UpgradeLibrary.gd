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


static func increase(spawner: Types.Spawn, upgrade: Types.Upgrade) -> void:
	match [spawner, upgrade]:
		[Types.Spawn.BASIC,Types.Upgrade.RATE]: lib[Set.BASIC_RATE] += 0.2
		[Types.Spawn.BASIC,Types.Upgrade.SPEED]: lib[Set.BASIC_SPEED] += 0.6
		[Types.Spawn.BASIC,Types.Upgrade.DAMAGE]: lib[Set.BASIC_DAMAGE] += 0.4
		[Types.Spawn.HEAVY,Types.Upgrade.RATE]: lib[Set.HEAVY_RATE] += 0.1
		[Types.Spawn.HEAVY,Types.Upgrade.SPEED]: lib[Set.HEAVY_SPEED] += 0.2
		[Types.Spawn.HEAVY,Types.Upgrade.DAMAGE]: lib[Set.HEAVY_DAMAGE] += 1.0
		[Types.Spawn.MAGIC,Types.Upgrade.RATE]: lib[Set.MAGIC_RATE] += 0.2
		[Types.Spawn.MAGIC,Types.Upgrade.SPEED]: lib[Set.MAGIC_SPEED] += 0.4
		[Types.Spawn.MAGIC,Types.Upgrade.DAMAGE]: lib[Set.MAGIC_DAMAGE] += 0.0


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
