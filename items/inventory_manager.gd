extends Node
class_name InventoryManager

signal inventory_changed
signal item_activated(item: InventoryItemData)

var items: Array[InventoryItemData] = []
var counts: Dictionary = {} # item_name -> count
var active_index: int = -1

func add_item(item: InventoryItemData, amount: int = 1) -> bool:
	if item.item_name in counts:
		if counts[item.item_name] < item.max_stack:
			counts[item.item_name] = min(counts[item.item_name] + amount, item.max_stack)
			inventory_changed.emit()
			return true
		else:
			return false # Already at max stack
	else:
		items.append(item)
		counts[item.item_name] = amount
		if active_index == -1:
			active_index = 0
		inventory_changed.emit()
		return true

func remove_item(item_name: String, amount: int = 1):
	if item_name in counts:
		counts[item_name] -= amount
		if counts[item_name] <= 0:
			counts.erase(item_name)
			var to_remove = -1
			for i in range(items.size()):
				if items[i].item_name == item_name:
					to_remove = i
					break
			if to_remove != -1:
				items.remove_at(to_remove)
				if active_index >= items.size():
					active_index = items.size() - 1
		inventory_changed.emit()

func has_item(item_name: String) -> bool:
	return item_name in counts and counts[item_name] > 0

func get_active_item() -> InventoryItemData:
	if active_index >= 0 and active_index < items.size():
		return items[active_index]
	return null

func cycle_next():
	if items.size() > 1:
		active_index = (active_index + 1) % items.size()
		inventory_changed.emit()

func cycle_prev():
	if items.size() > 1:
		active_index = (active_index - 1 + items.size()) % items.size()
		inventory_changed.emit()

func use_active_item(player: Node):
	var item = get_active_item()
	if item:
		item.use(player)
		if item.is_consumable:
			remove_item(item.item_name, 1)
