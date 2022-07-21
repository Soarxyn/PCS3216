import os

from rich import box
from rich.style import Style
from rich.console import RenderableType
from rich.text import Text
from rich.table import Table

from textual import events
from textual.reactive import Reactive
from textual.widget import Widget

from pythonLib.memoryApps import memoryApps
from pythonLib.interface import interface
from pythonLib.codePeeker import codePeeker
from pythonLib.memoryDump import memoryDump
from sisprog import assemble, link, execute, CPUState, get_state, get_print, cycle, feed_read

class _cmdLine(Widget):
    _instance = None
    
    cmdHeight= 3
    line = Reactive(Text("cmd> "))
    cmdText = ""
    cmdSuffix = ""
    history = [""]
    printedHistory = list()
    y = 0
    x = 0
    errorStyle = Style(color= "red1", bold= True)
    goodStyle = Style(color= "green1", bold= True)
    printStyle = Style(color= "cadet_blue", bold= True)
    
    validCommands = [
        "home",
        "clear",
        # "run",
        "simulate",
        "load",
        "unload",
        "delete",
        "peek",
        "assemble",
        "link",
        "assemble",
        "link",
        "step",
        "see",
    ]
    ignoreKeys = [
        "ctrl",
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
        "insert",
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
            if get_state() == CPUState.INPUT:
                feed_read(int(self.cmdText))
                self.printedHistory.append(Text(self.cmdText, style= Style(color= "medium_turquoise")))
            else:
                if (not self.cmdText.isspace()) and self.cmdText:
                    self.history.append(self.cmdText)
                    self.printedHistory.append(self.cmdText)
                    self.commands(self.cmdText.split())
            self.cmdText = ""
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
        if get_state() == CPUState.OUTPUT:
            toPrint = get_print()
            try:
                self.printExit(str(bytes(toPrint).decode('utf-8')))
            except:
                self.printExit(str(int.from_bytes(toPrint, 'little')))
        interface().refresher()
        self.line = Text("cmd> ").append(self.cmdText[:self.x]).append("_", style= Style(blink= True)).append(self.cmdText[self.x:])
        
    def printExit(self, text: str):
        self.printedHistory.append(
            Text(text, style= self.printStyle)
        )
        
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
            if memoryApps().appsList.count(args[1]) == 1:
                index = memoryApps().appsList.index(args[1])
                instStart = memoryApps().appsPos[index][2]
                execute(instStart, True)
                while True:
                    self.printSuccess(str(get_state()))
                    if get_state() == CPUState.OUTPUT:
                        self.printSuccess(str(get_state()))
                    if get_state() == CPUState.INPUT:
                        self.printSuccess(str(get_state()))
                        break
                    if get_state() == CPUState.IDLE:
                        self.printSuccess(str(get_state()))
                        break
                    cycle()
            else:
                self.printError("Arquivo não está na memória: " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdSimulate(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])  
        elif len(args) == 2:
            if memoryApps().appsList.count(args[1]) == 1:
                interface().changeMode("Simulation")
                codePeeker("Simulation").setPath(args[1][:-4] + "qck")
                codePeeker("Simulation").activeLine = codePeeker("Simulation").startLine+2
                index = memoryApps().appsList.index(args[1])
                instStart = memoryApps().appsPos[index][2]
                execute(instStart, True)
            else:
                self.printError("Arquivo não está na memória: " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdLoad(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])  
        elif len(args) == 2:
            if args[1][-4:] == "fita":
                if os.path.exists("./root/" + args[1]):
                    if memoryApps().appsList.count(args[1]) == 0:
                        memoryApps().addApp(args[1])
                        interface().refresher()
                        self.printSuccess(args[1] + " adicionado a memória")
                    else:
                        self.printError("Arquivo já carregado")
                else:
                    self.printError("Arquivo inexistente: " + args[1])
            else:
                self.printError("Só é possível fazer LOAD em arquivos '.fita'")
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
    def cmdUnload(self, args: iter):
        if len(args) == 1:
            self.printError("Faltam argumentos para " + args[0])  
        elif len(args) == 2:
            if args[1] != "loader":
                if memoryApps().appsList.count(args[1]) == 1:
                    memoryApps().removeApp(args[1])
                    interface().refresher()
                    self.printSuccess(args[1] + " removido da memória")
                else:
                    self.printError("Arquivo não está na memória: " + args[1])
            else:
                self.printError("Não é possível descarregar o loader")
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
                result = assemble("./root/" + args[1], "./root/" + args[1][:-3] + "bdc")
                if result[0]:
                    self.printSuccess("Assembled " + args[1])
                    interface().refresher()
                else:
                    self.printError(result[1])
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
                        if result[0]:
                            self.printSuccess("Assembled " + args[1] + " into " + args[3])
                            interface().refresher()
                        else:
                            self.printError(result[1])
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
                result = link(toLink, "./root/" + args[1][:-3] + "fita")
                if result[0]:
                    self.printSuccess("Linked " + str(args[1:]))
                    interface().refresher()
                else:
                    self.printError(result[1])
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
                    if result[0]:
                        self.printSuccess("Linked " + str(args[1:-2]) + " to " + args[-1])
                        interface().refresher()
                    else:
                        self.printError(result[1])
            else:
                self.printError("Posicao errada do argumento '-o'")
    
    def cmdStep(self, args: iter):
        if get_state() == CPUState.IDLE:
            self.printError("A simulação já acabou")
        elif len(args) == 1:
            cycle()
            codePeeker("Simulation").activeLine += 1
            interface().refresher()
        else:
            self.printError("Argumentos demais: " + str(args[1:]))
            
    def cmdSee(self, args: iter):
        if len(args) == 1:
            self.printError("Uso: SEE (instruction, data, stack, io)")
        elif len(args) == 2:
            if memoryDump().pages.count(args[1]) == 1:
                memoryDump().changePage(args[1])
                memoryDump().firstLine = 0
                interface().refresher()
            else:
                self.printError("Não há memória " + args[1])
        else:
            self.printError("Argumentos demais: " + str(args[2:]))
    
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
        elif cmd[0] == "step":
            self.cmdStep(cmd)
        elif cmd[0] == "see":
            self.cmdSee(cmd)
                
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