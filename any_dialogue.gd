extends Node3D
class_name BaseDialogue

@export_group("Dialogue")
@export var npc_display_name: String = "NPC"
@export var dialogue_lines: Array[String] = ["Hello there!"]

func _ready():
	super._ready()
	add_to_group("NPCs")
	if follows_player:
		npc_state = NPCState.FOLLOW

func _physics_process(delta):
	if is_dead:
		super._physics_process(delta)
		return
		
	# Global target scanning (unless talking)
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
	
	target_node = player.body # Ensure look logic has a target
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
	# Look at player while talking
	var dir_to_player = global_position.direction_to(player.body.global_position)
	var look_target = global_position - dir_to_player
	look_target.y = global_position.y
	if global_position.distance_to(look_target) > 0.1:
		look_at(look_target, Vector3.UP)

func check_for_targets():
	# 1. Check for hostile enemies first
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
			# Check Line of Sight
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

func interact(_player: Node = null):
	if npc_state == NPCState.COMBAT: return
	
	print("NPC: Interaction started with ", npc_display_name)
	npc_state = NPCState.TALKING
	var dialogue_ui = get_tree().get_first_node_in_group("DialogueUI")
	if dialogue_ui:
		print("NPC: DialogueUI found, starting dialogue.")
		dialogue_ui.start_dialogue(npc_display_name, dialogue_lines)
		if not dialogue_ui.dialogue_finished.is_connected(_on_dialogue_finished):
			dialogue_ui.dialogue_finished.connect(_on_dialogue_finished, CONNECT_ONE_SHOT)
	else:
		print("NPC: ERROR - DialogueUI NOT found in group 'DialogueUI'")

func _on_dialogue_finished():
	npc_state = NPCState.IDLE # Will transition back to FOLLOW if enabled
