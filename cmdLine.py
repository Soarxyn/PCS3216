from rich import box
from rich.align import Align
from rich.box import DOUBLE
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree
from rich.text import Text
from rich.table import Table

from textual import events
from textual.reactive import Reactive
from textual.widget import Widget

from memoryApp import memoryApps

class cmdLine(Widget):
    title = ""
    line = Reactive("cmd> ")
    cmdText = ""
    history = list()
    i = 0
    
    def __init__(self, title: str):
        super().__init__(title)
        self.title = title
        
    def commands(self, cmd: str):
        if cmd == "":
            return
        cmdSplited = cmd.split()
        cmdSplited[0].lower()
        if len(cmdSplited) == 1:
            return
        if cmdSplited[0] == "load":
            memoryApps().addApp(cmdSplited[1])
        elif cmdSplited[0] == "assemble":
            # chamar assembler
            pass
        elif cmdSplited[0] == "simulate":
            pass
    
    def on_focus(self) -> None:
        self.line += "_"
        
    def on_blur(self) -> None:
        self.line = self.line[:-1]
        
    def on_key(self, event: events.Key):
        if event.key == "ctrl+h":
            self.cmdText = self.cmdText[:-1]
        elif event.key == "ctrl+a" or event.key == "left" or event.key == "right":
            pass
        elif event.key == "enter":
            self.history.append(self.cmdText)
            self.commands(self.cmdText)
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
        printedHistory = list()
        cmdInput = self.line
        # return Panel(renderizavel,
        #              title=self.title,
        #              title_align="left",
        #              border_style= Style(color= "green"),
        #              box= DOUBLE)
        
        grid = Table(show_header= False,
                     expand= True,
                     box= box.HEAVY,
                     style= Style(color= "green"))
        
        for i in range(4):
            if i >= 4-len(self.history):
                grid.add_row(self.history[-4+i])
            else:
                grid.add_row("")
        grid.add_row(cmdInput)
        return grid
