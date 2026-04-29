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
@export var start_node: DialogueNode # ALLOWS EDITING IN INSPECTOR
@export var dialogue_lines: Array[String] = [] # Fallback simple lines

var dialogue_script = load("res://dialogue_interactable.gd")
var dialogue_interactable: Node

func _ready():
	super._ready()
	add_to_group("NPCs")
	if follows_player:
		npc_state = NPCState.FOLLOW
		
	# Find or create a dialogue component
	dialogue_interactable = get_node_or_null("DialogueInteractable")
	if not dialogue_interactable:
		for child in get_children():
			if child.has_method("interact") and child.get_script() and child.get_script().get_path().contains("dialogue_interactable"):
				dialogue_interactable = child
				break
	
	# AGNOSTIC: If still not found, add it dynamically
	if not dialogue_interactable:
		dialogue_interactable = dialogue_script.new()
		dialogue_interactable.name = "DialogueInteractable"
		add_child(dialogue_interactable)
	
	if dialogue_interactable:
		dialogue_interactable.display_name = npc_display_name
		dialogue_interactable.start_node = start_node # Pass the inspector-set node

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
	
	if dialogue_interactable:
		print("BaseNPC: interacting with DialogueInteractable")
		npc_state = NPCState.TALKING
		dialogue_interactable.interact()
		var manager = dialogue_interactable.manager
		if not manager.dialogue_finished.is_connected(_on_dialogue_finished):
			manager.dialogue_finished.connect(_on_dialogue_finished, CONNECT_ONE_SHOT)

func _on_dialogue_finished():
	print("BaseNPC: Dialogue finished signal received")
	npc_state = NPCState.IDLE

func _on_dialogue_action(action_id: String):
	print("NPC: Action triggered: ", action_id)
