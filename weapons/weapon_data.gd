extends Resource
class_name WeaponData

@export var weapon_name: String = "Weapon"

@export_subgroup("Sprite")
@export var sprite_frames: SpriteFrames # The animations for this specific gun
@export var sprite_offset: Vector2 = Vector2.ZERO # Lets you nudge each gun!

@export_subgroup("Actions")
@export var actions: Dictionary = {} # String (e.g. "primary") to WeaponAction
@export var hud_hint: String = "BULLETS"
