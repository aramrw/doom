extends CharacterBody3D
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
@export var dm4_dialogue: Resource = null
@export var dm4_start_node: String = "start"

@onready var health_component = get_node_or_null("HealthComponent")
@onready var chase_component = get_node_or_null("ChaseComponent")
@onready var attack_component = get_node_or_null("AttackComponent")
@onready var animation_component = get_node_or_null("AnimationComponent")

var is_dead: bool = false

func _ready():
	add_to_group("NPCs")
	
	# Explicitly disable components unless they are needed
	if chase_component:
		chase_component.is_aggressive = follows_player or attacks_player or attacks_enemies
		if attacks_player:
			chase_component.target_group = "Player"
		elif attacks_enemies:
			chase_component.target_group = "Enemies"
		elif follows_player:
			chase_component.target_group = "Player"
			chase_component.stop_range = 4.0 # Keep distance when following
	
	if attack_component:
		attack_component.is_aggressive = attacks_player or attacks_enemies
		if attacks_player:
			attack_component.target_group = "Player"
		elif attacks_enemies:
			attack_component.target_group = "Enemies"

	if health_component:
		if animation_component:
			health_component.damaged.connect(animation_component.on_damaged)
			health_component.died.connect(animation_component.on_died)
		health_component.died.connect(_on_died)
	
	if attack_component and animation_component:
		attack_component.attack_fired.connect(animation_component.on_attack_fired)
		
	if animation_component:
		animation_component.actor = self
		animation_component.play_idle()
	
	print("[BaseNPC] ", name, " initialized. State: ", npc_state)

func _physics_process(delta):
	if is_dead:
		return

	match npc_state:
		NPCState.TALKING:
			process_talking(delta)
		NPCState.FOLLOW:
			# Animation component handles movement animations via its own _physics_process
			pass

func process_talking(_delta):
	var players = get_tree().get_nodes_in_group("Player")
	if players.size() > 0:
		var player = players[0]
		var look_target = player.global_position
		look_target.y = global_position.y
		if global_position.distance_to(look_target) > 0.1:
			look_at(look_target, Vector3.UP)

func take_damage(amount: int, source = null):
	if health_component:
		health_component.take_damage(amount, source)
	
	if become_hostile_on_damage and source and source.is_in_group("Player"):
		attacks_player = true
		npc_state = NPCState.COMBAT
		if chase_component:
			chase_component.is_aggressive = true
			chase_component.target_group = "Player"
		if attack_component:
			attack_component.is_aggressive = true
			attack_component.target_group = "Player"

func _on_died(_source):
	is_dead = true
	if chase_component: chase_component.set_physics_process(false)
	if attack_component: attack_component.set_process(false)
	
	# Disable collisions so player doesn't bump into the "corpse"
	collision_layer = 0
	collision_mask = 0

func _on_dialogue_action(action_id: String, args: Array = []):
	print("[BaseNPC] _on_dialogue_action: ", action_id)
	if action_id == "heal":
		var amount = args[0] if args.size() > 0 else 100
		heal(amount)

func heal(amount: int):
	print("[BaseNPC] Healing player for: ", amount)
	npc_state = NPCState.TALKING
	if animation_component:
		if animation_component.sprite.sprite_frames.has_animation("heal_1"):
			animation_component.sprite.play("heal_1")
			animation_component.sprite.animation_finished.connect(func():
				var players = get_tree().get_nodes_in_group("Player")
				if players.size() > 0:
					# The player node is the parent of the CharacterBody3D node if the CharacterBody3D was returned.
					# Let's get the script instance correctly.
					var player = players[0]
					if player.has_method("heal"):
						player.heal(amount)
					elif player.get_parent().has_method("heal"):
						player.get_parent().heal(amount)
				_on_dialogue_finished()
			, CONNECT_ONE_SHOT)
			return

	# Fallback if animation fails
	var players = get_tree().get_nodes_in_group("Player")
	if players.size() > 0:
		var player = players[0]
		if player.has_method("heal"):
			player.heal(amount)
		elif player.get_parent().has_method("heal"):
			player.get_parent().heal(amount)
	_on_dialogue_finished()

func interact(_p: Node = null):
	print("[BaseNPC] ", name, " interact() called. Current state: ", npc_state)
	if npc_state == NPCState.COMBAT or npc_state == NPCState.TALKING: 
		print("[BaseNPC] interaction blocked by state: ", npc_state)
		return

	if dm4_dialogue:
		print("[BaseNPC] Starting DM4 dialogue")
		npc_state = NPCState.TALKING
		
		if chase_component: chase_component.is_aggressive = false
		if attack_component: attack_component.is_aggressive = false
		
		var player = get_tree().get_first_node_in_group("Player")
		# DialogueManager handles balloons directly via show_example_dialogue_balloon
		# We must use Engine.get_singleton because DialogueManager is an autoload
		Engine.get_singleton("DialogueManager").show_example_dialogue_balloon(dm4_dialogue, dm4_start_node, [self, player])
		
		if not Engine.get_singleton("DialogueManager").is_connected("dialogue_ended", _on_dm4_finished):
			Engine.get_singleton("DialogueManager").connect("dialogue_ended", _on_dm4_finished)
		return

	var manager = get_node_or_null("RsDialogueManager")
	if not manager:
		# Fallback: check children for anything named RsDialogueManager
		for child in get_children():
			if "RsDialogueManager" in child.name:
				manager = child
				break

	if manager and manager.has_method("interact"):
		print("[BaseNPC] Starting dialogue with manager: ", manager.name)
		npc_state = NPCState.TALKING
		
		if chase_component: chase_component.is_aggressive = false
		if attack_component: attack_component.is_aggressive = false

		var ui = get_tree().get_first_node_in_group("DialogueUI")
		if ui:
			if not manager.is_connected("dialogue_started", ui.start_dialogue_rs):
				manager.connect("dialogue_started", ui.start_dialogue_rs.bind(npc_display_name, manager))
			if not manager.is_connected("line_changed", ui.update_line):
				manager.connect("line_changed", ui.update_line)
			if not manager.is_connected("dialogue_finished", ui.finish):
				manager.connect("dialogue_finished", ui.finish)

		manager.interact()

		if not manager.is_connected("dialogue_finished", _on_dialogue_finished):
			manager.connect("dialogue_finished", _on_dialogue_finished, CONNECT_ONE_SHOT)
	else:
		print("[BaseNPC] No RsDialogueManager found on ", name)

func _on_dialogue_finished():
	print("[BaseNPC] Dialogue finished, returning to normal state")
	npc_state = NPCState.IDLE
	if follows_player: npc_state = NPCState.FOLLOW
	
	# Restore component state based on original behavior flags
	if chase_component:
		chase_component.is_aggressive = follows_player or attacks_player or attacks_enemies
	if attack_component:
		attack_component.is_aggressive = attacks_player or attacks_enemies

func _on_dm4_finished(_resource):
	if Engine.get_singleton("DialogueManager").is_connected("dialogue_ended", _on_dm4_finished):
		Engine.get_singleton("DialogueManager").disconnect("dialogue_ended", _on_dm4_finished)
	_on_dialogue_finished()
