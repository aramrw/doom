extends StaticBody3D

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func interact(_p: Node = null):
	
	# New Rust-based Dialogue Component interaction
	var manager = get_node_or_null("RsDialogueManager")
	if manager and manager.has_method("interact"):
		print("BaseNPC: interacting with RsDialogueManager")
		
		# Wire UI to Manager signals
		var ui = get_tree().get_first_node_in_group("DialogueUI")
		if ui:
			# Use bind to pass the npc_display_name and manager reference to DialogueUI
			if not manager.is_connected("dialogue_started", ui.start_dialogue_rs):
				manager.connect("dialogue_started", ui.start_dialogue_rs.bind("第一話「恐怖其の物」", manager))
			if not manager.is_connected("line_changed", ui.update_line):
				manager.connect("line_changed", ui.update_line)
			if not manager.is_connected("dialogue_finished", ui.finish):
				manager.connect("dialogue_finished", ui.finish)
		
		manager.interact()
		
		if not manager.is_connected("dialogue_finished", _on_dialogue_finished):
			manager.connect("dialogue_finished", _on_dialogue_finished, CONNECT_ONE_SHOT)

func _on_dialogue_finished():
	print("BaseNPC: Dialogue finished signal received")
