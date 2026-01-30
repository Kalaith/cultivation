import re
import json

def get_accent_color(name, category):
    name_lower = name.lower()
    category_lower = category.lower()
    
    if any(x in name_lower for x in ['fire', 'flame', 'lava', 'magma', 'heat', 'burn', 'sun', 'yang']):
        return "red"
    if any(x in name_lower for x in ['water', 'ice', 'frost', 'lake', 'river', 'sea', 'ocean', 'rain', 'yin']):
        return "blue"
    if any(x in name_lower for x in ['nature', 'forest', 'wood', 'plant', 'herb', 'garden', 'green', 'poison', 'toxic', 'venom']):
        return "green"
    if any(x in name_lower for x in ['earth', 'stone', 'rock', 'mountain', 'sand', 'defense', 'wall', 'fort']):
        return "terracotta"
    if any(x in name_lower for x in ['metal', 'sword', 'blade', 'iron', 'steel', 'gold', 'silver', 'copper']):
        return "silver"
    if any(x in name_lower for x in ['spirit', 'soul', 'ghost', 'demon', 'void', 'shadow', 'dark', 'curse']):
        return "purple"
    if any(x in name_lower for x in ['heaven', 'divine', 'god', 'holy', 'light', 'star', 'celestial', 'sky']):
        return "gold"
    if any(x in name_lower for x in ['blood', 'red', 'crimson']):
        return "crimson"
    
    # Category based fallbacks
    if 'alchemy' in category_lower or 'medical' in category_lower:
        return "green"
    if 'forge' in category_lower or 'crafting' in category_lower:
        return "orange"
    if 'martial' in category_lower or 'combat' in category_lower:
        return "red"
    if 'learning' in category_lower or 'library' in category_lower:
        return "blue"
    if 'housing' in category_lower:
        return "warm yellow"
    
    return "golden"

def main():
    with open('wuxia-buildings.md', 'r', encoding='utf-8') as f:
        content = f.read()

    lines = content.split('\n')
    buildings = []
    
    current_category = "General"
    
    # Regex to capture "## HEADER"
    header_re = re.compile(r'^##\s+(.+)$')
    # Regex to capture "1. **Name** - Description"
    # Handling variations in spacing
    building_re = re.compile(r'^\d+\.\s+\*\*(.+?)\*\*\s*-\s*(.+)$')

    for line in lines:
        line = line.strip()
        header_match = header_re.match(line)
        if header_match:
            current_category = header_match.group(1).strip()
            continue
            
        building_match = building_re.match(line)
        if building_match:
            name = building_match.group(1).strip()
            description = building_match.group(2).strip()
            
            accent = get_accent_color(name, current_category)
            
            # Construct the prompt
            # "isolated ink illustration of an ancient Chinese alchemy building, top down 3/4 view, parchment background, hand drawn brush strokes, minimal shading, black ink, single green glowing accent, game map icon, transparent background"
            
            prompt_text = f"isolated ink illustration of an ancient Chinese {name}, top down 3/4 view, parchment background, hand drawn brush strokes, minimal shading, black ink, single {accent} glowing accent, game map icon, transparent background"
            
            # Create a filename friendly id
            # specific replacements
            sanitized_name = name.lower().replace("&", "and")
            # keep only alphanumeric and underscores
            file_id = re.sub(r'[^a-z0-9_]', '_', sanitized_name.replace(" ", "_"))
            # remove duplicate underscores
            file_id = re.sub(r'_+', '_', file_id)
            
            item = {
                "id": file_id,
                "Prompt": prompt_text,
                "NegativePrompt": "photorealistic, 3d render, bright colors, complex background, text, watermark, signature",
                "Width": 512,
                "Height": 512,
                "description": description
            }
            buildings.append(item)

    output = {"image_prompts": buildings}
    
    with open('image_prompts.json', 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2)
    
    print(f"Generated {len(buildings)} prompts.")

if __name__ == "__main__":
    main()
