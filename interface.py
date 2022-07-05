from rich.style import Style
from rich.panel import Panel
from rich.layout import Layout
from rich.console import RenderableType
from rich.columns import Columns
from rich.text import Text

from textual.reactive import Reactive
from textual.widget import Widget

from folderOpen import folderOpen
from memoryApps import memoryApps

class _interface(Widget):
    _instance = None
        
    actualMode = Reactive("Home")
    layout = Reactive(Layout())
    i = 0

    def changeMode(self, mode: str):
        self.actualMode = mode
        
    def refresher(self):
        self.layout = Layout()
        
    def render(self) -> RenderableType:
        if self.actualMode == "Home":
            self.layout.split_row(
                Layout(folderOpen()),
                Layout(memoryApps())
            )
        elif self.actualMode == "Simulation":
            self.layout.split_row(
                Layout(name="quackCode"),
                Layout(name="specialData"),
                Layout(name="hexDump")
            )
        return Panel(self.layout,
                     title= self.actualMode,
                     border_style= Style(color= "yellow1"))

def interface():
    if _interface._instance is None:
        _interface._instance = _interface()
    return _interface._instance