extends Resource
class_name WeaponData

@export var weapon_name: String = "Weapon"
@export var slot_number: int = 1
@export var ammo_type: String = "bullets"

@export_subgroup("Sprite")
@export var sprite_frames: SpriteFrames # The animations for this specific gun
@export var sprite_offset: Vector2 = Vector2.ZERO # Lets you nudge each gun!
@export var flip_h: bool = false

@export_subgroup("Offhand Sprite")
@export var offhand_frames: SpriteFrames
@export var offhand_offset: Vector2 = Vector2.ZERO
@export var offhand_flip_h: bool = false

@export_subgroup("Actions")
@export var actions: Dictionary = {} # String (e.g. "primary") to WeaponAction
@export var hud_hint: String = "BULLETS"

@export_subgroup("Ammo")
@export var ammo_give: int = 10
