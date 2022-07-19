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
    y = 0
    x = 0
    errorStyle = Style(color= "red1", bold= True)
    goodStyle = Style(color= "green1", bold= True)
    
    validCommands = [
        "home",
        "clear",
        "run",
        "simulate",
        "load",
        "unload",
        "delete",
        "peek",
        "assemble",
        "link",
        "assemble",
        "link",
    ]
    ignoreKeys = [
        "ctrl+q",
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
        "ctrl+delete",
    ]
        
    def on_key(self, event: events.Key):
        if self.ignoreKeys.count(event.key) == 1:
            pass
        elif event.key == "ctrl+h":
            self.cmdText = self.cmdText[:self.x-1] + self.cmdText[self.x:]
            self.x -= 1
        elif event.key == "delete":
            self.cmdText = self.cmdText[:self.x] + self.cmdText[self.x+1:]
        elif event.key == "ctrl+i":
            elements = [elem[:self.x] for elem in self.validCommands]
            if elements.count(self.cmdText[:self.x]) == 1:
                cmdNumber = elements.index(self.cmdText[:self.x])
                self.cmdText = self.cmdText[:self.x] + self.validCommands[cmdNumber][self.x:] + self.cmdText[self.x:]
                self.x += len(self.validCommands[cmdNumber][self.x:])
        elif event.key == "enter":
            self.cmdText = self.cmdText + self.cmdRight
            if (not self.cmdText.isspace()) and self.cmdText:
                self.history.append(self.cmdText)
                self.printedHistory.append(self.cmdText)
                self.commands(self.cmdText.split())
            self.cmdText = ""
            self.cmdRight = ""
            self.y = 0
            self.x = 0
        elif event.key == "up":
            if self.y != -len(self.history)+1:
                self.y -= 1
                self.cmdText = self.history[self.y]
                self.x = len(self.cmdText)
        elif event.key == "down":
            if self.y != 0:
                self.y += 1
                self.cmdText = self.history[self.y]
                self.x = len(self.cmdText)
        elif event.key == "left":
            if self.x > 0:
                self.x -= 1
        elif event.key == "right":
            if self.x < len(self.cmdText):
                self.x += 1
        else:
            self.cmdText = self.cmdText[:self.x] + event.key + self.cmdText[self.x:]
            self.x += 1
        self.line = Text("cmd> ").append(self.cmdText[:self.x]).append("_", style= Style(blink= True)).append(self.cmdText[self.x:])
        
    def printError(self, text: str):
        self.printedHistory.append(
            Text(text, style= self.errorStyle)
        )
        
    def printSuccess(self, text: str):
        self.printedHistory.append(
            Text(text, style= self.goodStyle)
        )
        
    def cmdHome(self, args: iter):
        if len(args) == 1:
            interface().changeMode("Home")
        else:
            self.printError("Argumentos demais: " + str(args[1:]))
    
    def cmdClear(self, args: iter):
        if len(args) == 1:
            self.printedHistory = list()
        else:
            self.printError("Argumentos demais: " + str(args[1:]))
            
    def cmdRun(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])  
        elif len(args) == 2:
            if memoryApps().appList.count(args[1]) == 1:
                pass # rodar o codigo
            else:
                self.printError("Arquivo não está na memória: " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdSimulate(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])  
        elif len(args) == 2:
            if memoryApps().appList.count(args[1]) == 1:
                interface().changeMode("Simulation")
                codePeeker("Simulation").setPath(args[1])
                # rodar codigo linha a linha
            else:
                self.printError("Arquivo não está na memória: " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdLoad(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])  
        elif len(args) == 2:
            if os.path.exists("./root/" + args[1]):
                if memoryApps().appList.count(args[1]) == 0:
                    memoryApps().addApp(args[1])
                    interface().refresher()
                    self.printSuccess(args[1] + " adicionado na memória")
                else:
                    self.printError("Arquivo já carregado")
            else:
                self.printError("Arquivo inexistente: " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdUnload(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])  
        elif len(args) == 2:
            if memoryApps().appList.count(args[1]) == 1:
                memoryApps().removeApp(args[1])
                interface().refresher()
                self.printSuccess(args[1] + " removido da memória")
            else:
                self.printError("Arquivo não está na memória: " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdDelete(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])  
        elif len(args) == 2:
            if os.path.exists("./root/" + args[1]):
                os.remove("./root/" + args[1])
                self.printSuccess("Deleted " + args[1])
                interface().refresher()
            else:
                self.printError("Arquivo inexistente: " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdPeek(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])
        elif len(args) == 2:
            if os.path.exists("./root/" + args[1]):
                codePeeker("Home").setPath(args[1])
                interface().refresher()
            else:
                self.printError("Arquivo inexistente: " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdAssemble(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])
        elif len(args) == 2:
            if os.path.exists("./root/" + args[1]):
                result = assemble("./root/" + args[1])
                if result == "Assembly successful":
                    self.printSuccess("Assembled " + args[1])
                    interface().refresher()
                else:
                    self.printError(result)
            else:
                self.printError("Arquivo inexistente: " + args[1])
        elif len(args) == 3:
            if args.count("-o") == 1:
                if args.index("-o") == 1:
                    self.printError("Faltou arquivo de entrada antes de '-o'")
                else:
                    self.printError("Faltou arquivo de saída após '-o'")
            else:
                self.printError("'-o' é necessário para nomear arquivo de saída")
        elif len(args) == 4:
            if args.count("-o") == 0:
                self.printError("Argumentos demais: " + str(args[1:]))
            else:
                if args.index("-o") == 2:
                    if os.path.exists("./root/" + args[1]):
                        result = assemble("./root/" + args[1], "./root/" + args[3])
                        if result == "Assembly successful":
                            self.printSuccess("Assembled " + args[1] + " into " + args[3])
                            interface().refresher()
                        else:
                            self.printError(result)
                    else:
                        self.printError("Arquivo inexistente: " + args[1])
                else:
                    self.printError("Posicao errada do argumento '-o'")
        else:
            self.printError("Argumentos demais: " + args[4:])
    
    def cmdLink(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])
        elif args.count("-o") == 0:
            toLink = list()
            pathError = list()
            for k in range(1, len(args)):
                if os.path.exists("./root/" + args[k]):
                    toLink.append("./root/" + args[k])
                else:
                    pathError.append(args[k])
            if len(pathError) != 0:
                self.printError("Arquivos não encontrados: " + str(pathError))
            else:
                result = link(toLink)
                if result == "Linking successful":
                    self.printSuccess("Linked " + str(args[1:]))
                    interface().refresher()
                else:
                    self.printError(result)
        else:
            if args.index("-o") == len(args) - 2:
                toLink = list()
                pathError = list()
                for k in range(1, len(args)-2):
                    if os.path.exists("./root/" + args[k]):
                        toLink.append("./root/" + args[k])
                    else:
                        pathError.append(args[k])
                if len(pathError) != 0:
                    self.printError("Arquivos não encontrados: " + str(pathError))
                else:
                    result = link(toLink, "./root/" + args[-1])
                    if result == "Linking successful":
                        self.printSuccess("Linked " + str(args[1:-2]) + " to " + args[-1])
                        interface().refresher()
                    else:
                        self.printError(result)
            else:
                self.printError("Posicao errada do argumento '-o'")
    
    def commands(self, cmd: iter):
        cmd[0] = cmd[0].lower()
        if self.validCommands.count(cmd[0]) == 0:
            self.printError("Comando inexistente")
        elif cmd[0] == "home":
            self.cmdHome(cmd)
        elif cmd[0] == "clear":
            self.cmdClear(cmd)
        elif cmd[0] == "run":
            self.cmdRun(cmd)
        elif cmd[0] == "simulate":
            self.cmdSimulate(cmd)
        elif cmd[0] == "load":
            self.cmdLoad(cmd)
        elif cmd[0] == "unload":
            self.cmdUnload(cmd)
        elif cmd[0] == "delete":
            self.cmdDelete(cmd)
        elif cmd[0] == "peek":
            self.cmdPeek(cmd)
        elif cmd[0] == "assemble":
            self.cmdAssemble(cmd)
        elif cmd[0] == "link":
            self.cmdLink(cmd)
                
    def on_focus(self):
        self.line = Text("cmd> ").append(self.cmdText).append("_", style=Style(blink=True))
        
    def on_blur(self):
        self.line = Text("cmd> ").append(self.cmdText).append("_", style=Style(blink=True))
        self.line = self.line[:-1]
        self.x = len(self.cmdText)

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