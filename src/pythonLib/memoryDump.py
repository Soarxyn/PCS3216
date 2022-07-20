import os

from rich import box
from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.table import Table
from rich.text import Text

from textual.reactive import Reactive
from textual.widget import Widget

from sisprog import read_memory

class _memoryDump(Widget):
    _instance = None
    
    height = 1
    page = Reactive(0)
    firstLine = Reactive(0)
    lastLine = 1
    pages = [
        "instruction", 
        "data", 
        "stack", 
        "io"
    ]
    names = [
        "Memória de Instrução",
        "Memória de Dados",
        "Memória da Pilha",
        "Memória Periférica"
    ]
    
    def changePage(self, pageDest: str):
        self.page = self.pages.index(pageDest)
    
    def render(self) -> RenderableType:
        self.height = int(3*os.get_terminal_size()[1]/4)
        self.height = max(self.height, os.get_terminal_size()[1] - 13)
        self.height = min(self.height, os.get_terminal_size()[1] - 3)
        self.height -= 6
        self.lastLine = self.firstLine + self.height
        memTable = Table(
            box= None,
            expand= True,
            show_header= False
        )
        for i in range(self.firstLine, self.height+self.firstLine):
            memTable.add_row(f"{i+self.page*0x10000:05x}: {read_memory(i+self.page*0x10000):08x}")
        return Panel(memTable,
                     title= self.names[self.page],
                     border_style= Style(color= "bright_cyan"))

def memoryDump():
    if _memoryDump._instance is None:
        _memoryDump._instance = _memoryDump()
    return _memoryDump._instance