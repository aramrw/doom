extends CanvasLayer

@onready var name_label = $Panel/NameLabel
@onready var text_label = $Panel/TextLabel

signal dialogue_finished

var current_lines: Array = []
var current_index: int = 0

func _ready():
	add_to_group("DialogueUI")
	hide()

func start_dialogue(npc_name: String, lines: Array):
	print("DialogueUI: Starting dialogue for ", npc_name, " with ", lines.size(), " lines.")
	current_lines = lines
	current_index = 0
	name_label.text = npc_name
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
	hide()
	dialogue_finished.emit()
