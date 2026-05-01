extends Node
class_name ChaseComponent

@export var speed: float = 3.5
@export var awareness_range: float = 15.0
@export var lose_interest_range: float = 20.0
@export var stop_range: float = 2.0
@export var is_aggressive: bool = true
@export var target_group: String = "Player"
@export var debug_mode: bool = true

@export var actor: CharacterBody3D
@export var nav_agent: NavigationAgent3D

var target: Node3D = null
var is_chasing: bool = false

func _ready():
	if not actor and get_parent() is CharacterBody3D:
		actor = get_parent()
	if not nav_agent and actor:
		nav_agent = actor.get_node_or_null("NavigationAgent3D")
	if debug_mode:
		var actor_name = str(actor.name) if actor else "NULL"
		print("[ChaseComponent] ", actor_name, " ready. Awareness: ", awareness_range)

func _physics_process(_delta: float):
	if not actor or not nav_agent or not is_aggressive:
		return
		
	_find_nearest_target()
	
	if not is_instance_valid(target):
		return
		
	var dist = actor.global_position.distance_to(target.global_position)
	
	if not is_chasing:
		if dist <= awareness_range:
			is_chasing = true
			if debug_mode:
				print("[ChaseComponent] ", actor.name, " SPOTTED: ", target.name, " at dist: ", "%.2f" % dist)
	else:
		if dist > lose_interest_range:
			is_chasing = false
			actor.velocity = Vector3.ZERO
			return
		
		if dist <= stop_range:
			actor.velocity = Vector3.ZERO
			return
			
		nav_agent.target_position = target.global_position
		var next_pos = nav_agent.get_next_path_position()
		var direction = actor.global_position.direction_to(next_pos)
		
		direction.y = 0
		direction = direction.normalized()
		
		actor.velocity = direction * speed
		actor.move_and_slide()

func _find_nearest_target():
	var targets = get_tree().get_nodes_in_group(target_group)
	var nearest_dist = INF
	var nearest_node = null
	
	for t in targets:
		if t == actor or t.get_parent() == actor or t == actor.get_parent():
			continue
		
		if not is_instance_valid(t):
			continue
		
		var d = actor.global_position.distance_to(t.global_position)
		if d < nearest_dist:
			nearest_dist = d
			nearest_node = t
	
	target = nearest_node
