import yaml
import os


def get_config(path: str):
    if not os.path.exists(path):
        raise FileNotFoundError(f"File {path} not found")
    
    config = yaml.safe_load(open(path, "r"))

    return config
