@tool
extends AnimatedSprite3D

@export_category("Realm667 Importer")
@export_dir var sprite_folder: String = ""
@export var sprite_prefix: String = "HOIM" # The 4-letter prefix of the enemy

@export_group("Animation Sequences")
@export var walk_sequence: String = "ABCD"
@export var attack_sequence: String = "EFG"
@export var pain_sequence: String = "H"
@export var death_sequence: String = "IJKLMNOPQRST" # Long to cover all bases

@export_category("Build")
@export var generate_frames: bool = false:
	set(value):
		if value:
			_build_sprite_frames()
			generate_frames = false # Uncheck the box automatically

func _build_sprite_frames():
	if sprite_folder == "" or sprite_prefix == "":
		print("ERROR: Please set the sprite folder and prefix.")
		return
		
	var dir = DirAccess.open(sprite_folder)
	if not dir:
		print("ERROR: Failed to open folder: ", sprite_folder)
		return
		
	# 1. Grab all PNG files in the folder
	var files =[]
	dir.list_dir_begin()
	var file_name = dir.get_next()
	while file_name != "":
		if not dir.current_is_dir() and file_name.ends_with(".png"):
			files.append(file_name)
		file_name = dir.get_next()
		
	# 2. Create a fresh SpriteFrames resource
	var new_frames = SpriteFrames.new()
	new_frames.remove_animation("default")
	
	var sequences = {
		"walk": walk_sequence.to_upper(),
		"attack": attack_sequence.to_upper(),
		"pain": pain_sequence.to_upper(),
		"death": death_sequence.to_upper()
	}
	
	var fps = 8.0 # Standard doom animation speed
	
	# 3. Build the animations
	for state_name in sequences.keys():
		var letters = sequences[state_name]
		if letters.is_empty(): 
			continue
		
		# We only generate angles 1 through 5 (Our math script handles mirroring!)
		for angle in range(1, 6):
			var anim_name = state_name + "_" + str(angle)
			var frames_added = 0
			
			new_frames.add_animation(anim_name)
			new_frames.set_animation_speed(anim_name, fps)
			
			for letter in letters:
				var prefix_and_letter = sprite_prefix.to_upper() + letter
				var found_file = ""
				
				# Search the files for a match
				for f in files:
					var base = f.get_basename().to_upper()
					if base.begins_with(prefix_and_letter):
						# Look at the numbers after the prefix (e.g. "2A8")
						var suffix = base.replace(prefix_and_letter, "")
						
						# If the suffix contains our target angle OR "0" (omnidirectional)
						if suffix.contains(str(angle)) or suffix.contains("0"):
							found_file = f
							break
				
				# If we found a file for this frame/angle, load it!
				if found_file != "":
					var tex = load(sprite_folder + "/" + found_file)
					if tex:
						new_frames.add_frame(anim_name, tex)
						frames_added += 1
			
			# If we didn't find any sprites for this sequence, clean up the empty animation
			if frames_added == 0:
				new_frames.remove_animation(anim_name)
				
	# 4. Apply the new frames to this node!
	self.sprite_frames = new_frames
	print("SUCCESS: Generated SpriteFrames for ", sprite_prefix, "!")
