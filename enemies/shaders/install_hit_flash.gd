@tool
extends AnimatedSprite3D

@export var install_shader: bool = false : set = _on_install

func _on_install(_val):
	# 1. Create the ShaderMaterial
	var mat = ShaderMaterial.new()
	
	# 2. Load the shader file (Make sure the path matches your project!)
	var shader_res = load("res:///enemies/shaders/hit_flash.gdshader")
	if not shader_res:
		printerr("Error: Could not find hit_flash.gdshader. Check your file path!")
		return
		
	mat.shader = shader_res
	
	# 3. Automatically find the texture for the shader
	# It looks at the first frame of your 'pain_1' animation
	if sprite_frames and sprite_frames.has_animation("pain_1"):
		var frame_tex = sprite_frames.get_frame_texture("pain_1", 0)
		mat.set_shader_parameter("tex", frame_tex)
	else:
		printerr("Warning: 'pain_1' animation not found. You'll need to drag the texture manually.")

	# 4. Apply to the sprite
	self.material_override = mat
	print("Hit Flash Shader installed successfully on ", name)

# This function makes it easy for your Enemy script to call the flash
func flash_red(intensity: float):
	if material_override:
		material_override.set_shader_parameter("flash_intensity", intensity)
