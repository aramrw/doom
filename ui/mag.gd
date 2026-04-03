extends AnimatedSprite2D

@onready var mag_label: Label = $MagLabel

func _ready():
	pass

func update_magazines_count(new_count: int):
	mag_label.text = str(new_count)
