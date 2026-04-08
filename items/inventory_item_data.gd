extends PickupResource
class_name InventoryItemData

@export_group("Visuals")
@export var icon: Texture2D
@export var description: String = ""

@export_group("Behavior")
@export var is_consumable: bool = false
@export var max_stack: int = 1
@export var use_sound: AudioStream

func use(player: Node):
	print("Using item: ", item_name)
	if use_sound:
		# Play sound logic here or in InventoryManager
		pass
