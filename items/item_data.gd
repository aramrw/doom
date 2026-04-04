extends Resource
class_name ItemData

enum ItemType { 
	HLTH_MEDKIT, 
	AMMO_MAGAZINE, 
	AMMO_SHELL, 
	AMMO_ROCKET 
}

@export var item_name: String = "Item"
@export var sprite_frames: SpriteFrames  
@export var type: ItemType = ItemType.AMMO_MAGAZINE
@export var amount: int = 1
