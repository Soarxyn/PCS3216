from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree

from textual.widget import Widget

class helpList(Widget):
    
    helpContents = ["[b]ASSEMBLE [i]arquivo[/]",
                    "[b]LOAD [i]arquivo[/]",
                    "[b]ALGO [i]arquivo[/]",
                    ]
    
    helpBar = Tree("Comandos")
    
    for _ in helpContents:
        helpBar.add(_)
    
    def render(self) -> RenderableType:
        return Panel(Align.left(self.helpBar),
                     title= "barra",
                     border_style= Style(color= "red"))
