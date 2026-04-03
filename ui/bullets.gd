extends TextureRect

@onready var bullets_label: Label = $BulletsLabel

func _ready():
	pass

func update_bullet_count(new_count: int):
	bullets_label.text = str(new_count)
