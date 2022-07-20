import os

from rich.style import Style
from rich.panel import Panel
from rich.layout import Layout
from rich.console import RenderableType

from textual.reactive import Reactive
from textual.widget import Widget
from textual import events

from pythonLib.folderOpen import folderOpen
from pythonLib.memoryApps import memoryApps
from pythonLib.codePeeker import codePeeker
from pythonLib.cpuVariables import cpuVariables
from pythonLib.memoryDump import memoryDump

class _interface(Widget):
    _instance = None
        
    actualMode = Reactive("Home")
    layout = Reactive(Layout())
    i = 0

    def changeMode(self, mode: str):
        self.actualMode = mode
        
    def refresher(self):
        cpuVariables().refresh()
        memoryDump().refresh()
        folderOpen().updater()
        self.layout = ""
        self.layout = Layout()

    def on_mouse_scroll_down(self, position: events.MouseScrollDown):
        width = os.get_terminal_size()[0]
        if self.actualMode == "Home":
            if position.x > width*2/3:
                if codePeeker("Home").lastLine < codePeeker("Home").lineCount:
                    codePeeker("Home").firstLine += 1
                    codePeeker("Home").lastLine += 1
                    interface().refresher()
        else:
            if position.x < width/3:
                if codePeeker("Simulation").lastLine < codePeeker("Simulation").lineCount:
                    codePeeker("Simulation").firstLine += 1
                    codePeeker("Simulation").lastLine += 1
                    interface().refresher()
            elif position.x > width*2/3:
                if memoryDump().lastLine < 0x10000-1:
                    memoryDump().firstLine += 1
                    memoryDump().lastLine += 1
                    interface().refresher()
    
    def on_mouse_scroll_up(self, position: events.MouseScrollDown):
        width = os.get_terminal_size()[0]
        if self.actualMode == "Home":
            if position.x > width*2/3:
                if codePeeker("Home").firstLine > 1:
                    codePeeker("Home").firstLine -= 1
                    codePeeker("Home").lastLine -= 1
                    interface().refresher()
        else:
            if position.x < width/3:
                if codePeeker("Simulation").firstLine > 1:
                    codePeeker("Simulation").firstLine -= 1
                    codePeeker("Simulation").lastLine -= 1
                    interface().refresher()
            elif position.x > width*2/3:
                if memoryDump().firstLine > 1:
                    memoryDump().firstLine -= 1
                    memoryDump().lastLine -= 1
                    interface().refresher()
        
    def render(self) -> RenderableType:
        if self.actualMode == "Home":
            self.layout.split_row(
                Layout(folderOpen()),
                Layout(memoryApps()),
                Layout(codePeeker(self.actualMode))
            )
        elif self.actualMode == "Simulation":
            self.layout.split_row(
                Layout(codePeeker(self.actualMode)),
                Layout(cpuVariables()),
                Layout(memoryDump())
            )
        return Panel(self.layout,
                     title= self.actualMode,
                     border_style= Style(color= "yellow1"))

def interface():
    if _interface._instance is None:
        _interface._instance = _interface()
    return _interface._instance