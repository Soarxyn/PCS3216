from rich.align import Align
from rich.style import Style
from rich.panel import Panel
from rich.console import RenderableType
from rich.tree import Tree

from textual.widget import Widget

import os

class _folderOpen(Widget):
    _instance = None
    
    apps = Tree("src")
    
    archives = os.scandir()
    for arch in archives:
        if arch.is_file():
            apps.add(arch.name)

    def render(self) -> RenderableType:
        renderizavel = Align.left(self.apps)
        return Panel(renderizavel,
                     title="Pasta aberta",
                     title_align="left",
                     border_style= Style(color= "blue"))

def folderOpen():
    if _folderOpen._instance is None:
        _folderOpen._instance = _folderOpen()
    return _folderOpen._instance