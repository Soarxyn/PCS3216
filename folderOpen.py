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
        
    archs = Reactive(Tree("src"))

    def updater(self):
        archives = os.scandir()
        archList = Tree("src")
        for arch in archives:
            if arch.is_file():
                archList.add(arch.name)
        
        self.archs = ""
        self.archs = archList

    def render(self) -> RenderableType:
        self.updater()
        return Panel(Align.left(self.archs),
                     title="Pasta aberta",
                     title_align="left",
                     border_style= Style(color= "blue"))

def folderOpen():
    if _folderOpen._instance is None:
        _folderOpen._instance = _folderOpen()
    return _folderOpen._instance