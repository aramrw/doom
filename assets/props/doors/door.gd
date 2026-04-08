@tool # This makes it run inside the Editor so you can see it work
extends Node3D

@export var door_mesh: MeshInstance3D:
	set(value):
		door_mesh = value
		_setup_shader()

@export var dissolve_noise: Texture2D
@export var required_item: InventoryItemData
@export var consume_on_use: bool = true
@export var is_open: bool = false

func _ready():
	if not Engine.is_editor_hint():
		_setup_shader()

func _setup_shader():
	if not door_mesh: return
	
	# 1. Create the shader material automatically
	var shader_mat = ShaderMaterial.new()
	shader_mat.shader = load("res://assets/props/doors/dissolve_door.gdshader")
	
	# 2. Grab the existing texture from the door so it doesn't turn white
	var old_mat = door_mesh.get_active_material(0)
	if old_mat is StandardMaterial3D:
		shader_mat.set_shader_parameter("albedo_texture", old_mat.albedo_texture)
	
	# 3. Apply it
	shader_mat.set_shader_parameter("noise_texture", dissolve_noise)
	door_mesh.set_surface_override_material(0, shader_mat)

func interact(player: Node):
	if is_open:
		return
		
	# Check if player is holding the required item
	var inv = player.get_inventory_manager()
	if not inv: 
		print("Door: No inventory manager found on player")
		return
	
	var active_item = inv.get_active_item()
	
	if required_item == null:
		open_door()
		return
		
	# Use name comparison for more robustness if references are weird
	if active_item and active_item.item_name == required_item.item_name:
		print("Door: Player used ", active_item.item_name)
		if consume_on_use:
			inv.consume_item(active_item.item_name, 1)
		open_door()
	else:
		print("Door: You need ", required_item.item_name, " to open this.")

func open_door():
	if is_open: return
	is_open = true
	print("Door: Opening!")
	play_dissolve()
	# Disable collision so player can pass
	if has_node("StaticBody3D"):
		$StaticBody3D/CollisionShape3D.disabled = true

func play_dissolve():
	# This looks for an AnimationPlayer child and plays it
	if has_node("AnimationPlayer"):
		$AnimationPlayer.play("dissolve")

func _on_area_3d_body_entered(body: Node3D) -> void:
	# Keep this for auto-opening if needed, or remove if interaction only
	pass
