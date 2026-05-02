extends CanvasLayer

@onready var name_label = $Panel/NameLabel
@onready var text_label = $Panel/TextLabel

var choices_container: Control
var audio_player: AudioStreamPlayer

signal dialogue_finished

var current_node = null
var is_active: bool = false
var active_manager: Node = null

@onready var retro_rect = get_tree().get_first_node_in_group("RetroFilter")

func _ready():
	add_to_group("DialogueUI")
	
	# Create choices container if it doesn't exist
	if not has_node("ChoicesContainer"):
		choices_container = GridContainer.new()
		choices_container.name = "ChoicesContainer"
		choices_container.columns = 2
		add_child(choices_container)
	else:
		choices_container = $ChoicesContainer
		if choices_container is GridContainer:
			choices_container.columns = 2
		
	# Position choices centrally above the panel
	choices_container.set_anchors_and_offsets_preset(Control.PRESET_BOTTOM_WIDE, Control.PRESET_MODE_MINSIZE, 2)
	choices_container.grow_vertical = Control.GROW_DIRECTION_BEGIN
	choices_container.offset_bottom = -46 
	choices_container.offset_left = 5
	choices_container.offset_right = -5
	choices_container.add_theme_constant_override("h_separation", 4)
	choices_container.add_theme_constant_override("v_separation", 2)
		
	if not has_node("AudioStreamPlayer"):
		audio_player = AudioStreamPlayer.new()
		audio_player.name = "AudioStreamPlayer"
		add_child(audio_player)
	else:
		audio_player = $AudioStreamPlayer
		
	# Shrink the dialogue text
	if text_label:
		if text_label.label_settings:
			# If it has label_settings, duplicate them so we don't change other labels
			text_label.label_settings = text_label.label_settings.duplicate()
			text_label.label_settings.font_size = 8 # Much smaller
		else:
			text_label.add_theme_font_size_override("font_size", 8)
			
	if name_label:
		name_label.add_theme_font_size_override("font_size", 10)
		
	hide()
	
	if not retro_rect:
		var filter_nodes = get_tree().get_nodes_in_group("RetroFilter")
		if filter_nodes.size() > 0:
			retro_rect = filter_nodes[0]

func _unhandled_input(event: InputEvent) -> void:
	if not is_active:
		return
	
	# Allow navigation with arrow keys (handled by Godot's focus system)
	# ui_accept or interact triggers the focused button
	if event.is_action_pressed("ui_accept") or event.is_action_pressed("interact"):
		var focused = get_viewport().gui_get_focus_owner()
		if focused and focused.get_parent() == choices_container:
			focused.pressed.emit()

func start_dialogue(npc_name: String, lines: Array):
	is_active = true
	name_label.text = npc_name
	if lines.size() > 0:
		text_label.text = lines[0]
	show()

func start_dialogue_rs(line, npc_name: String, manager: Node = null):
	is_active = true
	active_manager = manager
	name_label.text = npc_name
	
	if retro_rect:
		var mat = retro_rect.material as ShaderMaterial
		if mat:
			mat.set_shader_parameter("dialogue_focus", 1.0)
			
	show()
	display_line(line)

func display_line(line):
	current_node = line # Reusing variable name
	
	var display_text = line.text
	var display_audio = line.audio
	
	if line.line_type == 2 or line.line_type == 3: # RandStatic or RandMultiChoice
		if line.random_lines and line.random_lines.size() > 0:
			var rand_line = line.random_lines[randi() % line.random_lines.size()]
			if rand_line:
				display_text = rand_line.text
				display_audio = rand_line.audio
				
	text_label.text = display_text
	
	if display_audio and audio_player:
		audio_player.stream = display_audio
		audio_player.play()
	
	if choices_container:
		for child in choices_container.get_children():
			child.queue_free()
		
		var first_button = null
		
		if line.line_type == 0 or line.line_type == 2: # Static or RandStatic
			var button = Button.new()
			button.text = "[ 次へ ]"
			button.alignment = HORIZONTAL_ALIGNMENT_CENTER
			button.add_theme_font_size_override("font_size", 6)
			button.pressed.connect(_on_next_pressed)
			choices_container.add_child(button)
			first_button = button
		else: # Choice or RandMultiChoice
			for i in range(line.choices.size()):
				var choice = line.choices[i]
				var button = Button.new()
				button.text = choice.label
				button.alignment = HORIZONTAL_ALIGNMENT_CENTER
				button.add_theme_font_size_override("font_size", 6)
				button.pressed.connect(_on_choice_selected.bind(i))
				choices_container.add_child(button)
				if i == 0:
					first_button = button
		
		if first_button:
			first_button.grab_focus()

func _on_next_pressed():
	if active_manager:
		active_manager.advance()
	else:
		var manager = get_tree().get_first_node_in_group("RsDialogueManager")
		if manager:
			manager.advance()

func _on_choice_selected(index: int):
	if active_manager:
		active_manager.select_choice(index)
	else:
		var manager = get_tree().get_first_node_in_group("RsDialogueManager")
		if manager:
			manager.select_choice(index)

func update_line(line):
	if line:
		display_line(line)
	else:
		finish()

func finish():
	is_active = false
	if retro_rect:
		var mat = retro_rect.material as ShaderMaterial
		if mat:
			mat.set_shader_parameter("dialogue_focus", 0.0)
	hide()
	dialogue_finished.emit()
