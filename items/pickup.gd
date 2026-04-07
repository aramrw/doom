extends Area3D
class_name Pickup

enum Mode { AUTO, MANUAL, BOTH }

@export var data: PickupResource
@export var mode: Mode = Mode.AUTO
@export var auto_pickup_delay: float = 0.5 # Delay before auto-pickup is active

@onready var anim_sprite = $AnimatedSprite3D
@onready var collision_shape = $CollisionShape3D

var _timer: float = 0.0
var _is_active: bool = false

func _ready():
	if not data:
		push_error("Pickup " + name + " has no data assigned!")
		return
		
	if data.sprite_frames:
		anim_sprite.sprite_frames = data.sprite_frames
		# Try typical Doom ground animations
		if data.sprite_frames.has_animation("ground"):
			anim_sprite.play("ground")
		elif data.sprite_frames.has_animation("idle"):
			anim_sprite.play("idle")
		else:
			var anims = data.sprite_frames.get_animation_names()
			if anims.size() > 0:
				anim_sprite.play(anims[0])
			else:
				push_warning("Pickup " + name + " has SpriteFrames but no animations!")
			
	body_entered.connect(_on_body_entered)
	body_exited.connect(_on_body_exited)
	
	# Ensure we are on layer 1 (Environment) and Layer 3 (Interaction)
	collision_layer = 0
	set_collision_layer_value(1, true)
	set_collision_layer_value(3, true)
	
	# Detect Player (Layer 2)
	collision_mask = 0
	set_collision_mask_value(2, true)
	
	set_deferred("monitoring", true)
	set_deferred("monitorable", true)
	
	# Add a StaticBody3D child so RayCast3D can hit us
	var sb = StaticBody3D.new()
	sb.name = "InteractBody"
	sb.collision_layer = 0
	sb.set_collision_layer_value(3, true) # Layer 3: Interaction
	sb.collision_mask = 0
	add_child(sb)
	
	# Create a tiny script to proxy interaction to the parent
	var script = GDScript.new()
	script.set_source_code("extends StaticBody3D\nfunc interact(): get_parent().interact()")
	script.reload()
	sb.set_script(script)
	
	if collision_shape:
		var shape_node = collision_shape.duplicate()
		sb.add_child(shape_node)
	
	_is_active = true

func _process(delta):
	if _timer < auto_pickup_delay:
		_timer += delta

func _on_body_exited(body):
	if body.is_in_group("Player") or body.get_parent().is_in_group("Player"):
		_set_hud_hint("")

func _on_body_entered(body):
	if not _is_active: return
	if _timer < auto_pickup_delay: return
	
	var is_player = body.is_in_group("Player") or body.get_parent().is_in_group("Player")
	if not is_player: return

	if mode == Mode.AUTO or mode == Mode.BOTH:
		_attempt_pickup(body)
	elif mode == Mode.MANUAL:
		var hint = "G TO PICKUP " + data.item_name.to_upper()
		_set_hud_hint(hint)

func _set_hud_hint(text: String):
	var player = get_tree().get_first_node_in_group("Player")
	if player and player.has_node("Hud"):
		player.get_node("Hud").set_pickup_hint(text)

func interact():
	if mode == Mode.MANUAL or mode == Mode.BOTH:
		var player = get_tree().get_first_node_in_group("Player")
		if player:
			_attempt_pickup(player)
			_set_hud_hint("")

func _attempt_pickup(player_node: Node):
	print("Pickup: Attempting pickup of ", data.item_name)
	var player = player_node
	if not player.has_method("handle_pickup"):
		player = player_node.get_parent()
		
	if player and player.has_method("handle_pickup"):
		if player.handle_pickup(data):
			print("Pickup: Success!")
			_play_pickup_sound()
			queue_free()
		else:
			print("Pickup: Player rejected pickup (likely full inventory or logic error)")

func _play_pickup_sound():
	if data.pickup_sound:
		var asp = AudioStreamPlayer.new()
		asp.stream = data.pickup_sound
		asp.bus = &"SfxBus"
		get_tree().root.add_child(asp)
		asp.play()
		asp.finished.connect(asp.queue_free)
