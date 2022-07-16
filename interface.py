import os

from rich.style import Style
from rich.panel import Panel
from rich.layout import Layout
from rich.console import RenderableType

from textual.reactive import Reactive
from textual.widget import Widget
from textual import events

from folderOpen import folderOpen
from memoryApps import memoryApps
from codePeeker import codePeeker
from cpuVariables import cpuVariables
from memoryDump import memoryDump

class _interface(Widget):
    _instance = None
        
    actualMode = Reactive("Simulation")
    layout = Reactive(Layout())
    i = 0

    def changeMode(self, mode: str):
        self.actualMode = mode
        
    def refresher(self):
        folderOpen().updater()
        self.layout = ""
        self.layout = Layout()

    def on_mouse_scroll_down(self, position: events.MouseScrollDown):
        if self.actualMode == "Home":
            if position.x > os.get_terminal_size()[0]*2/3:
                if codePeeker("Home").lastLine < codePeeker("Home").lineCount:
                    codePeeker("Home").firstLine += 1
                    codePeeker("Home").lastLine += 1
                    interface().refresher()
        else:
            if position.x < os.get_terminal_size()[0]/3:
                if codePeeker("Simulation").lastLine < codePeeker("Simulation").lineCount:
                    codePeeker("Simulation").firstLine += 1
                    codePeeker("Simulation").lastLine += 1
                    interface().refresher()
    
    def on_mouse_scroll_up(self, position: events.MouseScrollDown):
        if self.actualMode == "Home":
            if position.x > os.get_terminal_size()[0]*2/3:
                if codePeeker("Home").firstLine > 1:
                    codePeeker("Home").firstLine -= 1
                    codePeeker("Home").lastLine -= 1
                    interface().refresher()
        else:
            if position.x < os.get_terminal_size()[0]/3:
                if codePeeker("Simulation").firstLine > 1:
                    codePeeker("Simulation").firstLine -= 1
                    codePeeker("Simulation").lastLine -= 1
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