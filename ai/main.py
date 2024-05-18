from dataclasses import dataclass
from enum import Enum, StrEnum

from ai.transports.socket_transport import TCPTransport
from time import sleep
import json
import random
import dataclasses
import logging
logger = logging.getLogger(__name__)

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


class MovementAction(str, Enum):
    Up="Up"
    Down="Down"
    Left="Left"
    Right="Right"


@dataclass
class AgentAction:
    x: float
    y: float
    kick: bool
    movement: list[MovementAction]

    @classmethod
    def get_random(cls) -> "AgentAction":
        x = random.uniform(0,1)
        y = random.uniform(0,1)
        kick = bool(random.choice([0,1]))
        movement = random.choice(list(MovementAction))
        
        return AgentAction(x, y, kick, [movement])


def main():
    transport = TCPTransport('localhost', 21878)
    frame_generator = transport.frame_generator()
    next(frame_generator)
    while True:
        action = AgentAction.get_random()
        b = json.dumps(dataclasses.asdict(action))
        frame = frame_generator.send(b)
        print(frame)

        sleep(3)



if __name__ == "__main__":
    logging.basicConfig()
    main()

