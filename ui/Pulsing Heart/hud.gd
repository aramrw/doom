extends CanvasLayer

@onready var health_bar = $TextureProgressBar
@onready var heart_anim = $AnimatedSprite2D 
@onready var fps_label = $FpsLabel
@onready var bullets = $Bullets

func _ready():
	# Force the heart to start pulsing the moment the HUD loads
	heart_anim.play("default") 

func update_health(current_health: int):
	health_bar.value = current_health

func _process(_delta):
	# Update the text every frame with the current FPS
	fps_label.text = str(Engine.get_frames_per_second())

func update_bullets(count: int):
	# This finds the child node that actually handles the ammo display
	$Bullets.update_bullet_count(count)
