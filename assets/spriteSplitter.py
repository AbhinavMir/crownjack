from PIL import Image
import numpy as np
import os

def verify_sprite(sprite):
    """Check if sprite has any non-transparent pixels"""
    data = np.array(sprite)
    alpha = data[:, :, 3]
    return np.any(alpha > 0)

def find_sprite_boundaries(image_path):
    img = Image.open(image_path)
    img = img.convert('RGBA')
    data = np.array(img)
    
    # Get alpha channel
    alpha = data[:, :, 3]
    
    # Find rows and columns with content (more strict threshold)
    threshold = 10  # Adjust if needed
    rows_with_content = np.where(alpha.max(axis=1) > threshold)[0]
    cols_with_content = np.where(alpha.max(axis=0) > threshold)[0]
    
    # Group continuous regions
    def group_continuous(indices, max_gap=2):
        groups = []
        current_group = [indices[0]]
        
        for i in range(1, len(indices)):
            if indices[i] - indices[i-1] <= max_gap:
                current_group.append(indices[i])
            else:
                groups.append((min(current_group), max(current_group)))
                current_group = [indices[i]]
        
        groups.append((min(current_group), max(current_group)))
        return groups
    
    row_regions = group_continuous(rows_with_content)
    col_regions = group_continuous(cols_with_content)
    
    return row_regions, col_regions

def split_sprites(image_path, output_dir='card_sprites'):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
        
    img = Image.open(image_path)
    row_regions, col_regions = find_sprite_boundaries(image_path)
    
    print(f"Detected {len(row_regions)} rows and {len(col_regions)} columns of sprites")
    
    empty_sprites = []
    saved_sprites = []
    
    for row_idx, (row_start, row_end) in enumerate(row_regions):
        for col_idx, (col_start, col_end) in enumerate(col_regions):
            # Add small padding
            left = max(0, col_start - 1)
            right = min(img.width, col_end + 2)
            top = max(0, row_start - 1)
            bottom = min(img.height, row_end + 2)
            
            # Crop sprite
            sprite = img.crop((left, top, right, bottom))
            
            # Generate filename based on position
            if row_idx < 5 and col_idx < 8:
                value = ['5', '4', '3', '2', 'A'][row_idx]
                suit = ['clubs', 'spades', 'hearts', 'diamonds'][col_idx // 2]
                variant = 'outline' if col_idx % 2 else 'colored'
                filename = f"{value}_{suit}_{variant}.png"
            else:
                filename = f"special_{row_idx}_{col_idx}.png"
            
            # Verify sprite has content
            if verify_sprite(sprite):
                output_path = os.path.join(output_dir, filename)
                sprite.save(output_path, 'PNG')
                saved_sprites.append(filename)
                print(f"Saved {filename} ({right-left}x{bottom-top} pixels)")
            else:
                empty_sprites.append(filename)
                print(f"WARNING: Empty sprite detected for {filename}")
    
    print("\nSummary:")
    print(f"Successfully saved {len(saved_sprites)} sprites")
    if empty_sprites:
        print(f"Found {len(empty_sprites)} empty sprites:")
        for empty in empty_sprites:
            print(f"  - {empty}")

if __name__ == "__main__":
    split_sprites('cards_sprite_sheet.png')