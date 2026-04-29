extends Node
class_name DialogueInteractable

@export var display_name: String = "Object"

var manager: DialogueManager

func _ready():
	manager = DialogueManager.new()
	add_child(manager)
	manager.add_to_group("DialogueManager")
	
	manager.dialogue_started.connect(_on_dialogue_started)
	manager.line_changed.connect(_on_dialogue_line_changed)
	manager.dialogue_finished.connect(_on_dialogue_finished)
	manager.action_triggered.connect(_on_dialogue_action)

func _on_dialogue_started(line):
	var ui = get_tree().get_first_node_in_group("DialogueUI")
	if ui: ui.start_dialogue_rs(display_name, line, manager)

func _on_dialogue_line_changed(line):
	var ui = get_tree().get_first_node_in_group("DialogueUI")
	if ui: ui.update_line(line)

func _on_dialogue_finished():
	print("DialogueInteractable: Dialogue finished")
	var ui = get_tree().get_first_node_in_group("DialogueUI")
	if ui: ui.finish()

func _on_dialogue_action(action_id: String):
	print("DialogueAction: ", action_id)
	if get_parent().has_method("_on_dialogue_action"):
		get_parent()._on_dialogue_action(action_id)
