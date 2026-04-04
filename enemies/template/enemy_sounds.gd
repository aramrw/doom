extends AudioStreamPlayer3D

# --- FOLDER PATHS ---
@export_group("Sound Folders")
@export_dir var death_folder: String
@export_dir var hurt_folder: String
@export_dir var taunt_folder: String
@export_dir var laugh_folder: String

# --- SETTINGS ---
@export var pitch_step: float = 0.08      # How much the pitch jumps per hit
@export var recovery_speed: float = 0.4   # How fast it goes back to normal

# This will store whatever you set in the Inspector
var base_pitch: float = 1.0

@onready var distortion_effect = AudioServer.get_bus_effect(AudioServer.get_bus_index("SfxBus"), 0)

func _ready():
	# Capture the starting pitch you set in the Godot Inspector
	base_pitch = pitch_scale

func _process(delta):
	# Gradually bring the pitch back to your base_pitch
	if pitch_scale != base_pitch:
		pitch_scale = move_toward(pitch_scale, base_pitch, delta * recovery_speed)
	
	# DYNAMIC DISTORTION:
	# Calculates "crunch" based on how far away you are from the base_pitch.
	if distortion_effect:
		var pitch_difference = abs(pitch_scale - base_pitch)
		var extra_crunch = pitch_difference * 0.8
		distortion_effect.drive = clamp(0.2 + extra_crunch, 0.0, 1.0)

# --- PUBLIC FUNCTIONS ---

func hurt():
	# Nudge the pitch up (the "stress" effect)
	var max_p = base_pitch + 0.6
	pitch_scale = clamp(pitch_scale + randf_range(0.02, pitch_step), base_pitch - 0.4, max_p)
	_play_random_from_dir(hurt_folder)

func death():
	pitch_scale = base_pitch 
	if distortion_effect: distortion_effect.drive = 0.2
	_play_random_from_dir(death_folder)

func taunt():
	# Taunts can be a bit more varied/chaotic
	pitch_scale = base_pitch + randf_range(-0.1, 0.1)
	_play_random_from_dir(taunt_folder)

func laugh():
	pitch_scale = base_pitch + randf_range(-0.05, 0.2) # Laughs usually sound better higher
	_play_random_from_dir(laugh_folder)

# --- INTERNAL HELPER ---

func _play_random_from_dir(dir_path: String):
	if dir_path == "" or not DirAccess.dir_exists_absolute(dir_path): 
		return
	
	var dir = DirAccess.open(dir_path)
	if dir:
		dir.list_dir_begin()
		var file_names = []
		var file_name = dir.get_next()
		
		while file_name != "":
			if not dir.current_is_dir() and (file_name.ends_with(".wav") or file_name.ends_with(".ogg")):
				file_names.append(file_name)
			file_name = dir.get_next()
		
		if file_names.size() > 0:
			var random_file = file_names[randi() % file_names.size()]
			self.stream = load(dir_path + "/" + random_file)
			self.play()
