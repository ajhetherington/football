from dataclasses import dataclass
from enum import Enum

from ai.transports.socket_transport import TCPTransport
from time import sleep
import logging

class Team(Enum):
    Home = 1
    Away = 2

@dataclass
class Position:
    x: float
    y: float

@dataclass
class Player:
    """Class for keeping track of an item in inventory."""
    name: str
    team: Team

    def isSameTeam(self, player: "Player"):
        """Returns whether the two players are in the same team"""
        return player.team == self.team
    
    

def main():
    transport = TCPTransport('localhost', 6969)
    a = transport.frame_generator()
    next(a)
    while True:
        frame = a.send("state")
        print(frame)
        # do stuff
        sleep(3)



if __name__ == "__main__":
    logging.basicConfig()
    main()

