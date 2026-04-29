extends SceneTree

func _init():
	print("Generating Monk Dialogue Resource...")
	
	# Create Lines
	var l_who = DialogueLine.new()
	l_who.text = "私はこの修道院の最後の守護者だ。かつては多くの仲間がいたが..."
	l_who.line_type = 0 # Static
	l_who.next_line_index = 2 # Back to choices
	
	var l_place = DialogueLine.new()
	l_place.text = "ここは聖なる場所だった。だが、亀裂（リフト）がすべてを変えてしまった。"
	l_place.line_type = 0 # Static
	l_place.next_line_index = 2 # Back to choices
	
	var l_choice = DialogueLine.new()
	l_choice.text = "他になにか聞きたいことはあるか？"
	l_choice.line_type = 1 # Choice
	
	var l_intro2 = DialogueLine.new()
	l_intro2.text = "ジゴクだ。はやく外に出ればいい。"
	l_intro2.line_type = 0 # Static
	l_intro2.next_line_index = 2 # To choice root
	
	var l_intro1 = DialogueLine.new()
	l_intro1.text = "生きてるか。"
	l_intro1.line_type = 0 # Static
	l_intro1.next_line_index = 3 # To intro 2
	
	# Create Choices for l_choice (Index 2)
	var c1 = DialogueChoice.new()
	c1.text = "お前は誰だ？"
	c1.next_line_index = 0
	
	var c2 = DialogueChoice.new()
	c2.text = "ここはどこだ？"
	c2.next_line_index = 1
	
	var c3 = DialogueChoice.new()
	c3.text = "去る"
	c3.next_line_index = -1
	
	l_choice.choices = [c1, c2, c3]
	
	# Create Resource
	var res = DialogueResource.new()
	res.lines = [l_who, l_place, l_choice, l_intro2, l_intro1]
	res.start_index = 4 # Start at intro 1
	
	var err = ResourceSaver.save(res, "res://npcs/monk_dialogue.tres")
	if err == OK:
		print("Successfully saved monk_dialogue.tres")
	else:
		print("Failed to save resource: ", err)
	
	quit()
