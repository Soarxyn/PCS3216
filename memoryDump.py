from rich import box
from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.table import Table
from rich.text import Text

from textual.reactive import Reactive
from textual.widget import Widget

class _memoryDump(Widget):
    _instance = None
    
    memory = [
        "linha1",
        "linha2",
        "linha3",
        "linha4",
        "linha5",
        "linha6",
        "linha7",
        "...",
    ]
    
    def render(self) -> RenderableType:
        memTable = Table(
            box= None,
            expand= True,
            show_header= False,
            # show_edge= False,
            # style= Style(color= "bright_cyan", bold= True)
        )
        for _ in self.memory:
            memTable.add_row(_)
        return Panel(memTable,
                     title= "Dump da memória",
                     border_style= Style(color= "bright_cyan"))

def memoryDump():
    if _memoryDump._instance is None:
        _memoryDump._instance = _memoryDump()
    return _memoryDump._instance