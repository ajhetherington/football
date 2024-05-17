"""
# Transports
Methods of IPC for environment-backend communication
"""

from abc import ABC
from typing import Generator

class Transport(ABC):
    """
    Transport interface
    """

    def read_frame(self) -> Generator:
        """
        Write actions back to the football sim
        """
        raise NotImplementedError(f"class {self.__class__.__name__} does not have method read_frame")

    def write_frame(self) -> Generator:
        """
        Read the next state from the football sim
        """
        raise NotImplementedError(f"class {self.__class__.__name__} does not have method write_frame")
