extends Node
class_name DialogueInteractable

@export var display_name: String = "Object"
@export var start_node: DialogueNode

var manager: DialogueManager

func _ready():
	print("DialogueInteractable: Initializing for ", display_name)
	manager = DialogueManager.new()
	add_child(manager)
	manager.add_to_group("DialogueManager")
	
	manager.dialogue_started.connect(_on_dialogue_started)
	manager.node_changed.connect(_on_dialogue_node_changed)
	manager.dialogue_finished.connect(_on_dialogue_finished)
	manager.action_triggered.connect(_on_dialogue_action)

func interact():
	print("DialogueInteractable: interact() called for ", display_name)
	var effective_node = start_node
	
	# Special handling for Shotgun Monk demo
	if not effective_node and (display_name.contains("僧") or display_name.to_lower().contains("monk")):
		print("DialogueInteractable: Using Shotgun Monk demo tree")
		effective_node = DialogueTreeFactory.create_shotgun_monk_demo()
	
	if effective_node:
		print("DialogueInteractable: Starting dialogue tree")
		manager.start_dialogue(effective_node)
	else:
		print("DialogueInteractable: WARNING - No start_node assigned!")
		push_warning("DialogueInteractable: No start_node assigned for " + display_name)

func _on_dialogue_started(node):
	print("DialogueInteractable: Dialogue started signal received")
	var ui = get_tree().get_first_node_in_group("DialogueUI")
	if ui: ui.start_dialogue_rs(display_name, node)

func _on_dialogue_node_changed(node):
	var ui = get_tree().get_first_node_in_group("DialogueUI")
	if ui: ui.update_node(node)

func _on_dialogue_finished():
	print("DialogueInteractable: Dialogue finished")
	var ui = get_tree().get_first_node_in_group("DialogueUI")
	if ui: ui.finish()

func _on_dialogue_action(action_id: String):
	print("DialogueAction: ", action_id)
	if get_parent().has_method("_on_dialogue_action"):
		get_parent()._on_dialogue_action(action_id)
