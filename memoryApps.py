from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree

from textual.reactive import Reactive
from textual.widget import Widget

class _memoryApps(Widget):
    _instance = None
    
    apps = Reactive(Tree("Memória"))
    appList = list() 

    def addApp(self, name: str):
        self.appList.append(name)
        self.apps = Tree("Memória")
        for i in range(len(self.appList)):
            self.apps.add(self.appList[i])
        
    def removeApp(self, name: str):
        self.appList.remove(name)
        self.apps = Tree("Memória")
        for i in range(len(self.appList)):
            self.apps.add(self.appList[i])

    def render(self) -> RenderableType:
        return Panel(Align(self.apps),
                     title= "Aplicativos carregados",
                     border_style= Style(color= "bright_cyan"))

def memoryApps():
    if _memoryApps._instance is None:
        _memoryApps._instance = _memoryApps()
    return _memoryApps._instance