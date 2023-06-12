from abc import ABC, abstractmethod
from .state import VeilidState

class VeilidAPI(ABC):
    @abstractmethod  
    def control(self, args: list[str]) -> str:
        pass
    @abstractmethod  
    def get_state(self) -> VeilidState:
        pass
    