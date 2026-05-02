You are an expert Godot engine developer. Your task is to implement Task 2 from the Slitherfist integration plan.

Task 2: Configure Slitherfist Logic
Modify: enemies/slitherfist/Slitherfist.gd

Step 1: Verify and adjust constants for Slitherfist (speed, meleerange, detection_range).
Step 2: Verify signal connections in _ready() for HealthComponent and AttackComponent, and ensure they are connected properly.

Context: 
- Current Slitherfist.gd:
extends CharacterBody3D
class_name Slitherfist

@export var speed: float = 4.0
@export var meleerange: float = 2.0 

@export var detection_range: float = 20.0 
@export var pain_chance: int = 50

# ... (rest of the file as previously read)

In _ready():
	health_component.damaged.connect(_on_health_component_damaged)
	health_component.died.connect(_on_health_component_died)
	
	attack_component.attack_ready.connect(_on_attack_component_attack_ready)
	attack_component.ranged_projectile_fired.connect(_on_attack_component_ranged_projectile_fired)

