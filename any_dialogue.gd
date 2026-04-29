extends CharacterBody3D
class_name BaseDialogue

# --- Exports ---
@export_group("Dialogue")
@export var npc_display_name: String = "NPC"
@export var start_node: DialogueNode # Using Rust DialogueNode

@export_group("Behavior")
@export var follows_player: bool = false
@export var detection_range: float = 15.0
@export var attacks_enemies: bool = true
@export var attacks_player: bool = false
@export var become_hostile_on_damage: bool = true

# --- State Management ---
enum NPCState { IDLE, FOLLOW, COMBAT, TALKING }
var npc_state: NPCState = NPCState.IDLE
var is_dead: bool = false
var target_node: Node3D = null

# --- References ---
@onready var player = get_tree().get_first_node_in_group("Player")
@onready var los_raycast: RayCast3D = $RayCast3D

# Rust Dialogue Manager
var dialogue_manager: DialogueManager

func _ready():
	add_to_group("NPCs")
	if follows_player:
		npc_state = NPCState.FOLLOW
	
	# Create the manager and add it to group so UI can find it
	dialogue_manager = DialogueManager.new()
	add_child(dialogue_manager)
	dialogue_manager.add_to_group("DialogueManager")
	
	# Connect manager signals
	dialogue_manager.dialogue_started.connect(_on_dialogue_started)
	dialogue_manager.node_changed.connect(_on_dialogue_node_changed)
	dialogue_manager.dialogue_finished.connect(_on_dialogue_finished_rs)
	dialogue_manager.action_triggered.connect(_on_dialogue_action)

func _physics_process(delta):
	if is_dead:
		return
		
	match npc_state:
		NPCState.IDLE:
			check_for_targets()
			process_idle(delta)
		NPCState.FOLLOW:
			check_for_targets()
			process_follow(delta)
		NPCState.COMBAT:
			check_for_targets()
			process_combat(delta)
		NPCState.TALKING:
			process_talking(delta)

# --- Dialogue Callbacks ---

func _on_dialogue_started(node):
	var dialogue_ui = get_tree().get_first_node_in_group("DialogueUI")
	if dialogue_ui:
		dialogue_ui.start_dialogue_rs(npc_display_name, node)

func _on_dialogue_node_changed(node):
	var dialogue_ui = get_tree().get_first_node_in_group("DialogueUI")
	if dialogue_ui:
		dialogue_ui.update_node(node)

func _on_dialogue_finished_rs():
	var dialogue_ui = get_tree().get_first_node_in_group("DialogueUI")
	if dialogue_ui:
		dialogue_ui.finish()
	npc_state = NPCState.IDLE

func _on_dialogue_action(action_id: String):
	print("NPC: Action triggered: ", action_id)
	if action_id == "give_item":
		# Logic for giving item to player
		pass
	elif action_id == "open_shop":
		# Logic for opening shop
		pass

# --- Rest of existing NPC logic ---

func process_idle(_delta):
	velocity.x = move_toward(velocity.x, 0, 1)
	velocity.z = move_toward(velocity.z, 0, 1)
	move_and_slide()
	if follows_player: npc_state = NPCState.FOLLOW

func process_follow(_delta):
	if not player: return
	var target_pos = player.global_position
	var dist = global_position.distance_to(target_pos)
	if dist > 4.0:
		var dir = global_position.direction_to(target_pos)
		velocity.x = dir.x * 3.0
		velocity.z = dir.z * 3.0
		look_at_target(target_pos)
	else:
		velocity.x = 0
		velocity.z = 0
		look_at_target(target_pos)
	move_and_slide()

func process_talking(_delta):
	velocity = Vector3.ZERO
	move_and_slide()
	if player: look_at_target(player.global_position)

func process_combat(_delta):
	if not target_node or (target_node.has_method("is_dead") and target_node.is_dead):
		npc_state = NPCState.IDLE
		return
	look_at_target(target_node.global_position)

func look_at_target(target_pos: Vector3):
	target_pos.y = global_position.y
	if global_position.distance_to(target_pos) > 0.1:
		look_at(target_pos, Vector3.UP)

func check_for_targets():
	var enemies = get_tree().get_nodes_in_group("Enemies")
	var nearest_enemy = null
	var min_dist = detection_range
	for enemy in enemies:
		if enemy == self or (enemy.has_method("is_dead") and enemy.is_dead): continue
		var d = global_position.distance_to(enemy.global_position)
		if d < min_dist:
			los_raycast.target_position = los_raycast.to_local(enemy.global_position + Vector3(0, 1, 0))
			los_raycast.force_raycast_update()
			if los_raycast.get_collider() == enemy:
				min_dist = d
				nearest_enemy = enemy
	if nearest_enemy and attacks_enemies:
		target_node = nearest_enemy
		npc_state = NPCState.COMBAT
	elif attacks_player and player:
		target_node = player
		npc_state = NPCState.COMBAT

func take_damage(_amount: int):
	if become_hostile_on_damage:
		attacks_player = true
		target_node = player
		npc_state = NPCState.COMBAT

func interact(_p: Node = null):
	if npc_state == NPCState.COMBAT: return
	
	var effective_node = start_node
	if not effective_node and npc_display_name.contains("僧"):
		effective_node = DialogueTreeFactory.create_shotgun_monk_demo()
	
	if effective_node:
		npc_state = NPCState.TALKING
		dialogue_manager.start_dialogue(effective_node)
	else:
		push_warning("NPC: No start_node assigned!")
