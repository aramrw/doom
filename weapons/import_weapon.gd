@tool
extends Node

@export_category("1. Setup")
@export var weapon_name: String = "NewGun"
@export var damage: int = 15

@export_category("2. Assets")
@export_dir var sprites_folder: String = "res://weapons/"
@export var shoot_sound: AudioStream

@export_category("3. Build")
# Clicking this checkbox in the Inspector runs the code!
@export var IMPORT_NOW: bool = false:
	set(value):
		if value:
			_build_weapon()
			IMPORT_NOW = false # Instantly unchecks the box

func _build_weapon():
	if not Engine.is_editor_hint(): return
	if sprites_folder == "":
		print("ERROR: Please select a sprite folder first!")
		return
		
	print("--- Importing ", weapon_name, " ---")
	
	# 1. Create the new SpriteFrames
	var new_frames = SpriteFrames.new()
	new_frames.remove_animation("default")
	new_frames.add_animation("idle")
	new_frames.add_animation("shoot")
	
	# Grab all the PNGs from the Realm667 folder you selected
	var dir = DirAccess.get_files_at(sprites_folder)
	var pngs = []
	for file in dir:
		if file.ends_with(".png"):
			pngs.append(file)
			
	pngs.sort() # Ensure SHTGA0 comes before SHTGB0
	
	# Realm667 sprites are usually just dumped in one folder.
	# We'll assign the first frame to "idle" and all frames to "shoot"
	if pngs.size() > 0:
		var first_tex = load(sprites_folder + "/" + pngs[0]) as Texture2D
		new_frames.add_frame("idle", first_tex)
		
		for img in pngs:
			var tex = load(sprites_folder + "/" + img) as Texture2D
			new_frames.add_frame("shoot", tex)
			
	# Save the animations
	var anim_path = sprites_folder + "/" + weapon_name + "_anims.tres"
	ResourceSaver.save(new_frames, anim_path)
	
	# 2. Create the actual WeaponData resource
	var new_weapon = WeaponData.new()
	new_weapon.weapon_name = weapon_name
	new_weapon.damage = damage
	new_weapon.shoot_sound = shoot_sound
	new_weapon.sprite_frames = load(anim_path) # Link the anims we just made
	
	# Save the final WeaponData
	var weapon_path = sprites_folder + "/" + weapon_name + "_data.tres"
	ResourceSaver.save(new_weapon, weapon_path)
	
	print("SUCCESS! Saved WeaponData to: ", weapon_path)
