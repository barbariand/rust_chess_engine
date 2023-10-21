from PIL import Image
import os

def crop_image(input_path, output_path):
    with Image.open(input_path) as img:
        # Get bounding box of non-transparent content
        bbox = img.getbbox()
        if bbox:
            img_cropped = img.crop(bbox)
            img_cropped.save(output_path)
        else:
            print(f"No content found in {input_path}, skipping.")

directory = './'  # Current directory. Change this if your images are in a different directory.

for filename in os.listdir(directory):
    if filename.endswith(".png"):
        file_path = os.path.join(directory, filename)
        crop_image(file_path, file_path)
        print(f"Cropped {filename}")

print("All PNGs cropped.")