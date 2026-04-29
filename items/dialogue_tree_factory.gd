extends Node
class_name DialogueTreeFactory

static func create_shotgun_monk_demo() -> DialogueNode:
	# --- Nodes ---
	
	var node_who = DialogueNode.new()
	node_who.dialogue_text = "私はこの修道院の最後の守護者だ。かつては多くの仲間がいたが..."
	
	var node_place = DialogueNode.new()
	node_place.dialogue_text = "ここは聖なる場所だった。だが、亀裂（リフト）がすべてを変えてしまった。"
	
	var node_advice = DialogueNode.new()
	node_advice.dialogue_text = "弾薬を節約しろ。地獄の住人どもは飢えている。頭を狙うのが一番だ。"
	
	var node_trade = DialogueNode.new()
	node_trade.dialogue_text = "いいだろう。地獄を生き抜くための道具を分けてやろう。"
	
	var node_fear = DialogueNode.new()
	node_fear.dialogue_text = "恐怖？ そんなものはとっくに捨てた。死ぬときは死ぬ、ただそれだけだ。"
	
	var node_why = DialogueNode.new()
	node_why.dialogue_text = "約束を果たさねばならんからな。この門を守るという、古き誓いだ。"
	
	# --- Choice Root Node ---
	
	var choice_root = DialogueNode.new()
	choice_root.dialogue_text = "他になにか聞きたいことはあるか？"
	
	# --- Intro Nodes ---
	
	var intro_2 = DialogueNode.new()
	intro_2.dialogue_text = "ジゴクだ。はやく外に出ればいい。"
	
	var intro_1 = DialogueNode.new()
	intro_1.dialogue_text = "生きてるか。"
	
	# --- Linking Intro ---
	
	var c_to_intro2 = DialogueChoice.new()
	c_to_intro2.text = "[ 次へ ]"
	c_to_intro2.next_node = intro_2
	intro_1.choices = [c_to_intro2]
	
	var c_to_choices = DialogueChoice.new()
	c_to_choices.text = "[ 話す ]"
	c_to_choices.next_node = choice_root
	intro_2.choices = [c_to_choices]
	
	# --- Root Choices ---
	
	var c1 = DialogueChoice.new()
	c1.text = "お前は誰だ？"
	c1.next_node = node_who
	
	var c2 = DialogueChoice.new()
	c2.text = "ここはどこだ？"
	c2.next_node = node_place
	
	var c3 = DialogueChoice.new()
	c3.text = "助言をくれ"
	c3.next_node = node_advice
	
	var c_fear = DialogueChoice.new()
	c_fear.text = "怖くないのか？"
	c_fear.next_node = node_fear
	
	var c_why = DialogueChoice.new()
	c_why.text = "なぜここにいる？"
	c_why.next_node = node_why
	
	var c4 = DialogueChoice.new()
	c4.text = "取引する"
	c4.next_node = node_trade
	c4.action_id = "open_shop"
	
	var c5 = DialogueChoice.new()
	c5.text = "去る"
	
	# Root choices array
	var root_choices = [c1, c2, c3, c_fear, c_why, c4, c5]
	choice_root.choices = root_choices
	
	# --- Choices for Sub-nodes (Back to start) ---
	
	var choice_back = DialogueChoice.new()
	choice_back.text = "[ 戻る ]"
	choice_back.next_node = choice_root
	
	# Create specific choice arrays for each sub-node to avoid filtering logic issues
	node_who.choices = [choice_back, c2, c3, c_fear, c_why, c4, c5]
	node_place.choices = [choice_back, c1, c3, c_fear, c_why, c4, c5]
	node_advice.choices = [choice_back, c1, c2, c_fear, c_why, c4, c5]
	node_fear.choices = [choice_back, c1, c2, c3, c_why, c4, c5]
	node_why.choices = [choice_back, c1, c2, c3, c_fear, c4, c5]
	node_trade.choices = [choice_back, c1, c2, c3, c_fear, c_why, c5]
	
	return intro_1
