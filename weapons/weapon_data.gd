extends Resource
class_name WeaponData

@export var weapon_name: String = "Weapon"
@export var damage: int = 10

@export_subgroup("Sprite")
@export var sprite_frames: SpriteFrames # The animations for this specific gun
@export var sprite_offset: Vector2 = Vector2.ZERO # Lets you nudge each gun!

@export_subgroup("Audio")
@export var shoot_sound: AudioStream

@export_group("Ammo Stats")
@export var max_bullets: int = 10
@export var ammo_type: ItemData.ItemType # Links perfectly to your pickup system!

@export_group("Shotgun Settings")
@export var pellet_count: int = 1 # 1 for pistol, 8 for shotgun
@export var spread_angle: float = 0.0 # 0 for perfect accuracy, 5.0 for wide spread
