extends BaseProjectile
class_name SimpleProjectile

@onready var sprite = $AnimatedSprite3D

func _ready():
	super._ready()
	if sprite:
		sprite.play("fly")
