extends Node
class_name HealthComponent

signal damaged(amount, new_health, source)
signal died(source)

@export var max_health: int = 100:
	set(value):
		max_health = max(0, value)
		if current_health > max_health:
			current_health = max_health

@export var current_health: int = 100:
	set(value):
		current_health = clampi(value, 0, max_health)

var is_dead: bool = false

func _ready():
	if current_health <= 0:
		is_dead = true

func take_damage(amount: int, source = null):
	if is_dead:
		return

	var actual_damage = min(amount, current_health)
	current_health -= actual_damage
	
	# Emit damaged first
	damaged.emit(actual_damage, current_health, source)
	print("Took ", actual_damage, " damage. Current health: ", current_health)

	# Then check for death
	if current_health <= 0:
		is_dead = true
		died.emit(owner)
