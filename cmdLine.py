import os

from rich import box
from rich.style import Style
from rich.console import RenderableType
from rich.text import Text
from rich.table import Table

from textual import events
from textual.reactive import Reactive
from textual.widget import Widget

from memoryApps import memoryApps
from interface import interface
from codePeeker import codePeeker
from sisprog import assemble, link

class _cmdLine(Widget):
    _instance = None
    
    cmdHeight= 3
    line = Reactive(Text("cmd> "))
    cmdText = ""
    cmdRight = ""
    history = [""]
    printedHistory = list()
    x = 0
    y = 0
    errorStyle = Style(color= "red1", bold= True)
    goodStyle = Style(color= "green1", bold= True)
    validCommands = ["home",
                     "run",
                     "simulate",
                     "load",
                     "unload",
                     "assemble",
                     "link",
                     "delete",
                     "clear",
                     "peek",
                     ]
    ignoreKeys = ["ctrl+q",
                  "ctrl+w",
                  "ctrl+e",
                  "ctrl+r",
                  "ctrl+t",
                  "ctrl+y",
                  "ctrl+u",
                  "ctrl+o",
                  "ctrl+p",
                  "ctrl+a",
                  "ctrl+s",
                  "ctrl+d",
                  "ctrl+f",
                  "ctrl+g",
                  "ctrl+j",
                  "ctrl+k",
                  "ctrl+l",
                  "ctrl+ç",
                  "ctrl+z",
                  "ctrl+x",
                  "ctrl+c",
                  "ctrl+v",
                  "ctrl+b",
                  "ctrl+n",
                  "ctrl+m",
                  "ctrl+@",
                  ]
        
    def on_key(self, event: events.Key):
        if self.ignoreKeys.count(event.key) == 1:
            pass
        elif event.key == "ctrl+h":
            self.cmdText = self.cmdText[:self.y-1] + self.cmdText[self.y:]
            self.y -= 1
        elif event.key == "delete":
            self.cmdText = self.cmdText[:self.y] + self.cmdText[self.y+1:]
        elif event.key == "ctrl+i":
            elements = [elem[:self.y] for elem in self.validCommands]
            if elements.count(self.cmdText[:self.y]) == 1:
                cmdNumber = elements.index(self.cmdText[:self.y])
                self.cmdText = self.cmdText[:self.y] + self.validCommands[cmdNumber][self.y:] + self.cmdText[self.y:]
                self.y += len(self.validCommands[cmdNumber][self.y:])
        elif event.key == "enter":
            self.cmdText = self.cmdText + self.cmdRight
            if (not self.cmdText.isspace()) and self.cmdText:
                self.history.append(self.cmdText)
                self.printedHistory.append(self.cmdText)
                self.commands(self.cmdText.split())
            self.cmdText = ""
            self.cmdRight = ""
            self.x = 0
            self.y = 0
        elif event.key == "up":
            if self.x != -len(self.history)+1:
                self.x -= 1
                self.cmdText = self.history[self.x]
                self.y = len(self.cmdText)
        elif event.key == "down":
            if self.x != 0:
                self.x += 1
                self.cmdText = self.history[self.x]
                self.y = len(self.cmdText)
        elif event.key == "left":
            if self.y != 0:
                self.y -= 1
        elif event.key == "right":
            if self.y != len(self.cmdText):
                self.y += 1
        else:
            self.cmdText = self.cmdText[:self.y] + event.key + self.cmdText[self.y:]
            self.y += 1
        self.line = Text("cmd> ").append(self.cmdText[:self.y]).append("_", style=Style(blink=True)).append(self.cmdText[self.y:])
    
    def commands(self, cmd: iter):
        cmd[0] = cmd[0].lower()
        if self.validCommands.count(cmd[0]) == 0:
            self.printedHistory.append(
                Text("Comando inexistente", style= self.errorStyle)
            )
        elif len(cmd) == 1:
            if cmd[0] == "home":
                interface().changeMode("Home")
            elif cmd[0] == "run":
                # comecar a rodar o arquivo carregado
                pass
            else:
                self.printedHistory.append(
                    Text("Faltam argumentos para " + cmd[0], style= self.errorStyle)
                )
        else:
            if cmd[0] == "load":
                if len(cmd) > 2:
                    self.printedHistory.append(
                        Text("Argumentos demais: " + str(cmd[2:]), style= self.errorStyle)
                    )
                else:
                    memoryApps().addApp(cmd[1])
                    interface().refresher()
            elif cmd[0] == "delete":
                if len(cmd) > 2:
                    self.printedHistory.append(
                        Text("Argumentos demais: " + str(cmd[2:]), style= self.errorStyle)
                    )
                else:
                    os.remove("./root/" + cmd[1])
                    self.printedHistory.append(
                        Text("Deleted " + str(cmd[1]), style= self.goodStyle)
                    )
                    interface().refresher()
            elif cmd[0] == "assemble":
                if cmd.count("-o") == 0:
                    if len(cmd) > 2:
                        self.printedHistory.append(
                            Text("Argumentos demais: " + str(cmd[2:]), style= self.errorStyle)
                        )
                    else:
                        result = assemble("./root/" + cmd[1])
                        if result == "Assembly successful":
                            self.printedHistory.append(
                                Text("Assembled " + cmd[1], style= self.goodStyle)
                            )
                            interface().refresher()
                        else:
                            self.printedHistory.append(
                                Text(result, style= self.errorStyle)
                            )
                else:
                    if len(cmd) > 4:
                        self.printedHistory.append(
                            Text("Argumentos demais: " + str(cmd[4:]), style= self.errorStyle)
                        )
                    else:
                        result = assemble("./root/" + cmd[1], "./root/" + cmd[3])
                        if result == "Assembly successful":
                            self.printedHistory.append(
                                Text("Assembled " + cmd[1] + " into " + cmd[3], style= self.goodStyle)
                            )
                            interface().refresher()
                        else:
                            self.printedHistory.append(
                                Text(result, style= self.errorStyle)
                            )
            elif cmd[0] == "link":
                if cmd.count("-o") == 0:
                    toLink = list()
                    for k in range(1, len(cmd)):
                        toLink.append("./root/" + cmd[k])
                    result = link(toLink)
                    if result == "Linking successful":
                        self.printedHistory.append(
                            Text("Linked " + str(cmd[1:]), style= self.goodStyle)
                        )
                    else:
                        self.printedHistory.append(
                            Text(result, style= self.errorStyle)
                        )
                else:
                    toLink = list()
                    for k in range(1, len(cmd)-2):
                        toLink.append("./root/" + cmd[k])
                    result = link(toLink, cmd[-1])
                    if result == "Linking successful":
                        self.printedHistory.append(
                            Text("Linked " + str(cmd[1:-2]) + " into " + cmd[-1], style= self.goodStyle)
                        )
                    else:
                        self.printedHistory.append(
                            Text(result, style= self.errorStyle)
                        )
            elif cmd[0] == "simulate":
                interface().changeMode("Simulation")
            elif cmd[0] == "peek":
                if len(cmd) > 2:
                    self.printedHistory.append(
                        Text("Argumentos demais: " + str(cmd[2:]), style= self.errorStyle)
                    )
                else:
                    if os.path.exists("./root/" + cmd[1]):
                        codePeeker().setPath(cmd[1])
                        interface().refresher()
            elif cmd[0] == "unload":
                if len(cmd) > 2:
                    self.printedHistory.append(
                        Text("Argumentos demais: " + str(cmd[2:]), style= self.errorStyle)
                    )
                else:
                    memoryApps().removeApp(cmd[1])
                    interface().refresher()
                
    
    def on_focus(self) -> None:
        self.line = Text("cmd> ").append(self.cmdText).append("_", style=Style(blink=True))
        
    def on_blur(self) -> None:
        self.line = self.line[:-1]

    def render(self) -> RenderableType:
        height = int(os.get_terminal_size()[1]/4)
        height = min(height, 13)
        height = max(height, 3)
        height -= 3
    
        grid = Table(show_header= False,
                     expand= True,
                     box= box.HEAVY,
                     style= Style(color= "blue1", bold= True))
        
        for x in range(height):
            if x >= height - len(self.printedHistory):
                grid.add_row(self.printedHistory[-height + x])
            else:
                grid.add_row("")
        grid.add_row(self.line)
        return grid

def cmdLine():
    if _cmdLine._instance is None:
        _cmdLine._instance = _cmdLine()
    return _cmdLine._instance