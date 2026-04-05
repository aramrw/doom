extends CanvasLayer

@onready var name_label = $Panel/NameLabel
@onready var text_label = $Panel/TextLabel

signal dialogue_finished

var current_lines: Array = []
var current_index: int = 0
var is_active: bool = false

@onready var retro_rect = get_tree().get_first_node_in_group("RetroFilter")

func _ready():
	add_to_group("DialogueUI")
	hide()
	
	# Find retro filter if not already assigned
	if not retro_rect:
		var filter_nodes = get_tree().get_nodes_in_group("RetroFilter")
		if filter_nodes.size() > 0:
			retro_rect = filter_nodes[0]

func start_dialogue(npc_name: String, lines: Array):
	print("DialogueUI: Starting dialogue for ", npc_name, " with ", lines.size(), " lines.")
	is_active = true
	current_lines = lines
	current_index = 0
	name_label.text = npc_name
	
	if retro_rect:
		var mat = retro_rect.material as ShaderMaterial
		if mat:
			mat.set_shader_parameter("dialogue_focus", 1.0)
			
	show()
	display_line()

func display_line():
	if current_index < current_lines.size():
		print("DialogueUI: Displaying line ", current_index, ": ", current_lines[current_index])
		text_label.text = current_lines[current_index]
	else:
		print("DialogueUI: Finished all lines.")
		finish()

func advance():
	current_index += 1
	display_line()

func finish():
	print("DialogueUI: Finishing and hiding.")
	is_active = false
	if retro_rect:
		var mat = retro_rect.material as ShaderMaterial
		if mat:
			mat.set_shader_parameter("dialogue_focus", 0.0)
	hide()
	dialogue_finished.emit()
