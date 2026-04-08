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
	
	# Hide weapons layer (CanvasLayer doesn't inherit visibility)
	var player = get_parent()
	if player and player.has_node("WeaponManager"):
		var wm = player.get_node("WeaponManager")
		if wm.has_node("WeaponLayer"):
			wm.get_node("WeaponLayer").hide()

func deactivate():
	is_active = false
	current_item = null
	item_layer.hide()
	
	# Show weapons layer
	var player = get_parent()
	if player and player.has_node("WeaponManager"):
		var wm = player.get_node("WeaponManager")
		if wm.has_node("WeaponLayer"):
			wm.get_node("WeaponLayer").show()

func update_item(item: InventoryItemData):
	if is_active:
		if item:
			current_item = item
			item_sprite.texture = item.icon
		else:
			deactivate()
