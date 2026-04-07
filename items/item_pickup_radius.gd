extends Area3D

@export var data: ItemData 

@onready var anim_sprite = $AnimatedSprite3D

func _ready():
	if data and data.sprite_frames:
		anim_sprite.sprite_frames = data.sprite_frames
		# Most pickups just use an animation named "default" or "idle"
		anim_sprite.play("default") 
	
	body_entered.connect(_on_body_entered)

func _on_body_entered(body):
	var player = body.get_parent()
	if not (player and player.is_in_group("Player")):
		player = body
		
	# Only trigger if the thing we hit actually has the pickup function
	if player.has_method("handle_pickup"):
		if player.handle_pickup(data):
			queue_free()
