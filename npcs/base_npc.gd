extends DoomEnemyBase
class_name BaseNPC

enum NPCState { IDLE, FOLLOW, COMBAT, TALKING }
var npc_state = NPCState.IDLE

@export_group("NPC Behavior")
@export var follows_player: bool = false
@export var attacks_enemies: bool = true
@export var attacks_player: bool = false
@export var become_hostile_on_damage: bool = true

@export_group("Dialogue")
@export var npc_display_name: String = "NPC"

func _ready():
	super._ready()
	add_to_group("NPCs")
	if follows_player:
		npc_state = NPCState.FOLLOW

func _physics_process(delta):
	if is_dead:
		super._physics_process(delta)
		return
		
	if npc_state != NPCState.TALKING:
		check_for_targets()
		
	match npc_state:
		NPCState.IDLE:
			process_idle(delta)
		NPCState.FOLLOW:
			process_follow(delta)
		NPCState.COMBAT:
			super._physics_process(delta)
		NPCState.TALKING:
			process_talking(delta)

func process_idle(_delta):
	current_anim_state = "walk"
	velocity.x = 0
	velocity.z = 0
	move_and_slide()
	if follows_player:
		npc_state = NPCState.FOLLOW

func process_follow(_delta):
	var target_pos = player.body.global_position
	var dist = global_position.distance_to(target_pos)
	
	target_node = player.body
	if dist > 4.0:
		chase_target()
	else:
		velocity.x = 0
		velocity.z = 0
		stand_and_stare()
		move_and_slide()

func process_talking(_delta):
	velocity.x = 0
	velocity.z = 0
	move_and_slide()
	var dir_to_player = global_position.direction_to(player.body.global_position)
	var look_target = global_position - dir_to_player
	look_target.y = global_position.y
	if global_position.distance_to(look_target) > 0.1:
		look_at(look_target, Vector3.UP)

func check_for_targets():
	var enemies = get_tree().get_nodes_in_group("Enemies")
	var nearest_enemy = null
	var min_dist = detection_range
	
	for enemy in enemies:
		if enemy == self or enemy.is_dead: continue
		var enemy_body = enemy
		if enemy.has_node("CharacterBody3D"):
			enemy_body = enemy.get_node("CharacterBody3D")
			
		var d = global_position.distance_to(enemy_body.global_position)
		if d < min_dist:
			los_raycast.target_position = to_local(enemy_body.global_position) + Vector3(0, 1, 0)
			los_raycast.force_raycast_update()
			if los_raycast.get_collider() == enemy_body:
				min_dist = d
				nearest_enemy = enemy
			
	if nearest_enemy and attacks_enemies:
		target_node = nearest_enemy
		npc_state = NPCState.COMBAT
	elif attacks_player:
		target_node = player.body
		npc_state = NPCState.COMBAT
	elif follows_player:
		target_node = null
		npc_state = NPCState.FOLLOW
	else:
		target_node = null
		npc_state = NPCState.IDLE

func take_damage(amount: int):
	if become_hostile_on_damage:
		attacks_player = true
		npc_state = NPCState.COMBAT
		target_node = player.body
	super.take_damage(amount)

func interact(_p: Node = null):
	if npc_state == NPCState.COMBAT: return
	
	# New Rust-based Dialogue Component interaction
	var manager = get_node_or_null("DialogueManager")
	if manager and manager.has_method("interact"):
		print("BaseNPC: interacting with DialogueManager")
		npc_state = NPCState.TALKING
		
		# Wire UI to Manager signals
		var ui = get_tree().get_first_node_in_group("DialogueUI")
		if ui:
			# Use bind to pass the npc_display_name and manager reference to DialogueUI
			if not manager.is_connected("dialogue_started", ui.start_dialogue_rs):
				manager.connect("dialogue_started", ui.start_dialogue_rs.bind(npc_display_name, manager))
			if not manager.is_connected("line_changed", ui.update_line):
				manager.connect("line_changed", ui.update_line)
			if not manager.is_connected("dialogue_finished", ui.finish):
				manager.connect("dialogue_finished", ui.finish)
		
		manager.interact()
		
		if not manager.is_connected("dialogue_finished", _on_dialogue_finished):
			manager.connect("dialogue_finished", _on_dialogue_finished, CONNECT_ONE_SHOT)

func _on_dialogue_finished():
	print("BaseNPC: Dialogue finished signal received")
	npc_state = NPCState.IDLE


func _on_dialogue_manager_dialogue_started(line: DialogueLine) -> void:
	pass # Replace with function body.
