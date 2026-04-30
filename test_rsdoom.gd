extends Node

func _ready():
    var imp = Realm667Importer.new()
    imp.pk3_path = "enemies/wicked.zip"
    imp.import_mode = 2 # Enemy
    imp.trigger_import = true
    add_child(imp)
    print("Triggered import")
