extends SceneTree

func _init():
	print("Generating Robust Monk Dialogue Resource...")
	
	# Line 0: Who am I?
	var l_who = DialogueLine.new()
	l_who.text = "私はこの修道院の最後の守護者だ。"
	l_who.line_type = 0 # Static
	l_who.next_line_index = 2 # Back to main menu
	
	# Line 1: Where am I?
	var l_place = DialogueLine.new()
	l_place.text = "ここは聖なる場所だった。だが、リフトが全てを変えた。"
	l_place.line_type = 0
	l_place.next_line_index = 2
	
	# Line 2: The Main Choice Menu
	var l_menu = DialogueLine.new()
	l_menu.text = "他になにか聞きたいことはあるか？"
	l_menu.line_type = 1 # Choice
	
	# Line 3: Intro 2
	var l_intro2 = DialogueLine.new()
	l_intro2.text = "ジゴクだ。はやく外に出ればいい。"
	l_intro2.line_type = 0
	l_intro2.next_line_index = 2
	
	# Line 4: Intro 1
	var l_intro1 = DialogueLine.new()
	l_intro1.text = "生きてるか。"
	l_intro1.line_type = 0
	l_intro1.next_line_index = 3
	
	# Choices for the Menu
	var c_who = DialogueChoice.new()
	c_who.text = "お前は誰だ？"
	c_who.next_line_index = 0
	
	var c_place = DialogueChoice.new()
	c_place.text = "ここはどこだ？"
	c_place.next_line_index = 1
	
	var c_leave = DialogueChoice.new()
	c_leave.text = "去る"
	c_leave.next_line_index = -1 # Finish
	
	l_menu.choices = [c_who, c_place, c_leave]
	
	# Create Resource
	var res = DialogueResource.new()
	res.lines = [l_who, l_place, l_menu, l_intro2, l_intro1]
	res.start_index = 4
	
	var dir = DirAccess.open("res://npcs/")
	if not dir:
		DirAccess.make_dir_recursive_absolute("res://npcs/")
		
	var err = ResourceSaver.save(res, "res://npcs/monk_dialogue.tres")
	if err == OK:
		print("Successfully saved monk_dialogue.tres")
	else:
		print("Failed to save: ", err)
	
	quit()
