extends PickupResource
class_name ItemData

enum ItemType { 
	HLTH_MEDKIT, 
	AMMO_MAGAZINE, 
	AMMO_SHELL, 
	AMMO_ROCKET 
}

@export var type: ItemType = ItemType.AMMO_MAGAZINE
@export var amount: int = 1
