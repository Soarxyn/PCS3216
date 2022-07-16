from rich import box
from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.table import Table
from rich.text import Text

from textual.reactive import Reactive
from textual.widget import Widget

class _cpuVariables(Widget):
    _instance = None
    
    varList = [
        Text("Acumulador", justify= "center"),
        Text("Stack Pointer", justify= "center"),
        Text("Return Address", justify= "center"),
        Text("Contador de Programa", justify= "center"),
        Text("Flags (Z N C V)", justify= "center"),
    ]
    
    variables = Reactive("")
    
    variables = [
        "0x12345678",
        "0xFFFFFFFF",
        "0x00000000",
        "0x00000000",
        "0 0 0 0"
    ]
    
    def render(self) -> RenderableType:
        varTable = Table(
            box= box.HEAVY,
            expand= True,
            show_header= False,
            show_edge= False,
            style= Style(color= "bright_cyan", bold= True)
        )
        varTable.add_row()
        for i in range(len(self.varList)):
            varTable.add_row(self.varList[i])
            varTable.add_row(Align.center(self.variables[i]), end_section= True)
        return Panel(varTable,
                     title= "Variáveis da CPU",
                     border_style= Style(color= "bright_cyan"))

def cpuVariables():
    if _cpuVariables._instance is None:
        _cpuVariables._instance = _cpuVariables()
    return _cpuVariables._instance