from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.text import Text
from rich.align import Align

from textual.widget import Widget

class _patinhOs(Widget):
    _instance = None
    
    pato = "                  __\n              ___( o)>\n              \ <_. )\n               `---'\n        "
        
    def render(self) -> RenderableType:
        return Panel(Align(Text(self.pato)),
                     title= "Patinho",
                     border_style= Style(color= "yellow1"))

def patinhOs():
    if _patinhOs._instance is None:
        _patinhOs._instance = _patinhOs()
    return _patinhOs._instance