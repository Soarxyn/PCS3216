from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree

from textual.reactive import Reactive
from textual.widget import Widget

class _memoryApps(Widget):
    _instance = None
    
    appList = Tree("Memória") 
    
    apps = Reactive("")

    def addApp(self, name: str):
        self.appList.add(name)
        self.apps = ""
        self.apps = self.appList

    def render(self) -> RenderableType:
        renderizavel = Align.left(self.apps)
        return Panel(renderizavel,
                     title="Aplicativos carregados",
                     title_align="left",
                     border_style= Style(color= "blue"))

def memoryApps():
    if _memoryApps._instance is None:
        _memoryApps._instance = _memoryApps()
    return _memoryApps._instance