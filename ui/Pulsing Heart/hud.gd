extends CanvasLayer

@onready var health_bar = $TextureProgressBar
@onready var heart_anim = $AnimatedSprite2D 
@onready var fps_label = $FpsLabel
@onready var bullets = $Bullets
@onready var magazines = $Mag
@onready var crosshair = $CenterContainer/TextureRect
@onready var damage_flash = $DamageFlash

func _ready():
	# Force the heart to start pulsing the moment the HUD loads
	heart_anim.play("default") 
	
	if damage_flash:
		damage_flash.modulate.a = 0
	
	# Connect to weapon manager for hit feedback
	var wm = get_tree().get_first_node_in_group("WeaponManager")
	if wm:
		if not wm.enemy_hit.is_connected(flash_hitmarker):
			wm.enemy_hit.connect(flash_hitmarker)

func flash_hitmarker():
	if crosshair:
		var tween = create_tween()
		crosshair.modulate = Color(10, 10, 10, 1) # Pure White Glow
		tween.tween_property(crosshair, "modulate", Color(1, 1, 1, 1), 0.1)

func flash_damage():
	if damage_flash:
		var tween = create_tween()
		damage_flash.modulate.a = 1.0
		tween.tween_property(damage_flash, "modulate:a", 0.0, 0.3)

func update_health(current_health: int):
	health_bar.value = current_health

func _process(_delta):
	# Update the text every frame with the current FPS
	fps_label.text = str(Engine.get_frames_per_second())

func update_weapon_ui(weapon: WeaponData):
	if weapon == null:
		bullets.hide()
		magazines.hide()
		return
		
	if weapon.hud_hint == "SHELLS" or weapon.hud_hint == "BULLETS":
		bullets.show()
		magazines.show()
	else:
		bullets.hide()
		magazines.hide()

func update_bullets(count: int):
	# This finds the child node that actually handles the ammo display
	$Bullets.update_bullet_count(count)
	
func update_magazines(count: int):
	# This finds the child node that actually handles the ammo display
	$Mag.update_magazines_count(count)
