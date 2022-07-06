import os
import time

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
from sisprog import assemble, link

class _cmdLine(Widget):
    _instance = None
    
    cmdHeight= 3
    line = Reactive(Text("cmd> "))
    cmdText = ""
    history = [""]
    printedHistory = list()
    i = 0
    errorStyle = Style(color= "red1", bold= True)
    goodStyle = Style(color= "green1", bold= True)
    validCommands = ["home",
                     "run",
                     "simulate",
                     "load",
                     "assemble",
                     "link",
                     ]
    ignoreKeys = ["ctrl+a",
                  "ctrl+r",
                  "left",
                  "right",
                  "ctrl+i",
                  ]
    
    def commands(self, cmd: iter):
        cmd[0] = cmd[0].lower()
        if self.validCommands.count(cmd[0]) == 0:
            self.printedHistory.append(
                Text("Comando inexistente", style= self.errorStyle)
            )
            return
        if len(cmd) == 1:
            if cmd[0] == "home":
                interface().changeMode("Home")
            elif cmd[0] == "run":
                # comecar a rodar o arquivo carregado
                pass
            else:
                self.printedHistory.append(
                    Text("Faltam argumentos para " + cmd[0], style= self.errorStyle)
                )
                return
        if cmd[0] == "load":
            # chamar loader
            memoryApps().addApp(cmd[1])
            interface().refresher()
        elif cmd[0] == "assemble":
            if cmd.count("-o") == 0:
                if len(cmd) > 2:
                    self.printedHistory.append(
                        Text("Argumentos demais: " + str(cmd[2:]), style= self.errorStyle)
                    )
                else:
                    result = assemble(cmd[1])
                    if result == "Assembly successful":
                        self.printedHistory.append(
                            Text("Assembled " + cmd[1], style= self.goodStyle)
                        )
                    else:
                        self.printedHistory.append(
                            Text(result, style= errorStyle)
                        )
            else:
                if len(cmd) > 4:
                    self.printedHistory.append(
                        Text("Argumentos demais: " + str(cmd[4:]), style= self.errorStyle)
                    )
                else:
                    result = assemble(cmd[1], cmd[3])
                    if result == "Assembly successful":
                        self.printedHistory.append(
                            Text("Assembled " + cmd[1] + " into " + cmd[3], style= self.goodStyle)
                        )
                    else:
                        self.printedHistory.append(
                            Text(result, style= errorStyle)
                        )
        elif cmd[0] == "link":
            if cmd.count("-o") == 0:
                result = link(cmd[1:])
                if result == "Linking successful":
                    self.printedHistory.append(
                        Text("Linked " + str(cmd[1:]), style= self.goodStyle)
                    )
                else:
                    self.printedHistory.append(
                        Text(result, style= errorStyle)
                    )
            else:
                result = link(cmd[1:-2], cmd[-1])
                if result == "Linking successful":
                    self.printedHistory.append(
                        Text("Linked " + str(cmd[1:-2]) + " into " + cmd[-1], style= self.goodStyle)
                    )
                else:
                    self.printedHistory.append(
                        Text(result, style= errorStyle)
                    )
        elif cmd[0] == "simulate":
            interface().changeMode("Simulation")
    
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
            if (not self.cmdText.isspace()) and self.cmdText:
                self.history.append(self.cmdText)
                self.printedHistory.append(self.cmdText)
                self.commands(self.cmdText.split())
            self.cmdText = ""
        elif event.key == "up":
            if self.i != -len(self.history)+1:
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
                     style= Style(color= "blue1", bold= True))
        
        for i in range(height):
            if i >= height - len(self.printedHistory):
                grid.add_row(self.printedHistory[-height + i])
            else:
                grid.add_row("")
        grid.add_row(self.line)
        return grid

def cmdLine():
    if _cmdLine._instance is None:
        _cmdLine._instance = _cmdLine()
    return _cmdLine._instance