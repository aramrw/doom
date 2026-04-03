extends CanvasLayer

@onready var health_bar = $TextureProgressBar
@onready var heart_anim = $AnimatedSprite2D # Make sure the name matches your node!

func _ready():
	# Force the heart to start pulsing the moment the HUD loads
	heart_anim.play("default") 

func update_health(current_health: int):
	health_bar.value = current_health
