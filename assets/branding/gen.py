import os

def generate_content(directory):
    files = [f for f in os.listdir(directory) if os.path.isfile(os.path.join(directory, f))]
    frames = ',\n\t\t'.join([f'"{file}"' for file in files])

    content = f'''[
Id(1):IndexNode(
    name: "Zombie1_Idle",
    frames: [
        {frames}
    ],
    isloop: false,
    index: Index(0),
    )
]'''
    return content

def write_content_to_file(directory, output_filename):
    content = generate_content(directory)
    with open(output_filename, 'w') as file:
        file.write(content)

if __name__ == "__main__":
    dir_path = input("Enter the path to the directory containing the files: ")
    output_file = input("Enter the name of the output file: ")
    write_content_to_file(dir_path, output_file)