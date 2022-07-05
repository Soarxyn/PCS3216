import os

from rich import box
from rich.style import Style
from rich.console import RenderableType
from rich.text import Text
from rich.table import Table
from rich.layout import Layout

from textual import events
from textual.reactive import Reactive
from textual.widget import Widget

from memoryApps import memoryApps
from interface import interface

class _cmdLine(Widget):
    _instance = None
    
    cmdHeight= 3
    line = Reactive(Text("cmd> "))
    cmdText = ""
    history = list()
    i = 0
    ignoreKeys = ["ctrl+a",
                  "ctrl+r",
                  "left",
                  "right",
                  "ctrl+i",
                  ]
    
    def commands(self, cmd: iter):
        if len(cmd) < 1:
            return
        cmd[0].lower()
        # if len(cmdSplited) == 1:
        #     return
        if cmd[0] == "load":
            # chamar loader
            memoryApps().addApp(cmd[1])
            interface().refresher()
        elif cmd[0] == "assemble":
            # chamar assembler
            pass
        elif cmd[0] == "simulate":
            interface().changeMode("Simulation")
        elif cmd[0] == "home":
            interface().changeMode("Home")
    
    def on_focus(self) -> None:
        self.line = Text("cmd> ").append(self.cmdText).append("_", style=Style(blink=True))
        
    def on_blur(self) -> None:
        self.line = self.line[:-1]
        
    def on_key(self, event: events.Key):
        if self.ignoreKeys.count(event.key) == 1:
            pass
        elif event.key == "ctrl+h":
            self.cmdText = self.cmdText[:-1]
        elif event.key == "enter":
            if not self.cmdText.isspace():
                self.history.append(self.cmdText)
                self.commands(self.cmdText.split())
            self.cmdText = ""
        elif event.key == "up":
            if self.i != -len(self.history):
                self.i -= 1
                self.cmdText = self.history[self.i]
        elif event.key == "down":
            if self.i != 0:
                self.i += 1
                self.cmdText = self.history[self.i]
        else:
            self.i = 0
            self.cmdText = self.cmdText + event.key
        self.line = Text("cmd> ").append(self.cmdText).append("_", style=Style(blink=True))

    def render(self) -> RenderableType:
        height = int(os.get_terminal_size()[1]/4)
        height = min(height, 13)
        height = max(height, 3)
        height -= 3
    
        grid = Table(show_header= False,
                     expand= True,
                     box= box.HEAVY,
                     style= Style(color= "green1", bold= True))
        
        for i in range(height):
            if i >= height - len(self.history):
                grid.add_row(self.history[-height + i])
            else:
                grid.add_row("")
        grid.add_row(self.line)
        return grid

def cmdLine():
    if _cmdLine._instance is None:
        _cmdLine._instance = _cmdLine()
    return _cmdLine._instance