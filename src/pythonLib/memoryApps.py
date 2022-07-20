from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree

from textual.reactive import Reactive
from textual.widget import Widget

from sisprog import parse_binary, write_many, execute, cycle, cycle

class _memoryApps(Widget):
    _instance = None
    
    apps = Reactive(Tree("Memória"))
    appsList = list()
    appsPos = list()
    
    instMemoryUnused = [[0, 2**16]] # address & unused space at such address
    dataMemoryUnused = [[0x10000, 2**16]] # address & unused space at such address
    
    def addToMemory(self, dataSize: int, instSize: int):
        instStart = -1
        dataStart = -1
        for i in range(len(self.instMemoryUnused)):
            if self.instMemoryUnused[i][1] >= instSize:
                instStart = self.instMemoryUnused[i][0]
                if self.instMemoryUnused == instSize:
                    self.instMemoryUnused.pop(i)
                else:
                    self.instMemoryUnused[i][0] += instSize
                    self.instMemoryUnused[i][1] -= instSize
                break
        for i in range(len(self.dataMemoryUnused)):
            if self.dataMemoryUnused[i][1] >= dataSize:
                dataStart = self.dataMemoryUnused[i][0]
                if self.dataMemoryUnused == dataSize:
                    self.dataMemoryUnused.pop(i)
                else:
                    self.dataMemoryUnused[i][0] += dataSize
                    self.dataMemoryUnused[i][1] -= dataSize
                break
        return dataStart, instStart
    
    def addLoader(self) -> None:
        dataSize, instSize, data, inst = parse_binary("./loader.fita")
        dataStart, instStart = self.addToMemory(dataSize, instSize)
        if dataStart != -1 and instStart != -1:
            self.appsList.append("loader")
            self.appsPos.append((dataStart, dataSize, instStart, instSize))
            write_many(
                dataStart,
                data
            )
            write_many(
                instStart,
                inst
            )
            self.apps = Tree("Memória")
            for i in range(len(self.appsList)):
                self.apps.add(self.appsList[i])
    
    def addApp(self, appName: str) -> bool:
        dataSize, instSize, data, inst = parse_binary("./root/" + appName)
        dataStart, instStart = self.addToMemory(dataSize, instSize)
        if dataStart != -1 and instStart != -1:
            self.appsList.append(appName)
            self.appsPos.append((dataStart, dataSize, instStart, instSize))
            write_many(
                0x10000, [
                    instStart,
                    dataStart - 0x10000,
                    0x30000,
                    instSize,
                    dataSize,
                    dataStart,
                    instStart,
                ]
            )
            write_many(
                0x30000,
                data + inst
            )
            execute(0x0, False)
            cycle()
            self.apps = Tree("Memória")
            for i in range(len(self.appsList)):
                self.apps.add(self.appsList[i])
            return True
        return False
    
    def removeFromMemory(self, dataStart, dataSize, instStart, instSize):
        self.dataMemoryUnused.append([dataStart, dataSize])
        self.dataMemoryUnused.sort()
        self.instMemoryUnused.append([instStart, instSize])
        self.instMemoryUnused.sort()
        
    def removeApp(self, name: str):
        index = self.appsList.index(name)
        self.appsList.remove(name)
        self.removeFromMemory(*self.appsPos[index])
        self.apps = Tree("Memória")
        for i in range(len(self.appsList)):
            self.apps.add(self.appsList[i])

    def render(self) -> RenderableType:
        return Panel(Align(self.apps),
                     title= "Aplicativos carregados",
                     border_style= Style(color= "bright_cyan"))

def memoryApps():
    if _memoryApps._instance is None:
        _memoryApps._instance = _memoryApps()
    return _memoryApps._instance