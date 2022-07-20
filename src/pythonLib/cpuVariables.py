from rich import box
from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.table import Table
from rich.text import Text

from textual.reactive import Reactive
from textual.widget import Widget
from sisprog import get_acc, get_sp, get_pc, get_la, get_p, get_z, get_n, get_c, get_v, get_state

class _cpuVariables(Widget):
    _instance = None
    
    varList = [
        Text("Acumulador", justify= "center"),
        Text("Stack Pointer", justify= "center"),
        Text("Contador de Programa", justify= "center"),
        Text("Endereço de Retorno", justify= "center"),
        Text("Flags (P Z N C V)", justify= "center"),
        Text("Estado da CPU", justify= "center"),
    ]
    
    def render(self) -> RenderableType:
        variables = [
            str(get_acc()) + f" (0x{get_acc():08x})",
            f"0x{get_sp():08x}",
            f"0x{get_pc():08x}",
            f"0x{get_la():08x}",
            str(int(get_p())) + " " + str(int(get_z())) + " " + str(int(get_n())) + " " + str(int(get_c())) + " " + str(int(get_v())),
            str(get_state())[9:]
        ]
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
            varTable.add_row(Align.center(variables[i]), end_section= True)
        return Panel(varTable,
                     title= "Variáveis da CPU",
                     border_style= Style(color= "bright_cyan"))

def cpuVariables():
    if _cpuVariables._instance is None:
        _cpuVariables._instance = _cpuVariables()
    return _cpuVariables._instance