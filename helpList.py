from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree

from textual.widget import Widget

class helpList(Widget):
    
    helpContents = [
        "[b]ASSEMBLE [i]arquivo[/i] \[-o saida][/]",
        "[b]LINK [i]arquivos[/i] \[-o saida][/]",
        "[b]LOAD [i]arquivo[/]",
        "[b]UNLOAD [i]arquivo[/]",
        "[b]HOME[/]",
        "[b]SIMULATE [i]arquivo[/]",
        "[s]RUN[/]",
        "[b]PEEK [i]arquivo[/]",
        "[b]DELETE [i]arquivo[/]",
        "[s]CLEAR[/]",
    ]
    
    helpBar = Tree("Comandos")
    
    for _ in helpContents:
        helpBar.add(_)
    
    def render(self) -> RenderableType:
        return Panel(Align(self.helpBar),
                     title= "Comandos",
                     border_style= Style(color= "bright_magenta"))
