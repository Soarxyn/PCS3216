from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree

from textual.reactive import Reactive
from textual.widget import Widget

import os

class _folderOpen(Widget):
    _instance = None
        
    def getFiles(self) -> Tree:
        archives = os.scandir("./root")
        archList = Tree("src")
        for arch in archives:
            if arch.is_file():
                archList.add(arch.name)
        return archList
    
    archives = os.scandir("./root")
    archList = Tree("src")
    for arch in archives:
        if arch.is_file():
            archList.add(arch.name)
    archs = Reactive(archList)
    
    def updater(self):
        self.archs = self.getFiles()

    def render(self) -> RenderableType:
        return Panel(Align(self.archs),
                     title="Pasta externa",
                     border_style= Style(color= "bright_cyan"))

def folderOpen():
    if _folderOpen._instance is None:
        _folderOpen._instance = _folderOpen()
    return _folderOpen._instance