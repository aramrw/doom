extends Area3D

@onready var animation: AnimationPlayer = $"../AnimationPlayer"

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	animation.play("RESET")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_body_entered(body: Node3D) -> void:
	print(body.name, " touched the rift!") # Let's see EXACTLY what touched it
	
	if body.is_in_group("Player") or body.get_parent().is_in_group("Player"):
		# Check if the animation is already running so we don't restart it
		if animation.current_animation != "open":
			animation.play("open")


func _on_body_exited(body: Node3D) -> void:
	if animation.current_animation != "close":
		animation.play("close")
