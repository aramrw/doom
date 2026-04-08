extends Node3D
class_name ItemManager

@onready var item_sprite = $ItemLayer/ItemSprite
@onready var item_layer = $ItemLayer

var current_item: InventoryItemData = null
var is_active: bool = false

func _ready():
	# Start hidden
	deactivate()

func activate(item: InventoryItemData):
	if not item:
		deactivate()
		return
		
	current_item = item
	item_sprite.texture = item.icon
	item_layer.show()
	is_active = true
	
	# Tell player to hide weapons
	var player = get_parent()
	if player and player.has_node("WeaponManager"):
		player.get_node("WeaponManager").hide()

func deactivate():
	is_active = false
	item_layer.hide()
	
	# Tell player to show weapons if needed
	var player = get_parent()
	if player and player.has_node("WeaponManager"):
		player.get_node("WeaponManager").show()

func update_item(item: InventoryItemData):
	if is_active:
		if item:
			current_item = item
			item_sprite.texture = item.icon
		else:
			deactivate()
